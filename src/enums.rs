
pub enum Match<V, O: Enum> {
    Variant(V),
    Next(O),
}

pub trait Enum {
    type Variant;
    type Next: Enum;
}

impl Enum for ! {
    type Variant = !;
    type Next = !;
}

impl<V, E: Enum> Enum for Match<V, E> {
    type Variant = V;
    type Next = E;
}

#[cfg(test)]
mod tests {

    use super::{Match};
    #[test]
    fn enum_once() {
        type Vars = Match<i32,
                    Match<&'static str,
                    Match<f64,
                    !>>>;
        use self::Match::*;
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
