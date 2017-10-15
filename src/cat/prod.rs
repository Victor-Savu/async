use cat::Iso;

pub trait Prod {
    type Left;
    type Right;
    type Output: Iso<(Self::Left, Self::Right)>;
}

impl<A, B> Prod for (A, B) {
    type Left = A;
    type Right = B;
    type Output = Self;
}

impl Prod for ! {
    type Left = !;
    type Right = !;
    type Output = !;
}
