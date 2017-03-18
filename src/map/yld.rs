use co::{Coroutine, CoResult};
use map::CoMap;

pub struct CoMapYield<C, F>(CoMap<C, F>);

impl<C, F, Input, Output> Coroutine<Input> for CoMapYield<C, F>
    where F: FnMut(C::Yield) -> Output,
          C: Sized + Coroutine<Input>
{
    type Yield = Output;
    type Return = C::Return;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        let mut f = self.0.f;
        match self.0.c.next(i) {
            CoResult::Yield(y, c) => CoResult::Yield(f(y), c.map_yield(f)),
            CoResult::Return(res) => CoResult::Return(res),
        }
    }
}

pub trait MapYield<F, Input, Output>
    where Self: Sized + Coroutine<Input>,
          F: FnOnce(Self::Yield) -> Output
{
    fn map_yield(self, f: F) -> CoMapYield<Self, F> {
        CoMapYield(CoMap { c: self, f: f })
    }
}

impl<C, F, Input, Output> MapYield<F, Input, Output> for C
    where C: Sized + Coroutine<Input>,
          F: FnOnce(C::Yield) -> Output
{
}
