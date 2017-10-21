use gen::{Generator, GenResult, Yields, Returns};
use cat::sum::Either;
use cat::Inj;

pub struct GenMapReturn<C, F>(C, F);

impl<C, F> Yields for GenMapReturn<C, F>
    where C: Yields
{
    type Yield = C::Yield;
}

impl<C, F> Returns for GenMapReturn<C, F>
    where C: Returns,
          F: FnOnce<(C::Return,)>
{
    type Return = F::Output;
}

impl<C, F> Generator for GenMapReturn<C, F>
    where C: Returns,
          F: FnOnce<(C::Return,)>
{
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self.0.next().inj() {
            Either::Left(s) => {
                let (y, c) = s.inj();
                GenResult::Yield(y, c.map_return(self.1))
            }
            Either::Right(res) => GenResult::Return((self.1)(res)),
        }
    }
}

pub trait MapReturn
    where Self: Returns
{
    fn map_return<F>(self, f: F) -> GenMapReturn<Self, F>
        where F: FnOnce<(Self::Return,)>
    {
        GenMapReturn(self, f)
    }
}

impl<C> MapReturn for C {}
