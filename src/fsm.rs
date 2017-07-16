use enums::Enum;


pub trait ContinuationSet {
    type Emit;
    type Continue: State;
    type Suspend: ContinuationSet;
}

impl ContinuationSet for ! {
    type Emit = !;
    type Continue = !;
    type Suspend = !;
}

impl State for ! {
    type Input = !;
    type Exit = !;
    type Next = !;

    fn send(self, _: Self::Input) -> Transition<Self::Next, Self::Exit> {
        unreachable!()
    }
}

impl<Enumeration, Y, C> ContinuationSet for Enumeration
    where C: State,
          Enumeration: Enum<Variant = (Y, C)>,
          Enumeration::Next: ContinuationSet
{
    type Emit = Y;
    type Continue = C;
    type Suspend = Enumeration::Next;
}

pub enum Transition<Next, Return>
    where Next: ContinuationSet
{
    Next(Next),
    Return(Return),
}

pub trait State: Sized {
    type Input;
    type Exit;
    type Next: ContinuationSet;

    fn send(self, i: Self::Input) -> Transition<Self::Next, Self::Exit>;
}


#[cfg(test)]
mod tests {

    #![macro_use]

    use super::{State, Transition};
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

    impl State for Guess {
        type Input = i64;
        type Exit = Correct;

        type Next = enums![(TooSmall, Guess), (TooBig, Guess)];

        fn send(self, i: Self::Input) -> Transition<Self::Next, Self::Exit> {
            if self.0 == i {
                Transition::Return(Correct {})
            } else if i < self.0 {
                Transition::Next(Variant((TooSmall {}, self)))
            } else {
                Transition::Next(Next(Variant((TooBig {}, self))))
            }
        }
    }

    struct Game;

    impl State for Game {
        type Input = i64;
        type Exit = !;

        type Next = enums![((), Guess)];

        fn send(self, secret: Self::Input) -> Transition<Self::Next, Self::Exit> {
            Transition::Next(Variant(((), Guess(secret))))
        }
    }

    struct Quit;

    impl fmt::Display for Quit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "I quit!")
        }
    }

    struct Maverick;

    impl State for Maverick {
        type Input = (i64, i64);
        type Exit = Quit;

        type Next = enums![(i64, Strategist)];

        fn send(self, range: Self::Input) -> Transition<Self::Next, Self::Exit> {
            if range.0 > range.1 {
                Transition::Return(Quit {})
            } else {
                let guess = (range.0 + range.1) / 2;
                Transition::Next(Variant((guess, Strategist(range.0, guess, range.1))))
            }
        }
    }

    struct Strategist(i64, i64, i64);

    impl State for Strategist {
        type Input = enums![TooSmall, TooBig];
        type Exit = Quit;

        type Next = enums![(i64, Strategist)];

        fn send(self, result: Self::Input) -> Transition<Self::Next, Self::Exit> {
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
            Transition::Next(Variant(((), game))) => game,
        };

        let suspend = match game.send(10) {
            Transition::Next(suspend) => suspend,
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
            Transition::Next(suspend) => suspend,
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
            Transition::Return(msg) => assert_eq!(msg.to_string(), "You guessed it!"),
            _ => panic!("The answer was correct! The coroutine should return."),
        }
    }

    enum Finish<G, P> {
        GameWon(G),
        PlayerGaveUp(P),
    }

    fn play(secret: i64, lo: i64, hi: i64) -> Finish<Correct, Quit> {
        let (_, mut game) = match (Game {}.send(secret)) {
            Transition::Next(Variant(s)) => s,
        };

        let player = Maverick {};

        let (mut guess, mut player) = match player.send((lo, hi)) {
            Transition::Next(Variant(x)) => x,
            Transition::Return(_) => panic!("The range is valid, why did you return?"),
        };


        loop {
            let (result, game_) = match game.send(guess) {
                Transition::Next(Variant((r, g))) => (Variant(r), g),
                Transition::Next(Next(Variant((r, g)))) => (Next(Variant(r)), g),
                Transition::Return(r) => break Finish::GameWon(r),
            };
            game = game_;
            let (guess_, player_) = match player.send(result) {
                Transition::Next(Variant(s)) => s,
                Transition::Return(r) => break Finish::PlayerGaveUp(r),
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
