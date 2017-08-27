use gen::{Generator, GenResult};
use either::GenEither;
use meta::sum::{Either, Sum};
use meta::prod::Prod;


pub struct GenRace<F, L>(GenEither<(F, L), (F, L)>)
    where F: Generator,
          L: Generator<Yield = F::Yield>;


impl<F, L> Generator for GenRace<F, L>
    where F: Generator,
          L: Generator<Yield = F::Yield>
{
    type Yield = F::Yield;
    type Return = GenEither<(F::Return, L), (F, L::Return)>;
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self.0 {
            GenEither::Former((f, l)) => {
                match f.next().to_canonical() {
                    Either::Left(s) => {
                        let (y, f) = s.to_canonical();
                        GenResult::Yield(y, GenRace(GenEither::Latter((f, l))))
                    }
                    Either::Right(f) => GenResult::Return(GenEither::Former((f, l))),
                }
            }
            GenEither::Latter((f, l)) => {
                match l.next().to_canonical() {
                    Either::Left(s) => {
                        let (y, l) = s.to_canonical();
                        GenResult::Yield(y, GenRace(GenEither::Former((f, l))))
                    }
                    Either::Right(l) => GenResult::Return(GenEither::Latter((f, l))),
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
        GenRace(GenEither::Former((self, l)))
    }
}

impl<C> Race for C where C: Generator {}


#[cfg(test)]
mod tests {

    use iter::wrap::Wrap;
    use map::ret::MapReturn;
    use comb::race::Race;
    use either::GenEither;

    #[test]
    fn race() {
        let first = (0..5).wrap().map_return(|_| "first");
        let second = (0..10).wrap().map_return(|_| "second");
        let mut trace = vec![];
        let loser = each!(first.race(second) => i in {
            trace.push(i);
        } then with result in {
            match result {
                GenEither::Former((message, latter)) => {
                    assert_eq!(message, "first");
                    latter
                },
                _ => panic!("The first one should finish first")
            }
        });
        assert_eq!(trace, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4]);

        trace.clear();
        let message = each!(loser => i in {
            trace.push(i);
        });

        assert_eq!(trace, [5, 6, 7, 8, 9]);
        assert_eq!(message, "second");
    }
}
