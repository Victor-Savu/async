pub trait Sum {
    type Left;
    type Right;

    fn to_canonical(self) -> Either<Self::Left, Self::Right>;
}

impl Sum for ! {
    type Left = !;
    type Right = !;

    fn to_canonical(self) -> Either<Self::Left, Self::Right> {
        unreachable!()
    }
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Sum for Either<L, R> {
    type Left = L;
    type Right = R;

    fn to_canonical(self) -> Either<Self::Left, Self::Right> {
        self
    }
}

pub type Left<A> = Either<A, !>;

pub type Right<A> = Either<!, A>;
