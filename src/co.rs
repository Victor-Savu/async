use either::Either;


pub trait CoSuspend<Output> {
    type Yield;
    type Continuation: Coroutine<Output>;
    type Suspension: CoSuspend<Output>;

    fn get(self) -> Either<(Self::Yield, Self::Continuation), Self::Suspension>;
}

impl<Output> CoSuspend<Output> for ! {
    type Yield = !;
    type Continuation = !;
    type Suspension = !;

    fn get(self) -> Either<(Self::Yield, Self::Continuation), Self::Suspension> {
        unreachable!()
    }
}

pub enum CoResult<Suspend, Output>
    where Suspend: CoSuspend<Output>
{
    Suspend(Suspend),
    Return(Output),
}

pub trait Coroutine<Output>: Sized {
    type Input;
    type Suspend: CoSuspend<Output>;

    fn send(self, i: Self::Input) -> CoResult<Self::Suspend, Output>;
}

impl<Output> Coroutine<Output> for ! {
    type Input = !;
    type Suspend = !;

    fn send(self, _: Self::Input) -> CoResult<Self::Suspend, Output> {
        unreachable!()
    }
}


#[cfg(test)]
mod tests {

    use super::{Coroutine, CoSuspend, CoResult};
    use either::Either;

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

    impl CoSuspend<&'static str> for TooBig {
        type Yield = &'static str;
        type Continuation = Guess;
        type Suspension = !;

        fn get(self) -> Either<(Self::Yield, Self::Continuation), Self::Suspension> {
            Either::Former(("Too big!", self.0))
        }
    }

    impl CoSuspend<&'static str> for Retry {
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

    impl Coroutine<&'static str> for Guess {
        type Input = i64;
        type Suspend = Retry;

        fn send(self, i: Self::Input) -> CoResult<Self::Suspend, &'static str> {
            if self.0 == i {
                CoResult::Return("You guessed it!")
            } else if i < self.0 {
                CoResult::Suspend(Retry::too_small(self))
            } else {
                CoResult::Suspend(Retry::too_big(self))
            }
        }
    }

    fn play_guess(secret: i64) -> Guess {
        Guess(secret)
    }

    /*
    fn play_guess(secret: i64) {
        return move |guess: i64| {
            let mut guess = guess;
            loop {
                guess = if guess == secret {
                    return "You guessed it!";
                } else if guess < secret {
                    yield "Too small!";
                } else {
                    yield "Too big!";
                }
            }
        }
    */

    #[test]
    fn guess() {
        let game = play_guess(16);

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
}
