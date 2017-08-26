use gen::{Generator, GenResult};
use meta::sum::{self, Sum};
use meta::prod::Prod;

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
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self {
            Either::Former(f) => {
                match f.next().to_canonical() {
                    sum::Either::Left(s) => {
                        let (y, f) = s.to_canonical();
                        GenResult::Yield(y, Either::Former(f))
                    }
                    sum::Either::Right(r) => GenResult::Return(r),
                }
            }
            Either::Latter(l) => {
                match l.next().to_canonical() {
                    sum::Either::Left(s) => {
                        let (y, l) = s.to_canonical();
                        GenResult::Yield(y, Either::Latter(l))
                    }
                    sum::Either::Right(r) => GenResult::Return(r),
                }
            }
        }
    }
}
