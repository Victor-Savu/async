use std::marker::PhantomData;
use enums::Enum;


pub trait CoSuspend {
    type Yield;
    type Output;
    type Continuation: Coroutine<Output = Self::Output>;
    type Suspension: CoSuspend<Output = Self::Output>;
}

pub struct NeverReach<Output>(PhantomData<Output>);

impl<T> Enum for NeverReach<T> {
    type Variant = !;
    type Next = !;
}

#[macro_export]
macro_rules! suspend {
    ($head:ty, $(($taily:ty, $tailc:ty)),+) => {
        $crate::enums::Match<$head, suspend![ $(($taily, $tailc)),* ]>
    };

    (($yields:ty, $continues:ty)) => {
        $crate::enums::Match<($yields, $continues), $crate::co::NeverReach<<$continues as $crate::co::Coroutine>::Output>>
    };
}


impl<T> CoSuspend for NeverReach<T> {
    type Yield = !;
    type Output = T;
    type Continuation = Self;
    type Suspension = Self;
}

impl<T> Coroutine for NeverReach<T> {
    type Input = !;
    type Output = T;
    type Suspend = Self;

    fn send(self, _: Self::Input) -> CoResult<Self::Suspend> {
        unreachable!()
    }
}

impl<Enumeration, Y, C> CoSuspend for Enumeration
    where C: Coroutine,
          Enumeration: Enum<Variant = (Y, C)>,
          Enumeration::Next: CoSuspend<Output = C::Output>
{
    type Yield = Y;
    type Output = C::Output;
    type Continuation = C;
    type Suspension = Enumeration::Next;
}

pub enum CoResult<Suspend>
    where Suspend: CoSuspend
{
    Suspend(Suspend),
    Return(Suspend::Output),
}

pub trait Coroutine: Sized {
    type Input;
    type Output;
    type Suspend: CoSuspend<Output = Self::Output>;

    fn send(self, i: Self::Input) -> CoResult<Self::Suspend>;
}


#[cfg(test)]
mod tests {

    #![macro_use]

    use super::{Coroutine, CoResult};
    use enums::Match::*;

    struct Guess(i64);

    impl Coroutine for Guess {
        type Input = i64;
        type Output = &'static str;

        type Suspend = suspend![(&'static str, Guess), (&'static str, Guess)];

        fn send(self, i: Self::Input) -> CoResult<Self::Suspend> {
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
