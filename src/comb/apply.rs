use std::marker::PhantomData;

use gen::{Generator, GenResult};
use map::ret::{GenMapReturn, MapReturn};
use comb::all::{GenAll, All};

pub struct ApplyFn<F, I>(PhantomData<(F, I)>) where F: FnOnce<(I,)>;

impl<F, I> FnOnce<((F, I),)> for ApplyFn<F, I>
    where F: FnOnce<(I,)>
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, ((f, i),): ((F, I),)) -> Self::Output {
        f(i)
    }
}

pub struct GenApply<F, C>(GenMapReturn<GenAll<F, C>, ApplyFn<F::Return, C::Return>>)
    where C: Generator,
          F: Generator<Yield = C::Yield>,
          F::Return: FnOnce<(C::Return,)>;

impl<F, C> GenApply<F, C>
    where C: Generator,
          F: Generator<Yield = C::Yield>,
          F::Return: FnOnce<(C::Return,)>
{
    fn new(functor: F, c: C) -> Self {
        GenApply(functor.all(c).map_return(ApplyFn(PhantomData)))
    }
}

impl<C, F> Generator for GenApply<F, C>
    where C: Generator,
          F: Generator<Yield = C::Yield>,
          F::Return: FnOnce<(C::Return,)>
{
    type Yield = C::Yield;
    type Return = <F::Return as FnOnce<(C::Return,)>>::Output;

    fn next(self) -> GenResult<Self> {
        match self.0.next() {
            GenResult::Yield(y, s) => GenResult::Yield(y, GenApply(s)),
            GenResult::Return(r) => GenResult::Return(r),
        }
    }
}

pub trait Apply<I>: Generator
    where Self::Return: FnOnce<(I,)>
{
    fn apply<C>(self, c: C) -> GenApply<Self, C>
        where C: Generator<Yield = Self::Yield, Return = I>
    {
        GenApply::new(self, c)
    }
}

impl<I, T> Apply<I> for T
    where T: Generator,
          T::Return: FnOnce<(I,)>
{
}

#[cfg(test)]
mod tests {

    use comb::done::Done;
    use comb::apply::Apply;

    #[test]
    fn apply() {
        let four = (|x| x + 1).done().apply(3.done());
        let res = each!(four);
        assert_eq!(res, 4);
    }
}
