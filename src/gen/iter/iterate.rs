use gen::Generator;
use cat::sum::Either;


pub struct GenIterate<C>(Option<C>);

impl<C> Iterator for GenIterate<C>
    where C: Generator
{
    type Item = C::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(coro) => {
                match coro.next().to_canonical() {
                    Either::Left(s) => {
                        let (i, cnt) = s.to_canonical();
                        self.0 = Some(cnt);
                        Some(i)
                    }
                    _ => None,
                }
            }
            None => None,
        }
    }
}

pub trait Iterate
    where Self: Sized
{
    fn iterate(self) -> GenIterate<Self> {
        GenIterate(Some(self))
    }
}

impl<C> Iterate for C where C: Generator {}


#[cfg(test)]
mod tests {

    use gen::iter::{Iterate, Wrap};

    #[test]
    fn iterate_over_coroutine() {
        let mut cnt = 3;
        let lim = 10;
        let bart = (cnt..lim).wrap();
        for i in bart.iterate() {
            assert_eq!(i, cnt);
            cnt += 1;
        }
        assert_eq!(cnt, lim);
    }

    #[test]
    fn wrap_iterator() {
        let mut cnt = 1;
        let message = each!((1..10).wrap() => i in {
            assert_eq!(i, cnt);
            cnt += 1;
        } then with mut iter in {
            assert_eq!(iter.next(), None);
            assert_eq!(cnt, 10);
            42
        });
        assert_eq!(message, 42);
    }
}
