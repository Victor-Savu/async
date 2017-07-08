use map::ret::{MapReturn, GenMapReturn};
use gen::{Generator, GenResult};
use comb::join::{Join, GenJoin};


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
