use fsm::{State, ContinuationSet};
use cat::enums::Match;
use cat::sum::{Sum, Either};
use std::marker::PhantomData;
use cat::prod::Prod;


pub enum Transition<Next, Exit> {
    Next(Next),
    Exit(Exit),
}

impl<Next, Exit> Sum for Transition<Next, Exit> {
    type Left = Next;
    type Right = Exit;

    fn to_canonical(self) -> Either<Self::Left, Self::Right> {
        match self {
            Transition::Next(next) => Either::Left(next),
            Transition::Exit(exit) => Either::Right(exit),
        }
    }
}

pub struct MatchContinuation<C>(PhantomData<C>);

impl ContinuationSet for MatchContinuation<!> {
    type Emit = !;
    type Continue = !;
    type Head = !;
    type Suspend = !;
    type Output = !;
}

impl<H, T> ContinuationSet for MatchContinuation<Match<H, T>>
where H: Prod,
      H::Right: State,
      MatchContinuation<T>: ContinuationSet
{
    type Emit = H::Left;
    type Continue = H::Right;
    type Head = H;
    type Suspend = MatchContinuation<T>;
    type Output = Match<Self::Head, <Self::Suspend as ContinuationSet>::Output>;
}

impl<N, E> ContinuationSet for Transition<N, E>
where MatchContinuation<N>: ContinuationSet
{
    type Emit = <MatchContinuation<N> as ContinuationSet>::Emit;
    type Continue = <MatchContinuation<N> as ContinuationSet>::Continue;
    type Head = <MatchContinuation<N> as ContinuationSet>::Head;
    type Suspend = <MatchContinuation<N> as ContinuationSet>::Suspend;
    type Output = <MatchContinuation<N> as ContinuationSet>::Output;
}

#[cfg(test)]
mod tests {
    use fsm::State;
    use cat::enums::Match::*;
//    use cat::enums::Match;
//    use cat::sum::{Sum, Either};
//    use cat::prod::Prod;
    use super::Transition;
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

        type Transition = Transition<enums![(TooSmall, Guess), (TooBig, Guess)], Self::Exit>;

        fn send(self, i: Self::Input) -> Self::Transition {
            if self.0 == i {
                Transition::Exit(Correct {})
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

        type Transition = Transition<enums![((), Guess)], Self::Exit>;

        fn send(self, secret: Self::Input) -> Self::Transition {
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

        type Transition = Transition<enums![(i64, Strategist)], Self::Exit>;

        fn send(self, range: Self::Input) -> Self::Transition {
            if range.0 > range.1 {
                Transition::Exit(Quit {})
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

        type Transition = Transition<enums![(i64, Strategist)], Self::Exit>;

        fn send(self, result: Self::Input) -> Self::Transition {
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
            Transition::Exit(msg) => assert_eq!(msg.to_string(), "You guessed it!"),
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
            Transition::Exit(_) => panic!("The range is valid, why did you return?"),
        };


        loop {
            let (result, game_) = match game.send(guess) {
                Transition::Next(Variant((r, g))) => (Variant(r), g),
                Transition::Next(Next(Variant((r, g)))) => (Next(Variant(r)), g),
                Transition::Exit(r) => break Finish::GameWon(r),
            };
            game = game_;
            let (guess_, player_) = match player.send(result) {
                Transition::Next(Variant(s)) => s,
                Transition::Exit(r) => break Finish::PlayerGaveUp(r),
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
