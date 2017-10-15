//! The standard type used for defining a `fsm::State::Transition`
//!
//! This is the go-to type for when you are implementing `State` for your type.
//!
//! # Examples
//!
//! ```
//! #![feature(never_type)]
//! #[macro_use] extern crate o3;
//!
//! use o3::fsm::State;
//! use o3::fsm::match_transition::Transition;
//! use o3::cat::enums::Match::*;
//!
//! struct Idle;
//! struct Payment;
//! struct Pouring;
//!
//! enum Drink {
//!     Coffee,
//!     HotChocolate,
//!     HotWater,
//! }
//!
//! impl State for Idle {
//!     type Input = Drink;
//!     type Transition = Transition<enums![(&'static str, Payment), (&'static str, Pouring)], !>;
//!
//!     fn send(self, product: Self::Input) -> Self::Transition {
//!         match product {
//!             Drink::Coffee => Transition::Continue(Variant(("Ten cents and a piece of your soul.", Payment{}))),
//!             Drink::HotChocolate => Transition::Continue(Variant(("9c + $2 for the cream", Payment{}))),
//!             Drink::HotWater => Transition::Continue(Next(Variant(("Tea if free, as you should be!", Pouring{})))),
//!         }
//!     }
//! }
//!
//! struct DonePouring;
//!
//! impl State for Pouring {
//!     type Input = DonePouring;
//!     type Transition = Transition<enums![(&'static str, Idle)], !>;
//!
//!     fn send(self, _: Self::Input) -> Self::Transition {
//!         Transition::Continue(Variant(("Enjoy!", Idle{})))
//!     }
//! }
//!
//! enum Decision {
//!     Pay,
//!     Cancel,
//!     SmashTheMachine,
//! }
//!
//! struct ContortedMetal;
//!
//! impl State for Payment {
//!     type Input = Decision;
//!     type Transition = Transition<enums![(&'static str, Pouring), (&'static str, Idle)], ContortedMetal>;
//!
//!     fn send(self, decision: Self::Input) -> Self::Transition {
//!         match decision {
//!             Decision::Pay => Transition::Continue(Variant(("Your kindness shall be rewarded!", Pouring{}))),
//!             Decision::Cancel => Transition::Continue(Next(Variant(("Suit yorself!", Idle{})))),
//!             Decision::SmashTheMachine => Transition::Exit(ContortedMetal{}),
//!         }
//!     }
//! }
//!
//! fn main() {}
//! ```
use fsm::{ContinuationList, Continuation, StateTransition};
use cat::{Iso, Sur, Inj};
use cat::enums::Match;
use cat::sum::{Sum, Either};
use std::marker::PhantomData;


pub struct MatchContinuation<C>(PhantomData<C>);

impl ContinuationList for MatchContinuation<!> {
    type Head = !;
    type Tail = !;
    type Output = !;
}

impl<H, T> ContinuationList for MatchContinuation<Match<H, T>>
    where H: Continuation,
          MatchContinuation<T>: ContinuationList
{
    type Head = H;
    type Tail = MatchContinuation<T>;
    type Output = Match<<Self::Head as Continuation>::Output, <Self::Tail as ContinuationList>::Output>;
}

pub enum Transition<Next, Exit> {
    Continue(Next),
    Exit(Exit),
}

impl<Next, Exit> Sum for Transition<Next, Exit> {
    type Left = Next;
    type Right = Exit;
    type Output = Self;
}

impl<A, B> Sur<Either<A, B>> for Transition<A, B> {
    fn sur(e: Either<A, B>) -> Self {
        match e {
            Either::Left(cont) => Transition::Continue(cont),
            Either::Right(exit) => Transition::Exit(exit),
        }
    }
}

unsafe impl<A, B> Iso<Either<A, B>> for Transition<A, B> {}

impl <A, B> Inj<Either<A, B>> for Transition<A, B> {
    fn inj(self) -> Either<A, B> {
        match self {
            Transition::Continue(cont) => Either::Left(cont),
            Transition::Exit(exit) => Either::Right(exit),
        }
    }
}
impl<N, E> StateTransition for Transition<N, E>
    where MatchContinuation<N>: ContinuationList
{
    type Continuation = MatchContinuation<N>;
    type Exit = E;
}

#[cfg(test)]
mod tests {
    use fsm::State;
    use cat::enums::Match::*;
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
        type Transition = Transition<enums![(TooSmall, Guess), (TooBig, Guess)], Correct>;

        fn send(self, i: Self::Input) -> Self::Transition {
            if self.0 == i {
                Transition::Exit(Correct {})
            } else if i < self.0 {
                Transition::Continue(Variant((TooSmall {}, self)))
            } else {
                Transition::Continue(Next(Variant((TooBig {}, self))))
            }
        }
    }

    struct Game;

    impl State for Game {
        type Input = i64;
        type Transition = Transition<enums![((), Guess)], !>;

        fn send(self, secret: Self::Input) -> Self::Transition {
            Transition::Continue(Variant(((), Guess(secret))))
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
        type Transition = Transition<enums![(i64, Strategist)], Quit>;

        fn send(self, range: Self::Input) -> Self::Transition {
            if range.0 > range.1 {
                Transition::Exit(Quit {})
            } else {
                let guess = (range.0 + range.1) / 2;
                Transition::Continue(Variant((guess, Strategist(range.0, guess, range.1))))
            }
        }
    }

    struct Strategist(i64, i64, i64);

    impl State for Strategist {
        type Input = enums![TooSmall, TooBig];
        type Transition = Transition<enums![(i64, Strategist)], Quit>;

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
            Transition::Continue(Variant(((), game))) => game,
        };

        let suspend = match game.send(10) {
            Transition::Continue(suspend) => suspend,
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
            Transition::Continue(suspend) => suspend,
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
            Transition::Continue(Variant(s)) => s,
        };

        let player = Maverick {};

        let (mut guess, mut player) = match player.send((lo, hi)) {
            Transition::Continue(Variant(x)) => x,
            Transition::Exit(_) => panic!("The range is valid, why did you return?"),
        };


        loop {
            let (result, game_) = match game.send(guess) {
                Transition::Continue(Variant((r, g))) => (Variant(r), g),
                Transition::Continue(Next(Variant((r, g)))) => (Next(Variant(r)), g),
                Transition::Exit(r) => break Finish::GameWon(r),
            };
            game = game_;
            let (guess_, player_) = match player.send(result) {
                Transition::Continue(Variant(s)) => s,
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
