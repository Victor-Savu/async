use co::{Coroutine, CoResult};

pub enum Either<F, L> {
    Former(F),
    Latter(L),
}


impl<F> Either<F, F> {
    pub fn collapse(self) -> F {
        match self {
            Either::Former(f) => f,
            Either::Latter(l) => l,
        }
    }
}


impl<F, L> Coroutine for Either<F, L>
    where F: Coroutine<Continue = F>,
          L: Coroutine<Yield = F::Yield, Return = F::Return, Continue = L>
{
    type Yield = F::Yield;
    type Return = F::Return;
    type Continue = Self;

    fn next(self) -> CoResult<Self> {
        match self {
            Either::Former(f) => {
                match f.next() {
                    CoResult::Yield(y, f) => CoResult::Yield(y, Either::Former(f)),
                    CoResult::Return(r) => CoResult::Return(r),
                }
            }
            Either::Latter(l) => {
                match l.next() {
                    CoResult::Yield(y, l) => CoResult::Yield(y, Either::Latter(l)),
                    CoResult::Return(r) => CoResult::Return(r),
                }
            }
        }
    }
}
