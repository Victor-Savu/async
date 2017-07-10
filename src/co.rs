use enums::Enum;


pub trait CoSuspend<Output> {
    type Yield;
    type Continuation: Coroutine<Output>;
    type Suspension: CoSuspend<Output>;
}

impl<Output> CoSuspend<Output> for ! {
    type Yield = !;
    type Continuation = !;
    type Suspension = !;
}

impl<Output, Enumeration, Y, C> CoSuspend<Output> for Enumeration
    where C: Coroutine<Output>,
          Enumeration: Enum<Variant = (Y, C)>,
          Enumeration::Next: CoSuspend<Output>
{
    type Yield = Y;
    type Continuation = C;
    type Suspension = Enumeration::Next;
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

    #![macro_use]

    use super::{Coroutine, CoResult};
    use enums::Match::*;

    struct Guess(i64);

    impl Coroutine<&'static str> for Guess {
        type Input = i64;

        type Suspend = enums![(&'static str, Guess), (&'static str, Guess)];

        fn send(self, i: Self::Input) -> CoResult<Self::Suspend, &'static str> {
            if self.0 == i {
                CoResult::Return("You guessed it!")
            } else if i < self.0 {
                CoResult::Suspend(Variant(("Too small!", self)))
            } else {
                CoResult::Suspend(Next(Variant(("Too big!", self))))
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

        let game = match suspend {
            Variant((msg, guess_again)) => {
                assert_eq!(msg, "Too small!");
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
                assert_eq!(msg, "Too big!");
                guess_again
            }
            _ => panic!("The number was clearly too big! Why do you not see this?"),
        };

        match game.send(16) {
            CoResult::Return(msg) => assert_eq!(msg, "You guessed it!"),
            _ => panic!("The answer was correct! The coroutine should return."),
        }
    }
}
