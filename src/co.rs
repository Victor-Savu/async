use either::Either;

pub trait CoSuspend {
    type Yield;
    type Continuation: Coroutine;
    type Suspension: CoSuspend;

    fn get(self) -> Either<(Self::Yield, Self::Continuation), Self::Suspension>;
}

impl CoSuspend for ! {
    type Yield = !;
    type Continuation = !;
    type Suspension = !;

    fn get(self) -> Either<(Self::Yield, Self::Continuation), Self::Suspension> {
        unreachable!()
    }
}

pub enum CoResult<C>
    where C: Coroutine
{
    Suspend(C::Suspend),
    Return(C::Output),
}

pub trait Coroutine: Sized {
    type Input;
    type Output;
    type Suspend: CoSuspend;

    fn send(self, i: Self::Input) -> CoResult<Self>;
}

impl Coroutine for ! {
    type Input = !;
    type Suspend = !;
    type Output = !;

    fn send(self, _: Self::Input) -> CoResult<Self> {
        unreachable!()
    }
}
