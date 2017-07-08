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

pub trait MapReturn<F>: Sized {
    fn map_return(self, f: F) -> CoMapReturn<Self, F> {
        CoMapReturn(self, f)
    }
}

impl<C, F> MapReturn<F> for C
    where C: Coroutine,
          F: FnOnce<(C::Return,)>
{
}
