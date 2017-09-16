use gen::{Generator, GenResult};

pub struct GenDone<U>(U);

impl<U> Generator for GenDone<U> {
    type Yield = !;
    type Return = U;
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
