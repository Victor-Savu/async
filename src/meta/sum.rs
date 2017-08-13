use meta::matches::Match;

pub trait Sum {
    type Left;
    type Right;

    fn to_canonical(self) -> Match<Self::Left, Self::Right>;
}

impl Sum for ! {
    type Left = !;
    type Right = !;

    fn to_canonical(self) -> Match<Self::Left, Self::Right> {
        unreachable!()
    }
}

impl<A, B> Sum for Match<A, B>
{
    type Left = A;
    type Right = B;

    fn to_canonical(self) -> Match<Self::Left, Self::Right> {
        self
    }
}

impl<A> Sum for (A,)
{
    type Left = A;
    type Right = !;

    fn to_canonical(self) -> Match<Self::Left, Self::Right> {
        Match::Variant(self.0)
    }
}
