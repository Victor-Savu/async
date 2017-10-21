use gen::{Generator, GenResult, Yields, Returns};
use gen::either::GenEither;
use cat::sum::Either;
use cat::{Iso, Inj};


pub struct GenRace<F, L>(GenEither<(F, L), (F, L)>)
    where F: Generator,
          L: Generator<Yield = F::Yield>,
          L::Transition: Iso<Either<(F::Yield, L), L::Return>>;

impl<F, L> Yields for GenRace<F, L>
    where F: Yields
{
    type Yield = F::Yield;
}

impl<F, L> Returns for GenRace<F, L>
    where F: Returns,
          L: Returns
{
    type Return = GenEither<(F::Return, L), (F, L::Return)>;
}

impl<F, L> Generator for GenRace<F, L>
    where F: Generator,
          L::Transition: Iso<Either<(F::Yield, L), L::Return>>,
          L: Generator<Yield = F::Yield>
{
    type Transition = GenResult<Self>;

    fn next(self) -> GenResult<Self> {
        match self.0 {
            GenEither::Former((f, l)) => {
                match f.next().inj() {
                    Either::Left(s) => {
                        let (y, f) = s.inj();
                        GenResult::Yield(y, GenRace(GenEither::Latter((f, l))))
                    }
                    Either::Right(f) => GenResult::Return(GenEither::Former((f, l))),
                }
            }
            GenEither::Latter((f, l)) => {
                match l.next().inj() {
                    Either::Left(s) => {
                        let (y, l) = s.inj();
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
        where L: Generator<Yield = Self::Yield>,
              L::Transition: Iso<Either<(Self::Yield, L), L::Return>>
    {
        GenRace(GenEither::Former((self, l)))
    }
}

impl<C> Race for C where C: Generator {}


#[cfg(test)]
mod tests {

    use gen::iter::wrap::Wrap;
    use gen::map::ret::MapReturn;
    use gen::comb::race::Race;
    use gen::either::GenEither;

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
