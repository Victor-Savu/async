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
