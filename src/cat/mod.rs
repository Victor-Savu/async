#[macro_use]
pub mod enums;
pub mod sum;
pub mod prod;

pub trait Inj<X> {
    fn inj(self) -> X;
}

pub trait Sur<X> {
    fn sur(x: X) -> Self;
}

impl<X> Sur<X> for X {
    fn sur(x: X) -> Self { x }
}

impl<X, T> Inj<X> for T where X: Sur<T> {
    fn inj(self) -> X {
        X::sur(self)
    }
}

pub unsafe trait Iso<X> where Self: Sur<X>, Self: Inj<X>  {}
unsafe impl<X> Iso<X> for X {}

impl Sur<(!, !)> for ! {
    fn sur(_: (!, !)) -> Self {
        unreachable!()
    }
}

impl Inj<(!, !)> for ! {
    fn inj(self) -> (!, !) {
        unreachable!()
    }
}

unsafe impl Iso<(!, !)> for ! {}
