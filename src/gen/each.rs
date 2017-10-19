#[macro_export]
macro_rules! _each_impl {

// full
($iter:expr => $elem:pat in
     $loop_body:block
 then with $then_:pat in
     $then_body:block
 else with $else_:pat, gen $rest_:pat in
     $else_body:block) => {{
    let mut iter_ = $iter;
    'outer: loop {
        let $else_ = loop {
            match $crate::cat::Inj::inj($crate::gen::Generator::next(iter_)) {
                $crate::cat::sum::Either::Left(s) => {
                    let ($elem, tail) = $crate::cat::Inj::inj(s);
                    #[allow(unused_assignments)] {
                        iter_ = tail
                    }
                    $loop_body
                },
                $crate::cat::sum::Either::Right($then_) => {
                    break 'outer $then_body;
                }
            }
        };
        let $rest_ = iter_;
        break 'outer $else_body;
    }
}};

// without $else_body
($iter:expr => $elem:pat in
     $loop_body:block
 then with $then_:pat in
     $then_body:block) => {{
    let mut iter_ = $iter;
    loop {
        #[allow(unreachable_patterns)] {
            match $crate::cat::Inj::inj($crate::gen::Generator::next(iter_)) {
                $crate::cat::sum::Either::Left(s) => {
                    let ($elem, tail) = $crate::cat::Inj::inj(s);
                    #[allow(unused_assignments)] {
                        iter_ = tail
                    }
                    #[warn(unreachable_patterns)] {
                        $loop_body
                    }
                },
                $crate::cat::sum::Either::Right($then_) => {
                    #[warn(unreachable_patterns)] {
                        #[allow(unreachable_code)] {
                            break {
                                #[warn(unreachable_code)] {
                                    $then_body
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}};

}

#[macro_export]
macro_rules! each {

($iter:expr => $elem:pat in
     $loop_body:block
 then with $then_:pat in
     $then_body:block
 else with $else_:pat, gen $rest_:pat in
     $else_body:block) => {{
    _each_impl!($iter => $elem in
        $loop_body
    then with $then_ in
        $then_body
    else with $else_, gen $rest_ in
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 then with $then_:pat in
     $then_body:block
 else with $else_:pat in
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with $then_ in
        $then_body
    else with $else_, gen _ in
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 then with $then_:pat in
     $then_body:block
 else gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with $then_ in
        $then_body
    else with _, gen $rest_ in
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 then with $then_:pat in
     $then_body:block
 else
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with $then_ in
        $then_body
    else gen _ in
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 then with $then_:pat in
     $then_body:block) => {{
    _each_impl!($iter => $elem in
        $loop_body
    then with $then_ in
        $then_body
    )
}};

// no $then_
($iter:expr => $elem:pat in
     $loop_body:block
 then
     $then_body:block
 else with $else_:pat, gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with _ in
        $then_body
    else with $else_, gen $rest_ in
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 then
     $then_body:block
 else with $else_:pat in
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with _ in
        $then_body
    else with $else_ in
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 then
     $then_body:block
 else gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with _ in
        $then_body
    else gen $rest_ in
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 then
     $then_body:block
 else
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with _ in
        $then_body
    else
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 then
     $then_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with _ in
        $then_body
    )
}};

// no then_body
($iter:expr => $elem:pat in
     $loop_body:block
 else with $else_:pat, gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with break_value in {
        break_value
    } else with $else_, gen $rest_ in
        $else_body
    )
}};
($iter:expr => $elem:pat in
     $loop_body:block
 else with $else_:pat in
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with break_value in {
        break_value
    } else with $else_ in
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 else gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with break_value in {
        break_value
    } else gen $rest_ in
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block
 else
     $else_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with break_value in {
        break_value
    } else
        $else_body
    )
}};

($iter:expr => $elem:pat in
     $loop_body:block) => {{
    each!($iter => $elem in
        $loop_body
    then with break_value in {
        break_value
    })
}};

($iter:expr =>
 then with $then_:pat in
     $then_body:block) => {{
    each!($iter => _ in {}
    then with $then_ in
        $then_body
    )
}};

($iter:expr =>
 then
     $then_body:block) => {{
    each!($iter =>
    then with _ in
        $then_body
    )
}};
// no $elem

($iter:expr =>
     $loop_body:block
 then with $then_:pat in
     $then_body:block
 else with $else_:pat, gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with $then_ in
        $then_body
    else with $else_, gen $rest_ in
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 then with $then_:pat in
     $then_body:block
 else with $else_:pat in
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with $then_ in
        $then_body
    else with $else_, gen _ in
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 then with $then_:pat in
     $then_body:block
 else gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with $then_ in
        $then_body
    else with _, gen $rest_ in
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 then with $then_:pat in
     $then_body:block
 else
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with $then_ in
        $then_body
    else gen _ in
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 then with $then_:pat in
     then_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with $then_ in
        $then_body
    )
}};

// no $then_
($iter:expr =>
     $loop_body:block
 then
     $then_body:block
 else with $else_:pat, gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with _ in
        $then_body
    else with $else_, gen $rest_ in
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 then
     $then_body:block
 else with $else_:pat in
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with _ in
        $then_body
    else with $else_ in
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 then
     $then_body:block
 else gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with _ in
        $then_body
    else gen $rest_ in
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 then
     $then_body:block
 else
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with _ in
        $then_body
    else
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 then
     $then_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with _ in
        $then_body
    )
}};

// no then_body
($iter:expr =>
     $loop_body:block
 else with $else_:pat, gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with break_value in {
        break_value
    } else with $else_, gen $rest_ in
        $else_body
    )
}};
($iter:expr =>
     $loop_body:block
 else with $else_:pat in
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with break_value in {
        break_value
    } else with $else_ in
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 else gen $rest_:pat in
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with break_value in {
        break_value
    } else gen $rest_ in
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block
 else
     $else_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with break_value in {
        break_value
    } else
        $else_body
    )
}};

($iter:expr =>
     $loop_body:block) => {{
    each!($iter => _ in
        $loop_body
    then with break_value in {
        break_value
    })
}};

($iter:expr) => {{
    each!($iter =>
    then with final_value in {
        final_value
    })
}};

}

#[cfg(test)]
mod tests {

    use gen::iter::wrap::Wrap;
    use gen::map::ret::MapReturn;
    use gen::map::yld::MapYield;
    use gen::Generator;

    #[test]
    fn each_0() {
        use std::fmt::Display;

        fn run<S: Generator, B>(stream: S, should_break: B) -> (String, Vec<S::Yield>, Option<S>)
            where B: Fn(S::Yield) -> Option<S::Yield>,
                  S::Yield: Display + Copy,
                  S::Return: Display
        {
            let mut num = vec![];
            let mut rest = None;
            let message = each!(stream => i in {
                if let Some(value) = should_break(i) {
                    break value
                } else {
                    num.push(i)
                }
            } then with msg in {
                format!("Finished: {}", msg)
            } else with msg, gen rem in {
                rest = Some(rem);
                format!("Broken: {}", msg)
            });
            (message, num, rest)
        }

        // finishes normally
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
        let (msg, num, rest) = run(bart, |x| if x > 20 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(msg, "Finished: I'm done!");
        assert!(rest.is_none());

        // is broken
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
        let (msg, num, rest) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6]);
        assert_eq!(msg, "Broken: 7");
        let stream = rest.expect("The stream was broken, so there should be some left.");
        let (msg, num, rest) = run(stream, |_| None);
        assert_eq!(num, [8, 9]);
        assert_eq!(msg, "Finished: I'm done!");
        assert!(rest.is_none());

        // is broken, otherwise would never finish
        let bart = (3..).map_return(|_| "I'm done!");
        let (msg, num, rest) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6]);
        assert_eq!(msg, "Broken: 7");
        let stream = rest.expect("The stream was broken, so there should be some left.");
        let (msg, num, rest) = run(stream, |x| if x > 12 { Some(x) } else { None });
        assert_eq!(num, [8, 9, 10, 11, 12]);
        assert_eq!(msg, "Broken: 13");
        assert!(rest.is_some());

        // doesn't yield
        /*
        let bart = 3.done();
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, []);
        assert_eq!(msg, "Finished: 3");
        */







    }

    #[test]
    fn each_1() {
        use std::fmt::Display;

        fn run<S: Generator, B>(stream: S, should_break: B) -> (String, Vec<S::Yield>)
            where B: Fn(S::Yield) -> Option<S::Yield>,
                  S::Yield: Display + Copy,
                  S::Return: Display
        {
            let mut num = vec![];
            let message = each!(stream => i in {
                if let Some(value) = should_break(i) {
                    break value
                } else {
                    num.push(i)
                }
            } then with msg in {
                format!("Finished: {}", msg)
            } else with msg in {
                format!("Broken: {}", msg)
            });
            (message, num)
        }

        // finishes normally
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x > 20 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(msg, "Finished: I'm done!");

        // is broken
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6]);
        assert_eq!(msg, "Broken: 7");

        // is broken, otherwise would never finish
        let bart = (3..).map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6]);
        assert_eq!(msg, "Broken: 7");

        // doesn't yield
        /*
        let bart = 3.done();
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, []);
        assert_eq!(msg, "Finished: 3");
        */







    }

    #[test]
    fn each_2() {
        use std::fmt::Display;

        fn run<S: Generator, B>(stream: S, should_break: B) -> (String, Vec<S::Yield>)
            where B: Fn(S::Yield) -> Option<S::Yield>,
                  S::Yield: Display + Copy,
                  S::Return: Display
        {
            let mut num = vec![];
            let message = each!(stream => i in {
                if let Some(value) = should_break(i) {
                    break value
                } else {
                    num.push(i)
                }
            } then with msg in {
                format!("Finished: {}", msg)
            } else with msg in {
                format!("Broken: {}", msg)
            });
            (message, num)
        }

        // finishes normally
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x > 20 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(msg, "Finished: I'm done!");

        // is broken
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6]);
        assert_eq!(msg, "Broken: 7");

        // is broken, otherwise would never finish
        let bart = (3..).map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6]);
        assert_eq!(msg, "Broken: 7");

        // doesn't yield
        /*
        let bart = 3.done();
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, []);
        assert_eq!(msg, "Finished: 3");
        */







    }

    #[test]
    fn each_3() {
        use std::fmt::Display;

        fn run<S: Generator, B>(stream: S, should_break: B) -> (String, Vec<S::Yield>)
            where B: Fn(S::Yield) -> Option<S::Yield>,
                  S::Yield: Display + Copy,
                  S::Return: Display
        {
            let mut num = vec![];
            let message = each!(stream => i in {
                if let Some(value) = should_break(i) {
                    break value
                } else {
                    num.push(i)
                }
            } then with msg in {
                format!("Finished: {}", msg)
            } else {
                format!("Broken")
            });
            (message, num)
        }

        // finishes normally
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x > 20 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(msg, "Finished: I'm done!");

        // is broken
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6]);
        assert_eq!(msg, "Broken");

        // is broken, otherwise would never finish
        let bart = (3..).map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, [3, 4, 5, 6]);
        assert_eq!(msg, "Broken");

        // doesn't yield
        /*
        let bart = 3.done();
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, []);
        assert_eq!(msg, "Finished: 3");
        */







    }

    #[test]
    fn each_4() {
        use std::fmt::Display;

        fn run<S: Generator, B>(stream: S, should_break: B) -> (String, Vec<S::Yield>)
            where B: Fn(S::Yield) -> Option<S::Yield>,
                  S::Yield: Display + Copy,
                  S::Return: Display
        {
            let mut num = vec![];
            let message = each!(stream => i in {
                if let Some(value) = should_break(i) {
                    break format!("{}", value)
                } else {
                    num.push(i)
                }
            } then with msg in {
                format!("Finished: {}", msg)
            });
            (message, num)
        }

        // finishes normally
        let base = ["Hello", "World!"];
        let bart = base.iter().wrap().map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |_| None);
        assert_eq!(num, [&"Hello", &"World!"]);
        assert_eq!(msg, "Finished: I'm done!");

        // is broken
        let bart = base.iter().wrap().map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| Some(x));
        assert!(num.is_empty());
        assert_eq!(msg, "Hello");

        // is broken later
        let bart = base.iter().wrap().map_return(|_| "I'm done!");
        let (msg, num) = run(bart, |x| if x == &"World!" { Some(x) } else { None });
        assert_eq!(num, [&"Hello"]);
        assert_eq!(msg, "World!");

        // doesn't yield
        /*
        let bart = 3.done();
        let (msg, num) = run(bart, |x| if x > 6 { Some(x) } else { None });
        assert_eq!(num, []);
        assert_eq!(msg, "Finished: 3");
        */







    }

    #[test]
    fn full_each_with_break() {
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
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
        let bart = (3..10).wrap().map_yield(|i| (i, 10)).map_return(|_| ("I'm done!", 10));
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
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
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
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
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
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
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
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
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
        let bart =
            (3..).wrap().map_return(|_| unreachable!("An infinite series should not return"));
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
        let bart = (3..10).wrap().map_return(|_| "I'm done!");
        let mut cnt = 3;
        let message = each!(bart => i in {
        assert_eq!(i, cnt);
        cnt += 1;
    });
        assert_eq!(cnt, 10);
        assert_eq!(message, "I'm done!");
    }
}
