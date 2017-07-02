use std::marker::PhantomData;

use co::{Coroutine, CoResult};
use either::Either;
use comb::race::{CoRace, Race};
use comb::chain::{CoChain, Chain};
use map::ret::{CoMapReturn, MapReturn};

pub struct Prepend<F>(F);

impl<F> Prepend<F> {
    pub fn new(f: F) -> Self {
        Prepend(f)
    }
}

impl<I, F> FnOnce<(I,)> for Prepend<F> {
    type Output = (F, I);

    extern "rust-call" fn call_once(self, (i,): (I,)) -> Self::Output {
        (self.0, i)
    }
}

pub struct Append<F>(F);

impl<F> Append<F> {
    pub fn new(f: F) -> Self {
        Append(f)
    }
}

impl<I, F> FnOnce<(I,)> for Append<F> {
    type Output = (I, F);

    extern "rust-call" fn call_once(self, (i,): (I,)) -> Self::Output {
        (i, self.0)
    }
}

pub struct ContinueRemaining<F, L>(PhantomData<(F, L)>)
    where F: Coroutine<Continue = F>,
          L: Coroutine<Yield = F::Yield, Continue = L>;

impl<F: Coroutine<Continue = F>, L: Coroutine<Yield = F::Yield, Continue = L>>
    ContinueRemaining<F, L> {
    fn new() -> Self {
        ContinueRemaining(PhantomData)
    }
}

impl<F, L> FnOnce<(Either<(F::Return, L), (F, L::Return)>,)> for ContinueRemaining<F, L>
    where F: Coroutine<Continue = F>,
          L: Coroutine<Yield = F::Yield, Continue = L>
{
    type Output = Either<CoMapReturn<L, Prepend<F::Return>>, CoMapReturn<F, Append<L::Return>>>;

    extern "rust-call" fn call_once(self,
                                    (results,): (Either<(F::Return, L), (F, L::Return)>,))
                                    -> Self::Output {
        match results {
            Either::Former((f, l)) => Either::Former(l.map_return(Prepend::new(f))),
            Either::Latter((f, l)) => Either::Latter(f.map_return(Append::new(l))),
        }
    }
}

pub struct CoAll<F, L>(CoChain<CoRace<F, L>, ContinueRemaining<F, L>>)
    where F: Coroutine<Continue = F>,
          L: Coroutine<Yield = F::Yield, Continue = L>;

impl<F, L> Coroutine for CoAll<F, L>
    where F: Coroutine<Continue = F>,
          L: Coroutine<Yield = F::Yield, Continue = L>
{
    type Yield = F::Yield;
    type Return = (F::Return, L::Return);
    type Continue = Self;

    fn next(self) -> CoResult<Self> {
        match self.0.next() {
            CoResult::Yield(y, s) => CoResult::Yield(y, CoAll(s)),
            CoResult::Return(r) => CoResult::Return(r),
        }
    }
}

pub trait All: Coroutine<Continue = Self> {
    fn all<L>(self, l: L) -> CoAll<Self, L> where L: Coroutine<Yield = Self::Yield, Continue = L>;
}

impl<F> All for F
    where F: Coroutine<Continue = F>
{
    fn all<L>(self, l: L) -> CoAll<Self, L>
        where L: Coroutine<Yield = Self::Yield, Continue = L>
    {
        CoAll(self.race(l).chain(ContinueRemaining::new()))
    }
}
