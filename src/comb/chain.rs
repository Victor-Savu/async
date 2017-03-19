use co::{Coroutine, CoResult};

pub enum CoState<C>
    where C: Coroutine
{
    Live(C),
    Done(C::Return),
}

pub struct CoChain<F, L>
    where F: Coroutine
{
    former: CoState<F>,
    latter: L,
}

impl<F, L> Coroutine for CoChain<F, L>
    where F: Coroutine,
          L: Coroutine,
          F::Yield: From<L::Yield>
{
    type Yield = F::Yield;
    type Return = (F::Return, L::Return);

    fn next(self) -> CoResult<Self::Yield, Self, Self::Return> {
        match self.former {
            CoState::Live(former) => {
                match former.next() {
                    CoResult::Yield(y, fmr) => CoResult::Yield(y, fmr.chain(self.latter)),
                    CoResult::Return(retf) => {
                        CoChain {
                                former: CoState::Done(retf),
                                latter: self.latter,
                            }
                            .next()
                    }
                }
            }
            CoState::Done(result) => {
                match self.latter.next() {
                    CoResult::Yield(y, ltr) => {
                        CoResult::Yield(y.into(),
                                        CoChain {
                                            former: CoState::Done(result),
                                            latter: ltr,
                                        })
                    }
                    CoResult::Return(retl) => CoResult::Return((result, retl)),
                }
            }
        }
    }
}

pub trait Chain {
    type Former: Coroutine;

    fn chain<L>(self, l: L) -> CoChain<Self::Former, L>
        where L: Coroutine,
              <<Self as Chain>::Former as Coroutine>::Yield: From<L::Yield>;
}

impl<F> Chain for F
    where F: Coroutine
{
    type Former = Self;
    fn chain<L>(self, l: L) -> CoChain<Self::Former, L>
        where L: Coroutine,
              <<Self as Chain>::Former as Coroutine>::Yield: From<L::Yield>
    {
        CoChain {
            former: CoState::Live(self),
            latter: l,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Counter<T> {
        i: T,
        lim: T,
    }

    impl Coroutine for Counter<i64> {
        type Yield = i64;
        type Return = ();

        fn next(self) -> CoResult<Self::Yield, Self, Self::Return> {
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
