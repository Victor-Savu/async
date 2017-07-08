use gen::{Generator, GenResult};

pub enum GenJoin<C>
    where C: Generator
{
    Outer(C),
    Inner(C::Return),
}

impl<C> Generator for GenJoin<C>
    where C: Generator,
          C::Return: Generator,
          C::Yield: From<<C::Return as Generator>::Yield>
{
    type Yield = C::Yield;
    type Return = <C::Return as Generator>::Return;

    fn next(self) -> GenResult<Self> {
        match self {
            GenJoin::Outer(c) => {
                match c.next() {
                    GenResult::Yield(y, outer) => GenResult::Yield(y, outer.join()),
                    GenResult::Return(inner) => GenJoin::Inner(inner).next(),
                }
            }
            GenJoin::Inner(c) => {
                match c.next() {
                    GenResult::Yield(y, inner) => GenResult::Yield(y.into(), GenJoin::Inner(inner)),
                    GenResult::Return(result) => GenResult::Return(result),
                }
            }
        }
    }
}

pub trait Join
    where Self: Generator,
          Self::Return: Generator,
          Self::Yield: From<<Self::Return as Generator>::Yield>
{
    fn join(self) -> GenJoin<Self> {
        GenJoin::Outer(self)
    }
}

impl<C> Join for C
    where C: Generator,
          C::Return: Generator,
          C::Yield: From<<C::Return as Generator>::Yield>
{
}
