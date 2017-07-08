use gen::{Generator, GenResult};

pub struct GenIterate<C>(Option<C>);

impl<C> Iterator for GenIterate<C>
    where C: Generator
{
    type Item = C::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(coro) => {
                match coro.next() {
                    GenResult::Yield(i, cnt) => {
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
    fn iterate(self) -> GenIterate<Self> {
        GenIterate(Some(self))
    }
}

impl<C> Iterate for C where C: Generator {}
