pub mod arg;
pub mod yld;
pub mod ret;

pub struct CoMap<C, F> {
    c: C,
    f: F,
}

#[cfg(test)]
mod tests {
    use co::{Coroutine, CoResult};

    struct Counter<T> {
        i: T,
        lim: T,
    }

    impl Coroutine<()> for Counter<i64> {
        type Yield = i64;
        type Return = &'static str;

        fn next(self, _: ()) -> CoResult<Self::Yield, Self, Self::Return> {
            if self.i < self.lim {
                CoResult::Yield(self.i,
                                Counter {
                                    i: self.i + 1,
                                    lim: self.lim,
                                })
            } else {
                CoResult::Return("I'm done!")
            }
        }
    }

    struct InfiniteCounter<T> {
        i: T,
    }

    impl Coroutine<()> for InfiniteCounter<i64> {
        type Yield = i64;
        type Return = !;

        fn next(self, _: ()) -> CoResult<Self::Yield, Self, Self::Return> {
            CoResult::Yield(self.i, InfiniteCounter { i: self.i + 1 })
        }
    }

    #[test]
    fn map_app() {
        let bart = Counter::<i64> { i: 3, lim: 10 };
        let mut cnt = 3;
        let large_num = 1000;
        let message = each!(bart => i in {
            assert_eq!(i, cnt);
            cnt += 1;
            if large_num < 5 { break; }
        } then with msg in {
            String::from(msg) + " Yayy!"
        } else {
            String::from("I got broken!")
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, String::from("I'm done! Yayy!"));
    }
}
