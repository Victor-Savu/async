use cat::sum::{Sum, Either};

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
