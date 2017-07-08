use map::ret::{MapReturn, CoMapReturn};
use co::{Coroutine, CoResult};
use comb::join::{Join, CoJoin};


pub struct CoChain<F, L>(CoJoin<CoMapReturn<F, L>>)
    where F: Coroutine,
          L: FnOnce<(F::Return,)>,
          L::Output: Coroutine<Yield = F::Yield>;

impl<F, L> Coroutine for CoChain<F, L>
    where F: Coroutine,
          L: FnOnce<(F::Return,)>,
          L::Output: Coroutine<Yield = F::Yield>
{
    type Yield = F::Yield;
    type Return = <L::Output as Coroutine>::Return;

    fn next(self) -> CoResult<Self> {
        match self.0.next() {
            CoResult::Yield(y, s) => CoResult::Yield(y, CoChain(s)),
            CoResult::Return(r) => CoResult::Return(r),
        }
    }
}

pub trait Chain: Coroutine {
    fn chain<L>(self, l: L) -> CoChain<Self, L>
        where L: FnOnce<(Self::Return,)>,
              L::Output: Coroutine<Yield = Self::Yield>;
}

impl<F> Chain for F
    where F: Coroutine
{
    fn chain<L>(self, l: L) -> CoChain<F, L>
        where L: FnOnce<(Self::Return,)>,
              L::Output: Coroutine<Yield = Self::Yield>
    {
        CoChain(self.map_return(l).join())
    }
}
