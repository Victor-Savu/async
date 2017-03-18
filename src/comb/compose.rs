use co::{Coroutine, CoResult};

pub struct CoCompose<Left, Right> {
    left: Left,
    right: Right,
}

pub enum Which<L, R> {
    Left(L),
    Right(R),
}

impl<Input, Left, Right> Coroutine<Input> for CoCompose<Left, Right>
    where Left: Coroutine<Right::Yield>,
          Right: Coroutine<Input>
{
    type Yield = Left::Yield;
    type Return = Which<(Left::Return, Right), (Left, Right::Return)>;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        match self.right.next(i) {
            CoResult::Yield(l, right) => {
                match self.left.next(l) {
                    CoResult::Yield(y, left) => CoResult::Yield(y, left.compose(right)),
                    CoResult::Return(ret) => CoResult::Return(Which::Left((ret, right))),
                }
            }
            CoResult::Return(ret) => CoResult::Return(Which::Right((self.left, ret))),
        }
    }
}

pub trait Compose<Input, Left, Right>: Sized
{
    fn compose(self, r: Right) -> CoCompose<Self, Right> {
        CoCompose {
            left: self,
            right: r,
        }
    }
}

impl<Input, Left, Right> Compose<Input, Left, Right> for Left
    where Left: Coroutine<Right::Yield>,
          Right: Coroutine<Input>
{
}
