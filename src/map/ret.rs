use co::{Coroutine, CoResult};
use map::CoMap;

pub struct CoMapReturn<C, F>(CoMap<C, F>);

impl<C, F> Coroutine for CoMapReturn<C, F>
    where C: Coroutine<Continue = C>,
          F: FnOnce<(C::Return,)>
{
    type Yield = C::Yield;
    type Return = F::Output;
    type Continue = Self;

    fn next(self) -> CoResult<Self> {
        match self.0.c.next() {
            CoResult::Yield(y, c) => CoResult::Yield(y, c.map_return(self.0.f)),
            CoResult::Return(res) => CoResult::Return((self.0.f)(res)),
        }
    }
}

pub trait MapReturn<F>: Sized {
    fn map_return(self, f: F) -> CoMapReturn<Self, F> {
        CoMapReturn(CoMap { c: self, f: f })
    }
}

impl<C, F> MapReturn<F> for C
    where C: Coroutine,
          F: FnOnce<(C::Return,)>
{
}
