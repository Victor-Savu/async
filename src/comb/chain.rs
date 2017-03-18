use co::{Coroutine, CoResult};

pub enum CoChain<F, L> {
    Former(F, L),
    Latter(L),
}

impl<F, L, Input> Coroutine<Input> for CoChain<F, L>
    where F: Coroutine<Input>,
          L: Coroutine<Input>,
          Input: From<<F as Coroutine<Input>>::Return>,
          <F as Coroutine<Input>>::Yield: From<<L as Coroutine<Input>>::Yield>
{
    type Yield = <F as Coroutine<Input>>::Yield;
    type Return = <L as Coroutine<Input>>::Return;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        match self {
            CoChain::Former(fmr, ltr) => {
                match fmr.next(i) {
                    CoResult::Yield(y, fmr) => CoResult::Yield(y, fmr.chain(ltr)),
                    CoResult::Return(retf) => CoChain::Latter(ltr).next(retf.into()),
                }
            }
            CoChain::Latter(ltr) => {
                match ltr.next(i) {
                    CoResult::Yield(y, ltr) => CoResult::Yield(y.into(), CoChain::Latter(ltr)),
                    CoResult::Return(retl) => CoResult::Return(retl),
                }
            }
        }
    }
}

pub trait Chain<L, Input> where Self: Sized
{
    fn chain(self, l: L) -> CoChain<Self, L> {
        CoChain::Former(self, l)
    }
}

impl<F, L, Input> Chain<L, Input> for F
    where F: Sized + Coroutine<Input>,
          L: Coroutine<Input>,
          Input: From<<F as Coroutine<Input>>::Return>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Counter<T> {
        i: T,
        lim: T,
    }

    impl Coroutine<()> for Counter<i64> {
        type Yield = i64;
        type Return = ();

        fn next(self, _: ()) -> CoResult<Self::Yield, Self, Self::Return> {
            if self.i < self.lim {
                CoResult::Yield(self.i,
                                Counter {
                                    i: self.i + 1,
                                    lim: self.lim,
                                })
            } else {
                CoResult::Return(())
            }
        }
    }

    #[test]
    fn chain_integers() {
        let first = Counter::<i64> { i: 1, lim: 9 };
        let second = Counter::<i64> { i: 1, lim: 3 };

        let chain = first.chain(second);
        let msg = "This is the end";
        let mut elem = 1;
        let message = each!(chain => i in {
            assert_eq!(i, elem);
            elem = if elem == 8 { 1 } else { elem + 1 };
        } then {
            msg
        });
        assert_eq!(elem, 3);
        assert_eq!(message, msg);
    }
}
