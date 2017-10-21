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

pub type GenApply<F, C> = GenMapReturn<GenAll<F, C>, ApplyFn<<F as Returns>::Return, <C as Returns>::Return>>;

pub trait Apply
{
    fn apply<C>(self, c: C) -> GenApply<Self, C> where Self: Sized + Returns, C: Returns, Self::Return: FnOnce<(C::Return,)>
    {
        self.all(c).map_return(ApplyFn(PhantomData))
    }
}

impl<T> Apply for T { }

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
