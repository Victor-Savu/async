use gen::{Generator, GenResult};
use cat::sum::Either;

pub struct GenMapReturn<C, F>(C, F);

impl<C, F> Generator for GenMapReturn<C, F>
    where C: Generator,
          F: FnOnce<(C::Return,)>
{
    type Yield = C::Yield;
    type Return = F::Output;
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self.0.next().to_canonical() {
            Either::Left(s) => {
                let (y, c) = s.to_canonical();
                GenResult::Yield(y, c.map_return(self.1))
            }
            Either::Right(res) => GenResult::Return((self.1)(res)),
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
