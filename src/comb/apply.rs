use std::marker::PhantomData;

use co::{Coroutine, CoResult};
use map::ret::{CoMapReturn, MapReturn};
use comb::all::{CoAll, All};

pub struct ApplyFn<F, I>(PhantomData<(F, I)>) where F: FnOnce<(I,)>;

impl<F, I> FnOnce<((F, I),)> for ApplyFn<F, I>
    where F: FnOnce<(I,)>
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, ((f, i),): ((F, I),)) -> Self::Output {
        f(i)
    }
}

pub struct CoApply<F, C>(CoMapReturn<CoAll<F, C>, ApplyFn<F::Return, C::Return>>)
    where C: Coroutine,
          F: Coroutine<Yield = C::Yield>,
          F::Return: FnOnce<(C::Return,)>;

impl<F, C> CoApply<F, C>
    where C: Coroutine,
          F: Coroutine<Yield = C::Yield>,
          F::Return: FnOnce<(C::Return,)>
{
    fn new(functor: F, c: C) -> Self
    {
        CoApply(functor.all(c).map_return(ApplyFn(PhantomData)))
    }
}

impl<C, F> Coroutine for CoApply<F, C>
    where C: Coroutine,
          F: Coroutine<Yield = C::Yield>,
          F::Return: FnOnce<(C::Return,)>
{
    type Yield = C::Yield;
    type Return = <F::Return as FnOnce<(C::Return,)>>::Output;

    fn next(self) -> CoResult<Self> {
        match self.0.next() {
            CoResult::Yield(y, s) => CoResult::Yield(y, CoApply(s)),
            CoResult::Return(r) => CoResult::Return(r),
        }
    }
}

pub trait Apply<I>: Coroutine
    where Self::Return: FnOnce<(I,)>
{
    fn apply<C>(self, c: C) -> CoApply<Self, C>
        where C: Coroutine<Yield = Self::Yield, Return = I>
    {
        CoApply::new(self, c)
    }
}

impl<I, T> Apply<I> for T
    where T: Coroutine,
          T::Return: FnOnce<(I,)>
{
}
