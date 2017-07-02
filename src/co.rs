#![macro_use]

pub enum CoResult<Coro>
    where Coro: Coroutine
{
    Yield(Coro::Yield, Coro::Continue),
    Return(Coro::Return),
}

pub trait Coroutine: Sized {
    type Yield;
    type Return;
    type Continue;

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

    // no_else
    ($iter:expr => $elem:pat in
         $loop_body:block
     then with $then:pat in
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
                    $crate::co::CoResult::Return($then) => {
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
