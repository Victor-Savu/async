use gen::{Generator, GenResult};
use cat::sum::Either;
use cat::{Iso, Sur, Inj};


pub enum GenEither<F, L> {
    Former(F),
    Latter(L),
}

impl<F, L> Sur<Either<F, L>> for GenEither<F, L> {
    fn sur(e: Either<F, L>) -> Self {
        match e {
            Either::Left(l) => GenEither::Former(l),
            Either::Right(r) => GenEither::Latter(r),
        }
    }
}

impl<F, L> Inj<Either<F, L>> for GenEither<F, L> {
    fn inj(self) -> Either<F, L> {
        match self {
            GenEither::Former(f) => Either::Left(f),
            GenEither::Latter(l) => Either::Right(l),
        }
    }
}

unsafe impl<F, L> Iso<Either<F, L>> for GenEither<F, L>  {}

impl<F, L> Generator for GenEither<F, L>
    where F: Generator,
          L: Generator<Yield = F::Yield, Return = F::Return>,
          <L as Generator>::Transition: Iso<Either<(<F as Generator>::Yield, L), <F as Generator>::Return>>
{
    type Yield = F::Yield;
    type Return = F::Return;
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self {
            GenEither::Former(f) => {
                match f.next().inj() {
                    Either::Left(s) => {
                        let (y, f) = s.inj();
                        GenResult::Yield(y, GenEither::Former(f))
                    }
                    Either::Right(r) => GenResult::Return(r),
                }
            }
            GenEither::Latter(l) => {
                match l.next().inj() {
                    Either::Left(s) => {
                        let (y, l) = s.inj();
                        GenResult::Yield(y, GenEither::Latter(l))
                    }
                    Either::Right(r) => GenResult::Return(r),
                }
            }
        }
    }
}
