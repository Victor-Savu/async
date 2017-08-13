#![macro_use]

use meta::sum::{Sum, Left, Either};

pub enum Match<A, B> {
    Variant(A),
    Next(B),
}

impl<A, B> Sum for Match<A, B> {
    type Left = A;
    type Right = B;

    fn to_canonical(self) -> Either<Self::Left, Self::Right> {
        match self {
            Match::Variant(var) => Either::Left(var),
            Match::Next(next) => Either::Right(next),
        }
    }
}
pub trait Enum {
    type Head;
    type Tail: Enum;
    type Output: Sum<Left = Self::Head, Right = Self::Tail>;

    fn split(self) -> Self::Output;
}

impl Enum for ! {
    type Head = !;
    type Tail = !;
    type Output = !;

    fn split(self) -> Self::Output {
        unreachable!()
    }
}

impl<A, B> Enum for Match<A, B>
    where B: Enum
{
    type Head = A;
    type Tail = B;
    type Output = Self;

    fn split(self) -> Self::Output {
        self
    }
}

impl<A> Enum for (A,) {
    type Head = A;
    type Tail = !;
    type Output = Left<A>;

    fn split(self) -> Self::Output {
        Either::Left(self.0)
    }
}

#[macro_export]
macro_rules! enums {
    ($head:ty, $($tail:ty),+; $end:ty) => {
        $crate::meta::enums::Match<$head, enums![ $($tail),*; $end ]>
    };

    ($head:ty; $end:ty) => {
        $crate::meta::enums::Match<$head, $end>
    };

    ($($tail:ty),*) => {
        enums![ $($tail),*; ! ]
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn enum_once() {
        use meta::enums::Match::*;
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
