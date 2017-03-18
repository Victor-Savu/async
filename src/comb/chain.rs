use co::{Coroutine, CoResult};

pub enum CoChain<F, L> {
    Former(F, L),
    Latter(L),
}

impl<F, L, Input> Coroutine<Input> for CoChain<F, L>
    where F: Coroutine<Input, Return = Input>,
          L: Coroutine<Input, Yield = <F as Coroutine<Input>>::Yield>
{
    type Yield = <F as Coroutine<Input>>::Yield;
    type Return = <L as Coroutine<Input>>::Return;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        match self {
            CoChain::Former(fmr, ltr) => {
                match fmr.next(i) {
                    CoResult::Yield(y, fmr) => CoResult::Yield(y, fmr.chain(ltr)),
                    CoResult::Return(retf) => CoChain::Latter(ltr).next(retf),
                }
            }
            CoChain::Latter(ltr) => {
                match ltr.next(i) {
                    CoResult::Yield(y, ltr) => CoResult::Yield(y, CoChain::Latter(ltr)),
                    CoResult::Return(retl) => CoResult::Return(retl),
                }
            }
        }
    }
}

pub trait Chain<L, Input>
    where L: Coroutine<Input>,
          Self: Sized + Coroutine<Input, Return = Input>
{
    fn chain(self, l: L) -> CoChain<Self, L> {
        CoChain::Former(self, l)
    }
}

impl<F, L, Input> Chain<L, Input> for F
    where F: Coroutine<Input, Return = Input>,
          L: Coroutine<Input>
{
}
