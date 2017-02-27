use co::{Coroutine, CoResult};

struct CoIterWrapper<T>(T);

impl<Iter> Coroutine<()> for CoIterWrapper<Iter> where Iter: Iterator {
    type Yield = Iter::Item;
    type Return = Iter;

    fn next(self, _: ()) -> CoResult<Self::Yield, Self, Self::Return> {
        let mut i = self.0;
        match i.next() {
            Some(item) => CoResult::Yield(item, CoIterWrapper(i)),
            _ => CoResult::Return(i)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_iterator() {
        let mut cnt = 1;
        let message = each!(CoIterWrapper(1..10) => i in {
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
