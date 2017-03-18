use co::{Coroutine, CoResult};
use map::CoMap;

pub struct CoArgMap<C, F>(CoMap<C, F>);

pub trait ArgMap<F, Input, Output>
    where F: FnOnce(Input) -> Output,
          Self: Sized + Coroutine<Output>
{
    fn arg_map(self, f: F) -> CoArgMap<Self, F> {
        CoArgMap(CoMap { c: self, f: f })
    }
}

impl<C, F, Input, Output> ArgMap<F, Input, Output> for C
    where C: Sized + Coroutine<Output>,
          F: FnOnce(Input) -> Output
{
}

impl<C, F, Input, Output> Coroutine<Input> for CoArgMap<C, F>
    where F: FnMut(Input) -> Output,
          C: Sized + Coroutine<Output>
{
    type Yield = C::Yield;
    type Return = C::Return;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        let mut f = self.0.f;
        match self.0.c.next(f(i)) {
            CoResult::Yield(y, c) => CoResult::Yield(y, c.arg_map(f)),
            CoResult::Return(res) => CoResult::Return(res),
        }
    }
}
