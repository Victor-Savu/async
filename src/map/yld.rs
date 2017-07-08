use gen::{Generator, GenResult};

pub struct GenMapYield<C, F>(C, F);

impl<C, F> Generator for GenMapYield<C, F>
    where C: Generator,
          F: FnMut<(C::Yield,)>
{
    type Yield = F::Output;
    type Return = C::Return;

    fn next(self) -> GenResult<Self> {
        let mut f = self.1;
        match self.0.next() {
            GenResult::Yield(y, c) => GenResult::Yield(f(y), c.map_yield(f)),
            GenResult::Return(res) => GenResult::Return(res),
        }
    }
}

pub trait MapYield
    where Self: Generator
{
    fn map_yield<F>(self, f: F) -> GenMapYield<Self, F> where 
          F: FnMut<(Self::Yield,)>
    {
        GenMapYield(self, f)
    }
}

impl<C> MapYield for C
    where C: Generator
{
}
