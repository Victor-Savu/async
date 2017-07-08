#![feature(never_type)]

extern crate o3;

use o3::co::{Coroutine, CoSuspend, CoResult};
use o3::either::Either;

struct Guess(i64);

struct Retry {
    too_small: bool,
    guess: Guess,
}

impl Retry {
    fn too_small(guess: Guess) -> Self {
        Retry { too_small: true, guess }
    }

    fn too_big(guess: Guess) -> Self {
        Retry { too_small: false, guess }
    }
}

struct TooBig(Guess);

impl CoSuspend for TooBig {
    type Yield = &'static str;
    type Continuation = Guess;
    type Suspension = !;

    fn get(self) -> Either<(Self::Yield, Self::Continuation), Self::Suspension> {
        Either::Former(("Too big!", self.0))
    }
}

impl CoSuspend for Retry {
    type Yield = &'static str;
    type Continuation = Guess;
    type Suspension = TooBig;

    fn get(self) -> Either<(Self::Yield, Self::Continuation), Self::Suspension> {
        if self.too_small {
            Either::Former(("Too small!", self.guess))
        } else {
            Either::Latter(TooBig(self.guess))
        }
    }
}

impl Coroutine for Guess {
    type Input = i64;
    type Suspend = Retry;
    type Output = &'static str;

    fn send(self, i: Self::Input) -> CoResult<Self> {
        if self.0 == i {
            CoResult::Return("You guessed it!")
        } else if i < self.0 {
            CoResult::Suspend(Retry::too_small(self))
        } else {
            CoResult::Suspend(Retry::too_big(self))
        }
    }
}

#[test]
fn guess() {
    let game = Guess(16);

    let suspend = match game.send(10) {
        CoResult::Suspend(suspend) => suspend,
        _ => panic!("We didn't guess the number, so the coroutine should not return yet!"),
    };

    let game = match suspend.get() {
        Either::Former((msg, guess_again)) => {
            assert_eq!(msg, "Too small!");
            guess_again
        },
        _ => panic!("The number was clearly too small! Why do you not see this?")
    };

    let suspend = match game.send(19) {
        CoResult::Suspend(suspend) => suspend,
        _ => panic!("We didn't guess the number, so the coroutine should not return yet!"),
    };

    let suspend = match suspend.get() {
        Either::Latter(suspend) => suspend,
        _ => panic!("The number was clearly too big! Why do you not see this?")
    };

    let game = match suspend.get() {
        Either::Former((msg, guess_again)) => {
            assert_eq!(msg, "Too big!");
            guess_again
        }
    };
        
    match game.send(16) {
        CoResult::Return(msg) => assert_eq!(msg, "You guessed it!"),
        _ => panic!("The answer was correct! The coroutine should return.")
    }
}
