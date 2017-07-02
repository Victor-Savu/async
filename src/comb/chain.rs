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

pub trait Chain: Coroutine<Continue = Self>
{
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


#[cfg(test)]
mod tests {
    use super::*;
    use co::CoResult;

    struct Counter<T> {
        i: T,
        lim: T,
    }

    impl Coroutine for Counter<i64> {
        type Yield = i64;
        type Return = ();
        type Continue = Self;

        fn next(self) -> CoResult<Self> {
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
