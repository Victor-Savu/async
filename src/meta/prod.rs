pub trait Prod {
    type Left;
    type Right;

    fn to_canonical(self) -> (Self::Left, Self::Right);
}

impl<A, B> Prod for (A, B) {
    type Left = A;
    type Right = B;

    fn to_canonical(self) -> (Self::Left, Self::Right) {
        self
    }
}

impl Prod for ! {
    type Left = !;
    type Right = !;

    fn to_canonical(self) -> (Self::Left, Self::Right) {
        unreachable!()
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn split() {
        use super::Prod;

        let x = (1, ("Hello", ()));
        let (a, (b, ())) = x.to_canonical();
        assert_eq!(a, 1);
        assert_eq!(b, "Hello");
    }
}
