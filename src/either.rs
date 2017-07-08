use gen::{Generator, GenResult};

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


impl<F, L> Generator for Either<F, L>
    where F: Generator,
          L: Generator<Yield = F::Yield, Return = F::Return>
{
    type Yield = F::Yield;
    type Return = F::Return;

    fn next(self) -> GenResult<Self> {
        match self {
            Either::Former(f) => {
                match f.next() {
                    GenResult::Yield(y, f) => GenResult::Yield(y, Either::Former(f)),
                    GenResult::Return(r) => GenResult::Return(r),
                }
            }
            Either::Latter(l) => {
                match l.next() {
                    GenResult::Yield(y, l) => GenResult::Yield(y, Either::Latter(l)),
                    GenResult::Return(r) => GenResult::Return(r),
                }
            }
        }
    }
}
