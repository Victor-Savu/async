use co::{Coroutine, CoResult};
use map::CoMap;

pub struct CoMapReturn<C, F>(CoMap<C, F>);

impl<C, F, Input, Output> Coroutine<Input> for CoMapReturn<C, F>
    where F: FnOnce(C::Return) -> Output,
          C: Sized + Coroutine<Input>
{
    type Yield = C::Yield;
    type Return = Output;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        match self.0.c.next(i) {
            CoResult::Yield(y, c) => CoResult::Yield(y, c.map_return(self.0.f)),
            CoResult::Return(res) => CoResult::Return((self.0.f)(res)),
        }
    }
}

pub trait MapReturn<F, Input, Output>
    where F: FnOnce(Self::Return) -> Output,
          Self: Sized + Coroutine<Input>
{
    fn map_return(self, f: F) -> CoMapReturn<Self, F> {
        CoMapReturn(CoMap { c: self, f: f })
    }
}

impl<C, F, Input, Output> MapReturn<F, Input, Output> for C
    where C: Sized + Coroutine<Input>,
          F: FnOnce(C::Return) -> Output
{
}
