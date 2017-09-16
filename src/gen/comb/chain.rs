use gen::map::ret::{MapReturn, GenMapReturn};
use gen::{Generator, GenResult};
use gen::comb::join::{Join, GenJoin};


pub struct GenChain<F, L>(GenJoin<GenMapReturn<F, L>>)
    where F: Generator,
          L: FnOnce<(F::Return,)>,
          L::Output: Generator<Yield = F::Yield>;

impl<F, L> Generator for GenChain<F, L>
    where F: Generator,
          L: FnOnce<(F::Return,)>,
          L::Output: Generator<Yield = F::Yield>
{
    type Yield = F::Yield;
    type Return = <L::Output as Generator>::Return;
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self.0.next() {
            GenResult::Yield(y, s) => GenResult::Yield(y, GenChain(s)),
            GenResult::Return(r) => GenResult::Return(r),
        }
    }
}

pub trait Chain
    where Self: Generator
{
    fn chain<L>(self, l: L) -> GenChain<Self, L>
        where L: FnOnce<(Self::Return,)>,
              L::Output: Generator<Yield = Self::Yield>
    {
        GenChain(self.map_return(l).join())
    }
}

impl<F> Chain for F where F: Generator {}

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
