#![macro_use]

use meta::list::List;

pub enum Match<A, B> {
    Variant(A),
    Next(B),
}

pub trait Enum: List {
    fn split(self) -> Match<Self::Head, Self::Next>;
}

impl Enum for ! {
    fn split(self) -> Match<Self::Head, Self::Next> {
        unreachable!()
    }
}

impl<A, B: Enum> List for Match<A, B> {
    type Head = A;
    type Next = B;
}

impl<A, B> Enum for Match<A, B> where B: Enum {
    fn split(self) -> Match<Self::Head, Self::Next> {
        self
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
