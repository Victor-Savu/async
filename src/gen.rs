use std::ops::RangeFrom;

pub enum GenResult<Coro>
    where Coro: Generator
{
    Yield(Coro::Yield, Coro),
    Return(Coro::Return),
}

pub trait Generator: Sized {
    type Yield;
    type Return;

    fn next(self) -> GenResult<Self>;
}

impl<Idx> Generator for RangeFrom<Idx>
    where Self: Iterator
{
    type Yield = <Self as Iterator>::Item;
    type Return = !;

    fn next(self) -> GenResult<Self> {
        let mut x = self;
        loop {
            if let Some(y) = Iterator::next(&mut x) {
                break GenResult::Yield(y, x);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn range_from() {
        fn foo(q: i64) -> (usize, Vec<i64>) {
            let mut x = q;
            let mut v = vec![];
            each!(1.. => steps in {
                v.push(x);
                x = if x == 1 {
                    return (steps, v)
                } else if x % 2 == 0 {
                    x / 2
                } else {
                    x * 3 + 1
                };
            })
        }

        let (steps, values) = foo(10);

        assert_eq!(steps, values.len());
        assert_eq!(values, [10, 5, 16, 8, 4, 2, 1]);
    }
}
