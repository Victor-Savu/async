use std::marker::PhantomData;

use cat::Iso;
use cat::sum::Either;
use gen::{Generator, GenResult, Yields, Returns};
use gen::map::ret::{GenMapReturn, MapReturn};
use gen::comb::all::{GenAll, All};

pub struct ApplyFn<F, I>(PhantomData<(F, I)>);

impl<F, I> FnOnce<((F, I),)> for ApplyFn<F, I>
    where F: FnOnce<(I,)>
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, ((f, i),): ((F, I),)) -> Self::Output {
        f(i)
    }
}

pub struct GenApply<F, C>(GenMapReturn<GenAll<F, C>, ApplyFn<F::Return, C::Return>>)
    where C: Returns,
          F: Returns;

impl<F, C> GenApply<F, C>
{
    fn new(functor: F, c: C) -> Self {
        GenApply(functor.all(c).map_return(ApplyFn(PhantomData)))
    }
}

impl<C, F> Yields for GenApply<F, C>
    where C: Yields
{
    type Yield = C::Yield;
}

impl<C, F> Returns for GenApply<F, C>
    where F: Returns,
          C: Returns
{
    type Return = <F::Return as FnOnce<(C::Return,)>>::Output;
}

impl<C, F> Generator for GenApply<F, C>
{
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self.0.next() {
            GenResult::Yield(y, s) => GenResult::Yield(y, GenApply(s)),
            GenResult::Return(r) => GenResult::Return(r),
        }
    }
}

pub trait Apply<I>: Generator
{
    fn apply<C>(self, c: C) -> GenApply<Self, C>
    {
        GenApply::new(self, c)
    }
}

impl<I, T> Apply<I> for T
{
}

#[cfg(test)]
mod tests {

    use gen::comb::done::Done;
    use super::Apply;

    #[test]
    fn apply() {
        let four = (|x| x + 1).done().apply(3.done());
        let res = each!(four);
        assert_eq!(res, 4);
    }
}
