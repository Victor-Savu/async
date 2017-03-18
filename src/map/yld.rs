use co::{Coroutine, CoResult};
use map::CoMap;

pub struct CoYieldMap<C, F>(CoMap<C, F>);

pub trait YieldMap<F, Input, Output>
    where Self: Sized + Coroutine<Input>,
          F: FnOnce(Self::Yield) -> Output
{
    fn yield_map(self, f: F) -> CoYieldMap<Self, F> {
        CoYieldMap(CoMap { c: self, f: f })
    }
}

impl<C, F, Input, Output> YieldMap<F, Input, Output> for C
    where C: Sized + Coroutine<Input>,
          F: FnOnce(C::Yield) -> Output
{
}

impl<C, F, Input, Output> Coroutine<Input> for CoYieldMap<C, F>
    where F: FnMut(C::Yield) -> Output,
          C: Sized + Coroutine<Input>
{
    type Yield = Output;
    type Return = C::Return;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        let mut f = self.0.f;
        match self.0.c.next(i) {
            CoResult::Yield(y, c) => CoResult::Yield(f(y), c.yield_map(f)),
            CoResult::Return(res) => CoResult::Return(res),
        }
    }
}
