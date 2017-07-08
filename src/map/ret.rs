use co::{Coroutine, CoResult};

pub struct CoMapReturn<C, F>(C, F);

impl<C, F> Coroutine for CoMapReturn<C, F>
    where C: Coroutine,
          F: FnOnce<(C::Return,)>
{
    type Yield = C::Yield;
    type Return = F::Output;

    fn next(self) -> CoResult<Self> {
        match self.0.next() {
            CoResult::Yield(y, c) => CoResult::Yield(y, c.map_return(self.1)),
            CoResult::Return(res) => CoResult::Return((self.1)(res)),
        }
    }
}

pub trait MapReturn
    where Self: Coroutine
{
    fn map_return<F>(self, f: F) -> CoMapReturn<Self, F>
        where F: FnOnce<(Self::Return,)>
    {
        CoMapReturn(self, f)
    }
}

impl<C> MapReturn for C where C: Coroutine {}
