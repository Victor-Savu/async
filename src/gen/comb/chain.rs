use cat::Iso;
use cat::sum::Either;
use gen::map::ret::{MapReturn, GenMapReturn};
use gen::{Generator, GenResult, Returns, Yields};
use gen::comb::join::{Join, GenJoin};


pub struct GenChain<F, L>(GenJoin<GenMapReturn<F, L>>);

impl<F, L> Yields for GenChain<F, L>
    where F: Yields
{
    type Yield = F::Yield;
}

impl<F, L> Returns for GenChain<F, L>
    where F: Returns,
          L: FnOnce<(F::Return,)>,
          L::Output: Returns
{
    type Return = <L::Output as Returns>::Return;
}

impl<F, L> Generator for GenChain<F, L>
{
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self.0.next() {
            GenResult::Yield(y, s) => GenResult::Yield(y, GenChain(s)),
            GenResult::Return(r) => GenResult::Return(r),
        }
    }
}

pub trait Chain
{
    fn chain<L>(self, l: L) -> GenChain<Self, L>
    {
        GenChain(self.map_return(l).join())
    }
}

impl<F> Chain for F {}

#[cfg(test)]
mod tests {

    use gen::iter::wrap::Wrap;
    use gen::comb::chain::Chain;

    #[test]
    fn chain_integers() {
        let first = (1..9).wrap();
        let second = (1..3).wrap();

        let the_chain = first.chain(|_| second);
        let msg = "This is the end";
        let mut elem = 1;
        let message = each!(the_chain => i in {
            assert_eq!(i, elem);
            elem = if elem == 8 { 1 } else { elem + 1 };
        } then {
            msg
        });
        assert_eq!(elem, 3);
        assert_eq!(message, msg);
    }
}
