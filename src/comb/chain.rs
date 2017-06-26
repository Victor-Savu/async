use map::ret::{MapReturn, CoMapReturn};
use co::{Coroutine, CoResult};
use join::{Join, CoJoin};


pub struct Prepend<F>(F);

impl<F> Prepend<F> {
    pub fn new(f: F) -> Self {
        Prepend(f)
    }
}

impl<I, F> FnOnce<(I,)> for Prepend<F> {
    type Output = (F, I);

    extern "rust-call" fn call_once(self, (i,): (I,)) -> Self::Output {
        (self.0, i)
    }
}

pub struct PrependToReturn<F>(F) where F: Coroutine;

impl<F> PrependToReturn<F>
    where F: Coroutine
{
    pub fn new(f: F) -> Self {
        PrependToReturn(f)
    }
}

impl<I, F> FnOnce<(I,)> for PrependToReturn<F>
    where F: Coroutine
{
    type Output = CoMapReturn<F, Prepend<I>>;

    extern "rust-call" fn call_once(self, (i,): (I,)) -> Self::Output {
        self.0.map_return(Prepend::new(i))
    }
}

pub struct CoChain<F, L>(CoJoin<CoMapReturn<F, PrependToReturn<L>>>)
    where F: Coroutine,
          L: Coroutine<Yield = F::Yield>;

impl<F, L> Coroutine for CoChain<F, L>
    where F: Coroutine,
          L: Coroutine<Yield = F::Yield>
{
    type Yield = F::Yield;
    type Return = (F::Return, L::Return);

    fn next(self) -> CoResult<Self> {
        match self.0.next() {
            CoResult::Yield(y, s) => CoResult::Yield(y, CoChain(s)),
            CoResult::Return(r) => CoResult::Return(r),
        }
    }
}

pub trait Chain
    where Self: Coroutine
{
    fn chain<L>(self, l: L) -> CoChain<Self, L> where L: Coroutine<Yield = Self::Yield>;
}

impl<F> Chain for F
    where F: Coroutine
{
    fn chain<L>(self, l: L) -> CoChain<F, L>
        where L: Coroutine<Yield = F::Yield>
    {
        CoChain(self.map_return(PrependToReturn::new(l)).join())
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

        let the_chain = first.chain(second);
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
