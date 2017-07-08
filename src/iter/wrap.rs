use co::{Coroutine, CoResult};

pub struct CoWrap<Iter>(Iter) where Iter: Iterator + Sized;

impl<Iter> Coroutine for CoWrap<Iter>
    where Iter: Iterator
{
    type Yield = Iter::Item;
    type Return = Iter;

    fn next(self) -> CoResult<Self> {
        let mut i = self.0;
        match i.next() {
            Some(item) => CoResult::Yield(item, CoWrap(i)),
            _ => CoResult::Return(i),
        }
    }
}

pub trait Wrap
    where Self: Iterator + Sized
{
    fn wrap(self) -> CoWrap<Self> {
        CoWrap(self)
    }
}

impl<I> Wrap for I where I: Iterator {}
