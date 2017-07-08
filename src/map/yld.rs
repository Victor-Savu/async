use co::{Coroutine, CoResult};

pub struct CoMapYield<C, F>(C, F);

impl<C, F> Coroutine for CoMapYield<C, F>
    where C: Coroutine,
          F: FnMut<(C::Yield,)>
{
    type Yield = F::Output;
    type Return = C::Return;

    fn next(self) -> CoResult<Self> {
        let mut f = self.1;
        match self.0.next() {
            CoResult::Yield(y, c) => CoResult::Yield(f(y), c.map_yield(f)),
            CoResult::Return(res) => CoResult::Return(res),
        }
    }
}

pub trait MapYield
    where Self: Coroutine
{
    fn map_yield<F>(self, f: F) -> CoMapYield<Self, F> where 
          F: FnMut<(Self::Yield,)>
    {
        CoMapYield(self, f)
    }
}

impl<C> MapYield for C
    where C: Coroutine
{
}
