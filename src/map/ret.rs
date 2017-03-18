use co::{Coroutine, CoResult};
use map::CoMap;

pub struct CoReturnMap<C, F>(CoMap<C, F>);

pub trait ReturnMap<F, Input, Output>
    where F: FnOnce(Self::Return) -> Output,
          Self: Sized + Coroutine<Input>
{
    fn return_map(self, f: F) -> CoReturnMap<Self, F> {
        CoReturnMap(CoMap { c: self, f: f })
    }
}

impl<C, F, Input, Output> ReturnMap<F, Input, Output> for C
    where C: Sized + Coroutine<Input>,
          F: FnOnce(C::Return) -> Output
{
}

impl<C, F, Input, Output> Coroutine<Input> for CoReturnMap<C, F>
    where F: FnOnce(C::Return) -> Output,
          C: Sized + Coroutine<Input>
{
    type Yield = C::Yield;
    type Return = Output;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        match self.0.c.next(i) {
            CoResult::Yield(y, c) => CoResult::Yield(y, c.return_map(self.0.f)),
            CoResult::Return(res) => CoResult::Return((self.0.f)(res)),
        }
    }
}
