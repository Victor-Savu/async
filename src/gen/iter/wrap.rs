use gen::{Generator, GenResult, Yields, Returns};

pub struct GenWrap<Iter>(Iter) where Iter: Iterator + Sized;

impl<Iter> Yields for GenWrap<Iter>
    where Iter: Iterator
{
    type Yield = Iter::Item;
}

impl<Iter> Returns for GenWrap<Iter> {
    type Return = Iter;
}

impl<Iter> Generator for GenWrap<Iter>
    where Iter: Iterator
{
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        let mut i = self.0;
        match i.next() {
            Some(item) => GenResult::Yield(item, GenWrap(i)),
            _ => GenResult::Return(i),
        }
    }
}

pub trait Wrap
    where Self: Iterator + Sized
{
    fn wrap(self) -> GenWrap<Self> {
        GenWrap(self)
    }
}

impl<I> Wrap for I where I: Iterator {}
