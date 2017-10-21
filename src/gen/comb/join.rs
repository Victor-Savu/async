use gen::{Generator, GenResult, Yields, Returns};
use cat::sum::Either;
use cat::{Iso, Inj};

pub enum GenJoin<C>
    where C: Generator
{
    Outer(C),
    Inner(C::Return),
}

impl<C> Yields for GenJoin<C>
    where C: Yields
{
    type Yield = C::Yield;
}

impl<C> Returns for GenJoin<C>
    where C: Returns,
          C::Return: Returns
{
    type Return = <C::Return as Returns>::Return;
}

impl<C> Generator for GenJoin<C>
    where C: Generator,
          C::Return: Generator<Yield = C::Yield>,
          <C::Return as Generator>::Transition: Iso<Either<(C::Yield, C::Return),
                                                           <C::Return as Returns>::Return>>
{
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self {
            GenJoin::Outer(c) => {
                match c.next().inj() {
                    Either::Left(s) => {
                        let (y, outer) = s.inj();
                        GenResult::Yield(y, outer.join())
                    }
                    Either::Right(inner) => GenJoin::Inner(inner).next(),
                }
            }
            GenJoin::Inner(c) => {
                match c.next().inj() {
                    Either::Left(s) => {
                        let (y, inner) = s.inj();
                        GenResult::Yield(y, GenJoin::Inner(inner))
                    }
                    Either::Right(result) => GenResult::Return(result),
                }
            }
        }
    }
}

pub trait Join
    where Self: Generator
{
    fn join(self) -> GenJoin<Self> {
        GenJoin::Outer(self)
    }
}

impl<C> Join for C where C: Generator {}


#[cfg(test)]
mod tests {

    use gen::comb::join::Join;
    use gen::map::ret::MapReturn;
    use gen::iter::wrap::Wrap;

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
