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

#[macro_export]
macro_rules! each {
    // full_each
    ($iter:expr => $elem:pat in
         $loop_body:block
     then with $then:pat in
         $then_body:block
     else
         $else_body:block) => {{
        use $crate::co::Coroutine;
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                match iter_.next() {
                    $crate::co::CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
                        $loop_body;
                    },
                    $crate::co::CoResult::Return($then) => {
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

    // no_body_no_else
    ($iter:expr, $then:pat in
         $then_body:block) => {{
        use $crate::co::Coroutine;
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            match iter_.next() {
                $crate::co::CoResult::Yield(_, tail) => {
                    #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                    {
                        iter_ = tail;
                    }
                },
                $crate::co::CoResult::Return($then) => {
                    fin = $then_body;
                    break 'outer;
                }
            };
        }
        fin
    }};

    // jut_the_coroutine
    ($iter:expr) => {{
        use $crate::co::Coroutine;
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            match iter_.next() {
                $crate::co::CoResult::Yield(_, tail) => {
                    #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                    {
                        iter_ = tail;
                    }
                },
                $crate::co::CoResult::Return(ret) => {
                    fin = ret;
                    break 'outer;
                }
            };
        }
        fin
    }};

    // no_else
    ($iter:expr => $elem:pat in
         $loop_body:block
     then with $then:pat in
         $then_body:block) => {{
        use $crate::co::Coroutine;
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            match iter_.next() {
                $crate::co::CoResult::Yield($elem, tail) => {
                    #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                    {
                        iter_ = tail;
                    }
                    $loop_body;
                },
                $crate::co::CoResult::Return($then) => {
                    fin = $then_body;
                    break 'outer;
                }
            };
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
        use $crate::co::Coroutine;
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                match iter_.next() {
                    $crate::co::CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
                        $loop_body;
                    },
                    $crate::co::CoResult::Return(_) => {
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
        use $crate::co::Coroutine;
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                match iter_.next() {
                    $crate::co::CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
                        $loop_body;
                    },
                    $crate::co::CoResult::Return(_) => {
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
        use $crate::co::Coroutine;
        let mut iter_ = $iter;
        let fin;
        'outer: loop {
            loop {
                #[allow(unreachable_patterns, unreachable_code)] // if $iter::Return is !
                match iter_.next() {
                    $crate::co::CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
                        $loop_body;
                    },
                    $crate::co::CoResult::Return(ret) => {
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
        use $crate::co::Coroutine;
        let mut iter_ = $iter;
        let fin;
        loop {
            match iter_.next() {
                $crate::co::CoResult::Yield($elem, tail) => {
                        #[allow(unused_assignments)] // if $loop_body contains a `break` statement
                        {
                            iter_ = tail;
                        }
                    $loop_body;
                },
                $crate::co::CoResult::Return(ret) => {
                    fin = ret;
                    break;
                }
            };
        }
        fin
    }};
}
