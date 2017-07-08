use co::{Coroutine, CoResult};

pub struct CoDone<U>(U);

impl<U> Coroutine for CoDone<U> {
    type Yield = !;
    type Return = U;

    fn next(self) -> CoResult<Self> {
        CoResult::Return(self.0)
    }
}

pub trait Done: Sized {
    fn done(self) -> CoDone<Self> {
        CoDone(self)
    }
}

impl<U> Done for U where U: Sized {}
