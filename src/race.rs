use co::{Coroutine, CoResult};
use either::Either;


pub struct CoRace<F, L>(Either<(F, L), (F, L)>)
    where F: Coroutine,
          L: Coroutine<Yield = F::Yield>;


impl<F, L> Coroutine for CoRace<F, L>
    where F: Coroutine<Continue = F>,
          L: Coroutine<Yield = F::Yield, Continue = L>
{
    type Yield = F::Yield;
    type Return = Either<(F::Return, L), (F, L::Return)>;
    type Continue = Self;

    fn next(self) -> CoResult<Self> {
        match self.0 {
            Either::Former((f, l)) => {
                match f.next() {
                    CoResult::Yield(y, f) => CoResult::Yield(y, CoRace(Either::Latter((f, l)))),
                    CoResult::Return(f) => CoResult::Return(Either::Former((f, l))),
                }
            }
            Either::Latter((f, l)) => {
                match l.next() {
                    CoResult::Yield(y, l) => CoResult::Yield(y, CoRace(Either::Former((f, l)))),
                    CoResult::Return(l) => CoResult::Return(Either::Latter((f, l))),
                }
            }
        }
    }
}

pub trait Race
    where Self: Coroutine
{
    fn race<L>(self, l: L) -> CoRace<Self, L>
        where L: Coroutine<Yield = Self::Yield>
    {
        CoRace(Either::Former((self, l)))
    }
}

impl<C> Race for C where C: Coroutine {}
