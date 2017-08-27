use gen::{Generator, GenResult};
use cat::sum::{Either, Sum};
use cat::prod::Prod;

pub enum GenJoin<C>
    where C: Generator
{
    Outer(C),
    Inner(C::Return),
}

impl<C> Generator for GenJoin<C>
    where C: Generator,
          C::Return: Generator<Yield = C::Yield>
{
    type Yield = C::Yield;
    type Return = <C::Return as Generator>::Return;
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self {
            GenJoin::Outer(c) => {
                match c.next().to_canonical() {
                    Either::Left(s) => {
                        let (y, outer) = s.to_canonical();
                        GenResult::Yield(y, outer.join())
                    }
                    Either::Right(inner) => GenJoin::Inner(inner).next(),
                }
            }
            GenJoin::Inner(c) => {
                match c.next().to_canonical() {
                    Either::Left(s) => {
                        let (y, inner) = s.to_canonical();
                        GenResult::Yield(y, GenJoin::Inner(inner))
                    }
                    Either::Right(result) => GenResult::Return(result),
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


#[cfg(test)]
mod tests {

    use comb::join::Join;
    use map::ret::MapReturn;
    use iter::wrap::Wrap;

    #[test]
    fn chain_integers() {
        let first = (1..9).wrap();
        let second = (1..3).wrap();

        let msg = "This is the end";
        let chain = first.map_return(|_| second.map_return(|_| msg)).join();
        let mut elem = 1;
        let message = each!(chain => i in {
            assert_eq!(i, elem);
            elem = if elem == 8 { 1 } else { elem + 1 };
        });
        assert_eq!(elem, 3);
        assert_eq!(message, msg);
    }
}
