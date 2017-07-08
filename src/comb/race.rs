use gen::{Generator, GenResult};
use either::Either;


pub struct GenRace<F, L>(Either<(F, L), (F, L)>)
    where F: Generator,
          L: Generator<Yield = F::Yield>;


impl<F, L> Generator for GenRace<F, L>
    where F: Generator,
          L: Generator<Yield = F::Yield>
{
    type Yield = F::Yield;
    type Return = Either<(F::Return, L), (F, L::Return)>;

    fn next(self) -> GenResult<Self> {
        match self.0 {
            Either::Former((f, l)) => {
                match f.next() {
                    GenResult::Yield(y, f) => GenResult::Yield(y, GenRace(Either::Latter((f, l)))),
                    GenResult::Return(f) => GenResult::Return(Either::Former((f, l))),
                }
            }
            Either::Latter((f, l)) => {
                match l.next() {
                    GenResult::Yield(y, l) => GenResult::Yield(y, GenRace(Either::Former((f, l)))),
                    GenResult::Return(l) => GenResult::Return(Either::Latter((f, l))),
                }
            }
        }
    }
}

pub trait Race
    where Self: Generator
{
    fn race<L>(self, l: L) -> GenRace<Self, L>
        where L: Generator<Yield = Self::Yield>
    {
        GenRace(Either::Former((self, l)))
    }
}

impl<C> Race for C where C: Generator {}
