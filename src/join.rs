use co::{Coroutine, CoResult};

pub enum CoJoin<C>
    where C: Coroutine
{
    Outer(C),
    Inner(C::Return),
}

impl<C> Coroutine for CoJoin<C>
    where C: Coroutine,
          C::Return: Coroutine,
          C::Yield: From<<C::Return as Coroutine>::Yield>
{
    type Yield = C::Yield;
    type Return = <C::Return as Coroutine>::Return;

    fn next(self) -> CoResult<Self> {
        match self {
            CoJoin::Outer(c) => {
                match c.next() {
                    CoResult::Yield(y, outer) => CoResult::Yield(y, outer.join()),
                    CoResult::Return(inner) => CoJoin::Inner(inner).next(),
                }
            }
            CoJoin::Inner(c) => {
                match c.next() {
                    CoResult::Yield(y, inner) => CoResult::Yield(y.into(), CoJoin::Inner(inner)),
                    CoResult::Return(result) => CoResult::Return(result),
                }
            }
        }
    }
}

pub trait Join: Sized {
    fn join(self) -> CoJoin<Self>
        where Self: Coroutine,
              Self::Return: Coroutine,
              Self::Yield: From<<Self::Return as Coroutine>::Yield>
    {
        CoJoin::Outer(self)
    }
}

impl<C> Join for C
    where C: Coroutine,
          C::Return: Coroutine,
          C::Yield: From<<C::Return as Coroutine>::Yield>
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::map::ret::MapReturn;

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

        let msg = "This is the end";
        let chain = first.map_return(|()| second.map_return(|()| msg)).join();
        let mut elem = 1;
        let message = each!(chain => i in {
            assert_eq!(i, elem);
            elem = if elem == 8 { 1 } else { elem + 1 };
        });
        assert_eq!(elem, 3);
        assert_eq!(message, msg);
    }
}
