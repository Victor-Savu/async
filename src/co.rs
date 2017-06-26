#![macro_use]

pub enum CoResult<Coro>
    where Coro: Coroutine
{
    Yield(Coro::Yield, Coro),
    Return(Coro::Return),
}

pub trait Coroutine: Sized {
    type Yield;
    type Return;

    fn next(self) -> CoResult<Self>;
}

pub enum CoState<C>
    where C: Coroutine
{
    Live(C),
    Done(C::Return),
}

#[macro_export]
macro_rules! each {
    // full_each
    ($iter:expr => $elem:pat in
         $loop_body:block
     then with $then:pat in
         $then_body:block
     else
         $else_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                match iter_.next() {
                    CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
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

    // no_else
    ($iter:expr => $elem:pat in
         $loop_body:block
     then with $then:pat in
         $then_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                match iter_.next() {
                    CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
                        $loop_body;
                    },
                    CoResult::Return($then) => {
                        fin = $then_body;
                        break 'outer;
                    }
                };
            }
            #[allow(unreachable_code)] // if $loop_body contains a `break` statement
            {
                break;
            }
        }
        fin
    }};

    // no_with
    ($iter:expr => $elem:pat in
         $loop_body:block
     then
         $then_body:block
     else
         $else_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                match iter_.next() {
                    CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
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

    // no_with_else
    ($iter:expr => $elem:pat in
         $loop_body:block
     then
         $then_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                match iter_.next() {
                    CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
                        $loop_body;
                    },
                    CoResult::Return(_) => {
                        fin = $then_body;
                        break 'outer;
                    }
                };
            }
            #[allow(unreachable_code)] // if $loop_body contains a `break` statement
            {
                break;
            }
        }
        fin
    }};

    // no_then
    ($iter:expr => $elem:pat in
         $loop_body:block
     else
         $else_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                #[allow(unreachable_patterns, unreachable_code)] // if $iter::Return is !
                match iter_.next() {
                    CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
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

    // no_then_else
    ($iter:expr => $elem:pat in
         $loop_body:block) => {{
        let mut iter_ = $iter;
        let fin;
        loop {
            match iter_.next() {
                CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
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

    struct Counter<T> {
        i: T,
        lim: T,
    }

    impl Coroutine for Counter<i64> {
        type Yield = i64;
        type Return = &'static str;

        fn next(self) -> CoResult<Self> {
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

    impl Coroutine for InfiniteCounter<i64> {
        type Yield = i64;
        type Return = !;

        fn next(self) -> CoResult<Self> {
            CoResult::Yield(self.i, InfiniteCounter { i: self.i + 1 })
        }
    }

    #[test]
    fn full_each() {
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

    #[test]
    fn full_each_with_break() {
        let bart = Counter::<i64> { i: 3, lim: 10 };
        let mut cnt = 3;
        let message = each!(bart => i in {
            assert_eq!(i, cnt);
            cnt += 1;
            break;
        } then with msg in {
            msg
        } else {
            "I got broken!"
        });
        assert_eq!(cnt, 4);
        assert_eq!(message, "I got broken!");
    }

    #[test]
    fn full_each_with_capture_patterns() {

        struct Blabber<T> {
            i: T,
            lim: T,
        }

        impl Coroutine for Blabber<i64> {
            type Yield = (i64, i64);
            type Return = (&'static str, i64);

            fn next(self) -> CoResult<Self> {
                if self.i < self.lim {
                    CoResult::Yield((self.i, self.lim),
                                    Blabber {
                                        i: self.i + 1,
                                        lim: self.lim,
                                    })
                } else {
                    CoResult::Return(("I'm done!", self.lim))
                }
            }
        }

        let bart = Blabber::<i64> { i: 3, lim: 10 };
        let mut cnt = 3;
        let large_num = 1000;
        let (message, lim) = each!(bart => (i, lim) in {
            assert_eq!(i, cnt);
            assert_eq!(lim, 10);
            cnt += 1;
            if large_num < 5 { break; }
        } then with (msg, lim) in {
            (String::from(msg) + " Yayy!", lim)
        } else {
            (String::from("I got broken!"), -1)
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, String::from("I'm done! Yayy!"));
        assert_eq!(lim, 10);
    }

    #[test]
    fn no_with() {
        let bart = Counter::<i64> { i: 3, lim: 10 };
        let mut cnt = 3;
        let large_number = 1000;
        let message = each!(bart => i in {
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
    fn no_else() {
        let bart = Counter::<i64> { i: 3, lim: 10 };
        let mut cnt = 3;
        let message = each!(bart => i in {
            assert_eq!(i, cnt);
            cnt += 1;
        } then with msg in {
            String::from(msg) + " Yayy!"
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, String::from("I'm done! Yayy!"));
    }

    #[test]
    fn no_with_else() {
        let bart = Counter::<i64> { i: 3, lim: 10 };
        let mut cnt = 3;
        let message = each!(bart => i in {
            assert_eq!(i, cnt);
            cnt += 1;
        } then {
            "At last!"
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, "At last!");
    }


    #[test]
    fn no_then() {
        let bart = Counter::<i64> { i: 3, lim: 10 };
        let mut cnt = 3;
        let message = each!(bart => i in {
            assert_eq!(i, cnt);
            cnt += 1;
            if cnt > 100 { break; }
        } else {
            "bogus"
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, "I'm done!");
    }

    #[test]
    fn no_then_with_break() {
        let bart = InfiniteCounter::<i64> { i: 3 };
        let mut cnt = 3;
        let message = each!(bart => i in {
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
    fn no_then_else() {
        let bart = Counter::<i64> { i: 3, lim: 10 };
        let mut cnt = 3;
        let message = each!(bart => i in {
            assert_eq!(i, cnt);
            cnt += 1;
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, "I'm done!");
    }
}
