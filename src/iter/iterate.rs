use co::{Coroutine, CoResult};

pub struct CoIterate<C>(Option<C>);

impl<C> Iterator for CoIterate<C>
    where C: Coroutine
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

pub trait Iterate
    where Self: Sized
{
    fn iterate(self) -> CoIterate<Self> {
        CoIterate(Some(self))
    }
}

impl<C> Iterate for C where C: Coroutine {}
