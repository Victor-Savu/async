/*
use map::ret::MapReturn;
use co::Coroutine;
use join::Join;


pub trait Chain {
    fn chain<L>(self, l: L) -> impl Chain
        where L: Coroutine,
              <Self as Coroutine>::Yield: From<L::Yield>;
}

impl<F> Chain for F
    where F: Coroutine
{
    fn chain<L>(self, l: L) -> impl Chain
        where L: Coroutine,
              <Self as Coroutine>::Yield: From<L::Yield>
    {
        self.map_return(|res_f| l.map_return(|res_l| (res_f, res_l))).join()
    }
}
pub fn chain<F: Coroutine<Continue=F>, L: Coroutine<Continue=L, Yield=F::Yield>>(f: F, l: L) -> impl Coroutine<Yield=F::Yield, Return=(F::Return, L::Return)>
{
    f.map_return(move |res_f| l.map_return(move |res_l| (res_f, res_l))).join()
}
*/

#[cfg(test)]
mod tests {
    // use super::*;
    use co::CoResult;
    use map::ret::MapReturn;
    use co::Coroutine;
    use join::Join;

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

        // let the_chain = chain(first, second);
        let the_chain = first.map_return(move |res_f| second.map_return(move |res_l| (res_f, res_l))).join();
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
