use co::{Coroutine, CoResult};

pub struct CoWrap<T>(T);

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

pub trait Wrap<I>: Sized {
    fn wrap(self) -> CoWrap<Self> {
        CoWrap(self)
    }
}

impl<I> Wrap<I> for I where I: Iterator {}
