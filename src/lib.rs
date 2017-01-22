#![feature(never_type)]

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
    ($iter:expr => $elem:ident $loop_body:block then with $then:ident $then_body:block else $else_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                #[allow(unused_assignments)]
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

    ($iter:expr => $elem:ident $loop_body:block then $then_body:block else $else_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                #[allow(unused_assignments)]
                match iter_.next(()) {
                    CoResult::Yield($elem, tail) => {
                        iter_ = tail;
                        $loop_body;
                    },
                    CoResult::Return(_) => {
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

    ($iter:expr => $elem:ident $loop_body:block then with $then:ident $then_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            #[allow(unused_assignments)]
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
        fin
    }};

    ($iter:expr => $elem:ident $loop_body:block then $then_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            #[allow(unused_assignments)]
            match iter_.next(()) {
                CoResult::Yield($elem, tail) => {
                    iter_ = tail;
                    $loop_body;
                },
                CoResult::Return(_) => {
                    fin = $then_body;
                    break 'outer;
                }
            };
        }
        fin
    }};

    ($iter:expr => $elem:ident $loop_body:block else $else_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                #[allow(unused_assignments, unreachable_patterns, unreachable_code)]
                match iter_.next(()) {
                    CoResult::Yield($elem, tail) => {
                        iter_ = tail;
                        $loop_body;
                    },
                    CoResult::Return(ret) => {
                        fin = ret;
                        break 'outer;
                    }
                };
            }
            fin = $else_body;
            break;
        }
        fin
    }};

    ($iter:expr => $elem:ident $loop_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        loop {
            #[allow(unused_assignments)]
            match iter_.next(()) {
                CoResult::Yield($elem, tail) => {
                    iter_ = tail;
                    $loop_body;
                },
                CoResult::Return(ret) => {
                    fin = ret;
                    break;
                }
            };
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

    struct InfiniteCounter<T>{
        i: T
    }

    impl Coroutine<()> for InfiniteCounter<i64> {
        type Yield = i64;
        type Return = !;

        fn next(self, _: ()) -> CoResult<Self::Yield, Self, Self::Return> {
            CoResult::Yield(self.i, InfiniteCounter{i: self.i + 1})
        }
    }

    #[test]
    fn full_each() {
        let bart = Counter::<i64>{i: 3, lim: 10};
        let mut cnt = 3;
        let large_num = 1000;
        let message = each!(bart => i {
            assert_eq!(i, cnt);
            cnt += 1;
            if large_num < 5 { break; }
        } then with msg {
            String::from(msg) + " Yayy!"
        } else {
            String::from("I got broken!")
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, String::from("I'm done! Yayy!"));
    }

    #[test]
    fn full_each_with_break() {
        let bart = Counter::<i64>{i: 3, lim: 10};
        let mut cnt = 3;
        let message = each!(bart => i {
            assert_eq!(i, cnt);
            cnt += 1;
            break;
        } then with msg {
            msg
        } else {
            "I got broken!"
        });
        assert_eq!(cnt, 4);
        assert_eq!(message, "I got broken!");
    }

    #[test]
    fn each_ignore_return() {
        let bart = Counter::<i64>{i: 3, lim: 10};
        let mut cnt = 3;
        let large_number = 1000;
        let message = each!(bart => i {
            assert_eq!(i, cnt);
            cnt += 1;
            if large_number < 5 { break; }
        } then {
            "At last!"
        } else {
            "I got broken!"
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, "At last!");
    }

    #[test]
    fn each_break_no_then() {
        let bart = InfiniteCounter::<i64>{i: 3};
        let mut cnt = 3;
        let message = each!(bart => i {
            assert_eq!(i, cnt);
            cnt += 1;
            if cnt >= 20 { break; }
        } else {
            "I got broken!"
        });
        assert_eq!(cnt, 20);
        assert_eq!(message, "I got broken!");
    }

    #[test]
    fn each_no_else() {
        let bart = Counter::<i64>{i: 3, lim: 10};
        let mut cnt = 3;
        let message = each!(bart => i {
            assert_eq!(i, cnt);
            cnt += 1;
        } then {
            "At last!"
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, "At last!");
    }

    #[test]
    fn each_no_then_else() {
        let bart = Counter::<i64>{i: 3, lim: 10};
        let mut cnt = 3;
        let message = each!(bart => i {
            assert_eq!(i, cnt);
            cnt += 1;
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, "I'm done!");
    }

    #[test]
    fn each_no_then() {
        let bart = Counter::<i64>{i: 3, lim: 10};
        let mut cnt = 3;
        #[allow(unreachable_code)]
        let message = each!(bart => i {
            assert_eq!(i, cnt);
            cnt += 1;
        } else {
            "bogus"
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, "I'm done!");
    }
}
