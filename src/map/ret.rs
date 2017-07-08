use gen::{Generator, GenResult};

pub struct GenMapReturn<C, F>(C, F);

impl<C, F> Generator for GenMapReturn<C, F>
    where C: Generator,
          F: FnOnce<(C::Return,)>
{
    type Yield = C::Yield;
    type Return = F::Output;

    fn next(self) -> GenResult<Self> {
        match self.0.next() {
            GenResult::Yield(y, c) => GenResult::Yield(y, c.map_return(self.1)),
            GenResult::Return(res) => GenResult::Return((self.1)(res)),
        }
    }
}

pub trait MapReturn
    where Self: Generator
{
    fn map_return<F>(self, f: F) -> GenMapReturn<Self, F>
        where F: FnOnce<(Self::Return,)>
    {
        GenMapReturn(self, f)
    }
}

impl<C> MapReturn for C where C: Generator {}
