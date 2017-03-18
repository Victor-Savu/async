use co::{Coroutine, CoResult};
use map::CoMap;

pub struct CoMapArg<C, F>(CoMap<C, F>);

impl<C, F, Input, Output> Coroutine<Input> for CoMapArg<C, F>
    where F: FnMut(Input) -> Output,
          C: Sized + Coroutine<Output>
{
    type Yield = C::Yield;
    type Return = C::Return;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        let mut f = self.0.f;
        match self.0.c.next(f(i)) {
            CoResult::Yield(y, c) => CoResult::Yield(y, c.map_arg(f)),
            CoResult::Return(res) => CoResult::Return(res),
        }
    }
}

pub trait MapArg<F, Input, Output>
    where F: FnOnce(Input) -> Output,
          Self: Sized + Coroutine<Output>
{
    fn map_arg(self, f: F) -> CoMapArg<Self, F> {
        CoMapArg(CoMap { c: self, f: f })
    }
}

impl<C, F, Input, Output> MapArg<F, Input, Output> for C
    where C: Sized + Coroutine<Output>,
          F: FnOnce(Input) -> Output
{
}
