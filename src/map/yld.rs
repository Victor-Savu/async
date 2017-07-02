use co::{Coroutine, CoResult};
use map::CoMap;

pub struct CoMapYield<C, F>(CoMap<C, F>);

impl<C, F, Output> Coroutine for CoMapYield<C, F>
    where C: Coroutine<Continue = C>,
          F: FnMut(C::Yield) -> Output
{
    type Yield = Output;
    type Return = C::Return;
    type Continue = CoMapYield<C, F>;

    fn next(self) -> CoResult<Self> {
        let mut f = self.0.f;
        match self.0.c.next() {
            CoResult::Yield(y, c) => CoResult::Yield(f(y), c.map_yield(f)),
            CoResult::Return(res) => CoResult::Return(res),
        }
    }
}

pub trait MapYield<F, Output>: Sized {
    fn map_yield(self, f: F) -> CoMapYield<Self, F> {
        CoMapYield(CoMap { c: self, f: f })
    }
}

impl<C, F, Output> MapYield<F, Output> for C
    where C: Coroutine,
          F: FnOnce(C::Yield) -> Output
{
}
