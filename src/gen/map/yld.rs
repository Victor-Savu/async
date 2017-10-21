use gen::{Generator, GenResult, Yields, Returns};
use cat::sum::Either;
use cat::Inj;


pub struct GenMapYield<C, F>(C, F);

impl<C, F> Yields for GenMapYield<C, F>
    where C: Yields,
          F: FnOnce<(C::Yield,)>
{
    type Yield = F::Output;
}

impl<C, F> Returns for GenMapYield<C, F>
    where C: Returns
{
    type Return = C::Return;
}

impl<C, F> Generator for GenMapYield<C, F>
    where C: Generator,
          F: FnMut<(C::Yield,)>
{
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        let mut f = self.1;
        match self.0.next().inj() {
            Either::Left(s) => {
                let (y, c) = s.inj();
                GenResult::Yield(f(y), c.map_yield(f))
            }
            Either::Right(res) => GenResult::Return(res),
        }
    }
}

pub trait MapYield
    where Self: Generator
{
    fn map_yield<F>(self, f: F) -> GenMapYield<Self, F>
        where F: FnMut<(Self::Yield,)>
    {
        GenMapYield(self, f)
    }
}

impl<C> MapYield for C where C: Generator {}
