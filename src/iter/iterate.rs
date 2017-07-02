use co::{Coroutine, CoResult};

pub struct CoIterate<C>(Option<C>);

impl<C> Iterator for CoIterate<C>
    where C: Coroutine<Continue = C>
{
    type Item = C::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(coro) => {
                match coro.next() {
                    CoResult::Yield(i, cnt) => {
                        self.0 = Some(cnt);
                        Some(i)
                    }
                    _ => None,
                }
            }
            None => None,
        }
    }
}

pub trait Iterate<C>: Sized {
    fn iterate(self) -> CoIterate<Self> {
        CoIterate(Some(self))
    }
}

impl<C> Iterate<C> for C where C: Coroutine {}

#[cfg(test)]
mod tests {
    use super::*;

    struct Counter<T> {
        i: T,
        lim: T,
    }

    impl Coroutine for Counter<i64> {
        type Yield = i64;
        type Return = &'static str;
        type Continue = Self;

        fn next(self) -> CoResult<Self> {
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

    impl Coroutine for InfiniteCounter<i64> {
        type Yield = i64;
        type Return = !;
        type Continue = Self;

        fn next(self) -> CoResult<Self> {
            CoResult::Yield(self.i, InfiniteCounter { i: self.i + 1 })
        }
    }

    #[test]
    fn iterate_over_coroutine() {
        let mut cnt = 3;
        let lim = 10;
        let bart = Counter::<i64> { i: cnt, lim: lim };
        for i in bart.iterate() {
            assert_eq!(i, cnt);
            cnt += 1;
        }
        assert_eq!(cnt, lim);
    }
}
