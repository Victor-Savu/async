
pub trait Prod {
    type Head;
    type Tail: Prod;

    fn split(self) -> (Self::Head, Self::Tail);
}

impl Prod for ! {
    type Head = !;
    type Tail = !;

    fn split(self) -> (Self::Head, Self::Tail) {
        unreachable!()
    }
}

impl Prod for () {
    type Head = ();
    type Tail = ();

    fn split(self) -> (Self::Head, Self::Tail) {
        ((), ())
    }
}

impl<A> Prod for (A,)
{
    type Head = A;
    type Tail = ();

    fn split(self) -> (Self::Head, Self::Tail) {
        (self.0, ())
    }
}

impl<A, B> Prod for (A, B)
    where B: Prod
{
    type Head = A;
    type Tail = B;

    fn split(self) -> (Self::Head, Self::Tail) {
        self
    }
}

pub trait Pair<H, T> {
    type ProdTail: Prod<Head=T, Tail=()>;
    type Output: Prod<Head=H, Tail=Self::ProdTail>;
}

impl<H, T> Pair<H, T> for (H, T) {
    type ProdTail = (T,);
    type Output = (H, Self::ProdTail);
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
