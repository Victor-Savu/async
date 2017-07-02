use map::ret::{MapReturn, CoMapReturn};
use co::{Coroutine, CoResult};
use comb::join::{Join, CoJoin};


pub struct CoChain<F, L>(CoJoin<CoMapReturn<F, L>>)
    where F: Coroutine<Continue = F>,
          L: FnOnce<(F::Return,)>,
          L::Output: Coroutine<Yield = F::Yield, Continue = L::Output>;

impl<F, L> Coroutine for CoChain<F, L>
    where F: Coroutine<Continue = F>,
          L: FnOnce<(F::Return,)>,
          L::Output: Coroutine<Yield = F::Yield, Continue = L::Output>
{
    type Yield = F::Yield;
    type Return = <L::Output as Coroutine>::Return;
    type Continue = Self;

    fn next(self) -> CoResult<Self> {
        match self.0.next() {
            CoResult::Yield(y, s) => CoResult::Yield(y, CoChain(s)),
            CoResult::Return(r) => CoResult::Return(r),
        }
    }
}

pub trait Chain: Coroutine<Continue = Self> {
    fn chain<L>(self, l: L) -> CoChain<Self, L>
        where L: FnOnce<(Self::Return,)>,
              L::Output: Coroutine<Yield = Self::Yield, Continue = L::Output>;
}

impl<F> Chain for F
    where F: Coroutine<Continue = Self>
{
    fn chain<L>(self, l: L) -> CoChain<F, L>
        where L: FnOnce<(Self::Return,)>,
              L::Output: Coroutine<Yield = Self::Yield, Continue = L::Output>
    {
        CoChain(self.map_return(l).join())
    }
}
