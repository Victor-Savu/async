use co::{Coroutine, CoResult};

pub enum CoJoin<C>
    where C: Coroutine
{
    Outer(C),
    Inner(C::Return),
}

impl<C> Coroutine for CoJoin<C>
    where C: Coroutine,
          C::Return: Coroutine,
          C::Yield: From<<C::Return as Coroutine>::Yield>
{
    type Yield = C::Yield;
    type Return = <C::Return as Coroutine>::Return;

    fn next(self) -> CoResult<Self> {
        match self {
            CoJoin::Outer(c) => {
                match c.next() {
                    CoResult::Yield(y, outer) => CoResult::Yield(y, outer.join()),
                    CoResult::Return(inner) => CoJoin::Inner(inner).next(),
                }
            }
            CoJoin::Inner(c) => {
                match c.next() {
                    CoResult::Yield(y, inner) => CoResult::Yield(y.into(), CoJoin::Inner(inner)),
                    CoResult::Return(result) => CoResult::Return(result),
                }
            }
        }
    }
}

pub trait Join: Sized {
    fn join(self) -> CoJoin<Self>
        where Self: Coroutine,
              Self::Return: Coroutine,
              Self::Yield: From<<Self::Return as Coroutine>::Yield>
    {
        CoJoin::Outer(self)
    }
}

impl<C> Join for C
    where C: Coroutine,
          C::Return: Coroutine,
          C::Yield: From<<C::Return as Coroutine>::Yield>
{
}
