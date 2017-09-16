use gen::{Generator, GenResult};

pub struct GenWrap<Iter>(Iter) where Iter: Iterator + Sized;

impl<Iter> Generator for GenWrap<Iter>
    where Iter: Iterator
{
    type Yield = Iter::Item;
    type Return = Iter;
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
