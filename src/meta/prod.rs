use meta::list::TypeList;

pub trait Prod: TypeList {
    fn split(self) -> (Self::Head, Self::Tail);
}

impl Prod for () {
    fn split(self) -> (Self::Head, Self::Tail) {
        ((), ())
    }
}

impl Prod for ! {
    fn split(self) -> (Self::Head, Self::Tail) {
        unreachable!()
    }
}

impl<A, B> Prod for (A, B)
    where B: Prod
{
    fn split(self) -> (Self::Head, Self::Tail) {
        self
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn split() {
        use super::Prod;

        let x = (1, ("Hello", ()));
        let (a, (b, ())) = x.split();
        assert_eq!(a, 1);
        assert_eq!(b, "Hello");
    }
}
