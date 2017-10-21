use gen::{Generator, GenResult, Returns, Yields};

pub struct GenDone<U>(U);

impl<U> Yields for GenDone<U> {
    type Yield = !;
}

impl<U> Returns for GenDone<U> {
    type Return = U;
}

impl<U> Generator for GenDone<U> {
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        GenResult::Return(self.0)
    }
}

pub trait Done: Sized {
    fn done(self) -> GenDone<Self> {
        GenDone(self)
    }
}

impl<U> Done for U where U: Sized {}
