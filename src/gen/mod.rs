#[macro_use]
pub mod each;
pub mod comb;
pub mod either;
pub mod iter;
pub mod map;

use std::ops::RangeFrom;
use fsm::{State, ContinuationList, Continuation, StateTransition};
use cat::sum::{Either, Sum};
use cat::{Iso, Sur, Inj};

pub trait Yields {
    type Yield;
}

pub trait Returns {
    type Return;
}

pub trait Generator: Sized + Yields + Returns {
    type Transition: Iso<Either<(Self::Yield, Self), Self::Return>>;

    fn next(self) -> Self::Transition;
}

pub enum GenResult<Coro>
    where Coro: Yields + Returns
{
    Yield(Coro::Yield, Coro),
    Return(Coro::Return),
}

impl<Coro> Sum for GenResult<Coro>
    where Coro: Yields + Returns
{
    type Left = (Coro::Yield, Coro);
    type Right = Coro::Return;
    type Output = Self;
}

impl<Coro> Sur<Either<(Coro::Yield, Coro), Coro::Return>> for GenResult<Coro>
    where Coro: Yields + Returns
{
    fn sur(e: Either<(Coro::Yield, Coro), Coro::Return>) -> Self {
        match e {
            Either::Left((y, c)) => GenResult::Yield(y, c),
            Either::Right(r) => GenResult::Return(r),
        }
    }
}

impl<Coro> Inj<Either<(Coro::Yield, Coro), Coro::Return>> for GenResult<Coro>
    where Coro: Yields + Returns
{
    fn inj(self) -> Either<(Coro::Yield, Coro), Coro::Return> {
        match self {
            GenResult::Yield(y, c) => Either::Left((y, c)),
            GenResult::Return(r) => Either::Right(r),
        }
    }
}

unsafe impl<Coro> Iso<Either<(Coro::Yield, Coro), Coro::Return>> for GenResult<Coro>
    where Coro: Yields + Returns
{
}

pub struct GenState<S>(S);

impl<S> Yields for GenState<S>
    where S: State<Input = ()>,
          <S::Transition as StateTransition>::Continuation: ContinuationList<Tail = !>,
          <<S::Transition as StateTransition>::Continuation as ContinuationList>::Head: Continuation<Continue = S>,
          <<S::Transition as StateTransition>::Continuation as ContinuationList>::Output: Iso<Either<<<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Output, !>>,
          <<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Output: Iso<(<<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Emit, S)>
{
    type Yield = <<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Emit;
}

impl<S> Returns for GenState<S>
    where S: State<Input = ()>,
          <S::Transition as StateTransition>::Continuation: ContinuationList<Tail = !>,
          <<S::Transition as StateTransition>::Continuation as ContinuationList>::Head: Continuation<Continue = S>,
          <<S::Transition as StateTransition>::Continuation as ContinuationList>::Output: Iso<Either<<<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Output, !>>,
          <<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Output: Iso<(<<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Emit, S)>
{
    type Return = <S::Transition as StateTransition>::Exit;
}

impl<S> Generator for GenState<S>
    where S: State<Input = ()>,
          <S::Transition as StateTransition>::Continuation: ContinuationList<Tail = !>,
          <<S::Transition as StateTransition>::Continuation as ContinuationList>::Head: Continuation<Continue = S>,
          <<S::Transition as StateTransition>::Continuation as ContinuationList>::Output: Iso<Either<<<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Output, !>>,
          <<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Output: Iso<(<<<S::Transition as StateTransition>::Continuation as ContinuationList>::Head as Continuation>::Emit, S)>
{
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self.0.send(()).inj() {
            Either::Left(cont) => {
                let ei = cont.inj();
                let (y, c) = match ei {
                        Either::Left(l) => l,
                    }
                    .inj();
                GenResult::Yield(y, GenState(c))
            }
            Either::Right(ret) => GenResult::Return(ret),
        }
    }
}

impl<Idx> Yields for RangeFrom<Idx>
    where Self: Iterator
{
    type Yield = <Self as Iterator>::Item;
}

impl<Idx> Returns for RangeFrom<Idx>
    where Self: Iterator
{
    type Return = !;
}

impl<Idx> Generator for RangeFrom<Idx>
    where Self: Iterator
{
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        let mut x = self;
        loop {
            if let Some(y) = Iterator::next(&mut x) {
                break GenResult::Yield(y, x);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn range_from() {
        fn foo(q: i64) -> (usize, Vec<i64>) {
            let mut x = q;
            let mut v = vec![];
            each!(1.. => steps in {
                v.push(x);
                x = if x == 1 {
                    return (steps, v)
                } else if x % 2 == 0 {
                    x / 2
                } else {
                    x * 3 + 1
                };
            })
        }

        let (steps, values) = foo(10);

        assert_eq!(steps, values.len());
        assert_eq!(values, [10, 5, 16, 8, 4, 2, 1]);
    }
}
