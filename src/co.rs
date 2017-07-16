use enums::Enum;


pub trait CoSuspend {
    type Yield;
    type Output;
    type Continuation: Coroutine;
    type Suspension: CoSuspend;
}

impl CoSuspend for ! {
    type Yield = !;
    type Output = !;
    type Continuation = !;
    type Suspension = !;
}

impl Coroutine for ! {
    type Input = !;
    type Output = !;
    type Suspend = !;

    fn send(self, _: Self::Input) -> CoResult<Self::Suspend, Self::Output> {
        unreachable!()
    }
}

impl<Enumeration, Y, C> CoSuspend for Enumeration
    where C: Coroutine,
          Enumeration: Enum<Variant = (Y, C)>,
          Enumeration::Next: CoSuspend
{
    type Yield = Y;
    type Output = C::Output;
    type Continuation = C;
    type Suspension = Enumeration::Next;
}

pub enum CoResult<Suspend, Return>
    where Suspend: CoSuspend
{
    Suspend(Suspend),
    Return(Return),
}

pub trait Coroutine: Sized {
    type Input;
    type Output;
    type Suspend: CoSuspend;

    fn send(self, i: Self::Input) -> CoResult<Self::Suspend, Self::Output>;
}


#[cfg(test)]
mod tests {

    #![macro_use]

    use super::{Coroutine, CoResult};
    use enums::Match::*;
    use std::fmt;

    struct TooSmall;

    impl fmt::Display for TooSmall {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Too small!")
        }
    }

    struct TooBig;

    impl fmt::Display for TooBig {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Too big!")
        }
    }

    struct Correct;

    impl fmt::Display for Correct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "You guessed it!")
        }
    }

    struct Guess(i64);

    impl Coroutine for Guess {
        type Input = i64;
        type Output = Correct;

        type Suspend = enums![(TooSmall, Guess), (TooBig, Guess)];

        fn send(self, i: Self::Input) -> CoResult<Self::Suspend, Self::Output> {
            if self.0 == i {
                CoResult::Return(Correct {})
            } else if i < self.0 {
                CoResult::Suspend(Variant((TooSmall {}, self)))
            } else {
                CoResult::Suspend(Next(Variant((TooBig {}, self))))
            }
        }
    }

    struct Game;

    impl Coroutine for Game {
        type Input = i64;
        type Output = !;

        type Suspend = enums![((), Guess)];

        fn send(self, secret: Self::Input) -> CoResult<Self::Suspend, Self::Output> {
            CoResult::Suspend(Variant(((), Guess(secret))))
        }
    }

    struct Quit;

    impl fmt::Display for Quit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "I quit!")
        }
    }

    struct Maverick;

    impl Coroutine for Maverick {
        type Input = (i64, i64);
        type Output = Quit;

        type Suspend = enums![(i64, Strategist)];

        fn send(self, range: Self::Input) -> CoResult<Self::Suspend, Self::Output> {
            if range.0 > range.1 {
                CoResult::Return(Quit {})
            } else {
                let guess = (range.0 + range.1) / 2;
                CoResult::Suspend(Variant((guess, Strategist(range.0, guess, range.1))))
            }
        }
    }

    struct Strategist(i64, i64, i64);

    impl Coroutine for Strategist {
        type Input = enums![TooSmall, TooBig];
        type Output = Quit;

        type Suspend = enums![(i64, Strategist)];

        fn send(self, result: Self::Input) -> CoResult<Self::Suspend, Self::Output> {
            let mav = Maverick {};
            let range = match result {
                Variant(TooSmall) => (self.1 + 1, self.2),
                Next(Variant(TooBig)) => (self.0, self.1 - 1),
            };
            mav.send(range)
        }
    }

    /*
    fn play_guess(secret: i64) {
        guess = yield;
        loop {
            guess = if guess == secret {
                return Correct;
            } else if guess < secret {
                yield TooSmall;
            } else {
                yield TooBig;
            }
        }
    }

    fn player(lo: i64, hi: i64) {
        while lo >= hi {
            let med = (lo + hi) / 2;
            let res = yield med;
            match res {
                TooBig => hi = med - 1,
                TooSmall => lo = med + 1
            }
        }
        return Quit;
    }
    */

    #[test]
    fn guess() {
        let game = Game {};

        let game = match game.send(16) {
            CoResult::Suspend(Variant(((), game))) => game,
        };

        let suspend = match game.send(10) {
            CoResult::Suspend(suspend) => suspend,
            _ => panic!("We didn't guess the number, so the coroutine should not return yet!"),
        };

        let game = match suspend {
            Variant((msg, guess_again)) => {
                assert_eq!(msg.to_string(), "Too small!");
                guess_again
            }
            _ => panic!("The number was clearly too small! Why do you not see this?"),
        };

        let suspend = match game.send(19) {
            CoResult::Suspend(suspend) => suspend,
            _ => panic!("We didn't guess the number, so the coroutine should not return yet!"),
        };

        let game = match suspend {
            Next(Variant((msg, guess_again))) => {
                assert_eq!(msg.to_string(), "Too big!");
                guess_again
            }
            _ => panic!("The number was clearly too big! Why do you not see this?"),
        };

        match game.send(16) {
            CoResult::Return(msg) => assert_eq!(msg.to_string(), "You guessed it!"),
            _ => panic!("The answer was correct! The coroutine should return."),
        }
    }

    enum Finish<G, P> {
        GameWon(G),
        PlayerGaveUp(P),
    }

    fn play(secret: i64, lo: i64, hi: i64) -> Finish<Correct, Quit> {
        let (_, mut game) = match (Game {}.send(secret)) {
            CoResult::Suspend(Variant(s)) => s,
        };

        let player = Maverick {};

        let (mut guess, mut player) = match player.send((lo, hi)) {
            CoResult::Suspend(Variant(x)) => x,
            CoResult::Return(_) => panic!("The range is valid, why did you return?"),
        };


        loop {
            let (result, game_) = match game.send(guess) {
                CoResult::Suspend(Variant((r, g))) => (Variant(r), g),
                CoResult::Suspend(Next(Variant((r, g)))) => (Next(Variant(r)), g),
                CoResult::Return(r) => break Finish::GameWon(r),
            };
            game = game_;
            let (guess_, player_) = match player.send(result) {
                CoResult::Suspend(Variant(s)) => s,
                CoResult::Return(r) => break Finish::PlayerGaveUp(r),
            };
            guess = guess_;
            player = player_;
        }
    }

    #[test]
    fn solo() {
        let msg = match play(75, 10, 100) {
            Finish::GameWon(e) => e.to_string(),
            Finish::PlayerGaveUp(q) => q.to_string(),
        };
        assert_eq!(msg, "You guessed it!");

        let msg = match play(5, 10, 100) {
            Finish::GameWon(e) => e.to_string(),
            Finish::PlayerGaveUp(q) => q.to_string(),
        };
        assert_eq!(msg, "I quit!");
    }
}
