use std;
use co::{Coroutine, CoResult};

pub struct CoIterator<C>(Option<C>);

trait Iterator<C> where C: Coroutine<()>, Self: Sized
{
    fn iter(self) -> CoIterator<Self> {
        CoIterator(Some(self))
    }
}

impl<C> Iterator<C> for C where C: Coroutine<()>
{
}

impl<C> std::iter::Iterator for CoIterator<C> where C: Coroutine<()> {
    type Item = C::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(coro) => match coro.next(()) {
                CoResult::Yield(i, cnt) => { self.0 = Some(cnt); Some(i) },
                _ => None
            },
            None => None
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

    impl Coroutine<()> for Counter<i64> {
        type Yield = i64;
        type Return = &'static str;

        fn next(self, _: ()) -> CoResult<Self::Yield, Self, Self::Return> {
            if self.i < self.lim {
                CoResult::Yield(self.i,
                                Counter {
                                    i: self.i + 1,
                                    lim: self.lim,
                                })
            } else {
                CoResult::Return("I'm done!")
            }
        }
    }

    struct InfiniteCounter<T> {
        i: T,
    }

    impl Coroutine<()> for InfiniteCounter<i64> {
        type Yield = i64;
        type Return = !;

        fn next(self, _: ()) -> CoResult<Self::Yield, Self, Self::Return> {
            CoResult::Yield(self.i, InfiniteCounter { i: self.i + 1 })
        }
    }

    #[test]
    fn iterate_over_coroutine()
    {
        let mut cnt = 3;
        let lim = 10;
        let bart = Counter::<i64> {i: cnt, lim: lim};
        for i in  bart.iter() {
            assert_eq!(i, cnt);
            cnt += 1;
        }
        assert_eq!(cnt, lim);
    }
}
