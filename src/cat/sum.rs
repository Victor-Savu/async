use cat::{Iso, Sur, Inj};

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl Sur<Either<!, !>> for ! {
    fn sur(_: Either<!, !>) -> ! {
        unreachable!()
    }
}

impl Inj<Either<!, !>> for ! {
    fn inj(self) -> Either<!, !> {
        unreachable!()
    }
}

unsafe impl Iso<Either<!, !>> for ! {}

pub type Left<A> = Either<A, !>;

pub type Right<A> = Either<!, A>;

pub trait Sum {
    type Left;
    type Right;
    type Output: Iso<Either<Self::Left, Self::Right>>;
}

impl Sum for ! {
    type Left = !;
    type Right = !;
    type Output = !;
}

impl<L, R> Sum for Either<L, R> {
    type Left = L;
    type Right = R;
    type Output = Self;
}
