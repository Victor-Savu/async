use co::{Coroutine, CoResult};
use map::CoMap;

pub struct CoMapReturn<C, F>(CoMap<C, F>);

impl<C, F, Output> Coroutine for CoMapReturn<C, F>
    where C: Coroutine,
          F: FnOnce(C::Return) -> Output
{
    type Yield = C::Yield;
    type Return = Output;
    type Continue = CoMapReturn<C::Continue, F>;

    fn next(self) -> CoResult<Self> {
        match self.0.c.next() {
            CoResult::Yield(y, c) => CoResult::Yield(y, c.map_return(self.0.f)),
            CoResult::Return(res) => CoResult::Return((self.0.f)(res)),
        }
    }
}

pub trait MapReturn<F, Output>: Sized {
    fn map_return(self, f: F) -> CoMapReturn<Self, F> {
        CoMapReturn(CoMap { c: self, f: f })
    }
}

impl<C, F, Output> MapReturn<F, Output> for C
    where C: Coroutine,
          F: FnOnce(C::Return) -> Output
{
}
