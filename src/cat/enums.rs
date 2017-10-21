use cat::sum::{Sum, Either};
use cat::{Iso, Sur, Inj};

pub enum Match<A, B> {
    Variant(A),
    Next(B),
}

impl<A, B> Sur<Either<A, B>> for Match<A, B> {
    fn sur(e: Either<A, B>) -> Self {
        match e {
            Either::Left(l) => Match::Variant(l),
            Either::Right(r) => Match::Next(r),
        }
    }
}

impl<A, B> Inj<Either<A, B>> for Match<A, B> {
    fn inj(self) -> Either<A, B> {
        match self {
            Match::Variant(v) => Either::Left(v),
            Match::Next(n) => Either::Right(n),
        }
    }
}

unsafe impl<A, B> Iso<Either<A, B>> for Match<A, B> {}

impl<A, B> Sum for Match<A, B> {
    type Left = A;
    type Right = B;
    type Output = Self;
}

pub trait Enum {
    type Head;
    type Tail: Enum;
    type Output: Iso<Either<Self::Head, Self::Tail>>;

    fn split(self) -> Self::Output;
}

#[macro_export]
macro_rules! enums {
    ($head:ty, $($tail:ty),+; $end:ty) => {
        $crate::cat::enums::Match<$head, enums![ $($tail),*; $end ]>
    };

    ($head:ty; $end:ty) => {
        $crate::cat::enums::Match<$head, $end>
    };

    ($($tail:ty),*) => {
        enums![ $($tail),*; ! ]
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn enum_once() {
        use cat::enums::Match::*;
        type Vars = enums![i32, &'static str, f64];
        let integer: Vars = Variant(42);
        let string = Next(Variant("Happy!"));
        let float = Next(Next(Variant(42.0)));

        let answers = ["I am an integer: 42", "I am Happy!", "I am exactly: 42.0"];
        for (item, answer) in [integer, string, float].into_iter().zip(answers.into_iter()) {
            let ans = match item {
                &Variant(i) => format!("I am an integer: {}", i),
                &Next(Variant(s)) => format!("I am {}", s),
                &Next(Next(Variant(f))) => format!("I am exactly: {:.1}", f),
            };
            assert_eq!(ans, *answer);
        }
    }
}
