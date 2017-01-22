pub enum CoResult<YieldT, CoroT, ReturnT> {
    Yield(YieldT, CoroT),
    Return(ReturnT)
}

trait Coroutine<Input> : Sized {
    type Yield;
    type Return;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return>;
}

macro_rules! each {
    ($iter:expr => $elem:ident $loop_body:block then $then:ident $then_body:block else $else_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        #[allow(unused_assignments)]
        'outer: loop {
            loop {
                match iter_.next(()) {
                    CoResult::Yield($elem, tail) => {
                        iter_ = tail;
                        $loop_body;
                    },
                    CoResult::Return($then) => {
                        fin = $then_body;
                        break 'outer;
                    }
                };
            }
            fin = $else_body;
            break;
        }
        fin
    }};
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Counter<T>{
        i: T,
        lim: T
    }

    impl Coroutine<()> for Counter<i64> {
        type Yield = i64;
        type Return = &'static str;

        fn next(self, _: ()) -> CoResult<Self::Yield, Self, Self::Return> {
            if self.i < self.lim {
                CoResult::Yield(self.i, Counter{i: self.i + 1, lim: self.lim})
            } else {
                CoResult::Return("I'm done!")
            }
        }
    }

    #[test]
    fn it_works() {
        let bart = Counter::<i64>{i: 3, lim: 10};
        assert!(
        match bart.next(()) {
            CoResult::Yield(i, Counter::<i64>{i: j, lim: k}) => {
                assert_eq!(i, 3);
                assert_eq!(j, 4);
                assert_eq!(k, 10);
                true
            },
            CoResult::Return(_) => {
                false
            }
        });
    }

    #[test]
    fn each_works() {
        let bart = Counter::<i64>{i: 3, lim: 10};
        let mut cnt = 3;
        let message = each!(bart => i {
            assert_eq!(i, cnt);
            cnt += 1;
            break;
        } then msg {
            msg
        } else {
            "I got broken"
        });
        assert_eq!(cnt, 4);
        assert_eq!(message, "I got broken");
    }
}
