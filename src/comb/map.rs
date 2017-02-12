use co::{Coroutine, CoResult};

pub struct CoMap<C, F> {
    c: C,
    f: F,
}

pub struct CoArgMap<C, F>(CoMap<C, F>);

pub trait ArgMap<F, Input, Output>
    where F: FnOnce(Input) -> Output,
          Self: Sized + Coroutine<Output>
{
    fn arg_map(self, f: F) -> CoArgMap<Self, F> {
        CoArgMap(CoMap { c: self, f: f })
    }
}

impl<C, F, Input, Output> ArgMap<F, Input, Output> for C
    where C: Sized + Coroutine<Output>,
          F: FnOnce(Input) -> Output
{
}

impl<C, F, Input, Output> Coroutine<Input> for CoArgMap<C, F>
    where F: FnMut(Input) -> Output,
          C: Sized + Coroutine<Output>
{
    type Yield = C::Yield;
    type Return = C::Return;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        let mut f = self.0.f;
        match self.0.c.next(f(i)) {
            CoResult::Yield(y, c) => CoResult::Yield(y, c.arg_map(f)),
            CoResult::Return(res) => CoResult::Return(res),
        }
    }
}

pub struct CoYieldMap<C, F>(CoMap<C, F>);

pub trait YieldMap<F, Input, Output>
    where Self: Sized + Coroutine<Input>,
          F: FnOnce(Self::Yield) -> Output
{
    fn yield_map(self, f: F) -> CoYieldMap<Self, F> {
        CoYieldMap(CoMap { c: self, f: f })
    }
}

impl<C, F, Input, Output> YieldMap<F, Input, Output> for C
    where C: Sized + Coroutine<Input>,
          F: FnOnce(C::Yield) -> Output
{
}

impl<C, F, Input, Output> Coroutine<Input> for CoYieldMap<C, F>
    where F: FnMut(C::Yield) -> Output,
          C: Sized + Coroutine<Input>
{
    type Yield = Output;
    type Return = C::Return;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        let mut f = self.0.f;
        match self.0.c.next(i) {
            CoResult::Yield(y, c) => CoResult::Yield(f(y), c.yield_map(f)),
            CoResult::Return(res) => CoResult::Return(res),
        }
    }
}

pub struct CoReturnMap<C, F>(CoMap<C, F>);

pub trait ReturnMap<F, Input, Output>
    where F: FnOnce(Self::Return) -> Output,
          Self: Sized + Coroutine<Input>
{
    fn return_map(self, f: F) -> CoReturnMap<Self, F> {
        CoReturnMap(CoMap { c: self, f: f })
    }
}

impl<C, F, Input, Output> ReturnMap<F, Input, Output> for C
    where C: Sized + Coroutine<Input>,
          F: FnOnce(C::Return) -> Output
{
}

impl<C, F, Input, Output> Coroutine<Input> for CoReturnMap<C, F>
    where F: FnOnce(C::Return) -> Output,
          C: Sized + Coroutine<Input>
{
    type Yield = C::Yield;
    type Return = Output;

    fn next(self, i: Input) -> CoResult<Self::Yield, Self, Self::Return> {
        match self.0.c.next(i) {
            CoResult::Yield(y, c) => CoResult::Yield(y, c.return_map(self.0.f)),
            CoResult::Return(res) => CoResult::Return((self.0.f)(res)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let bart = Counter::<i64> { i:3, lim: 10 };
        let mut cnt = 3;
        let large_num = 1000;
        let message = each!(bart => i in {
            assert_eq!(i, cnt);
            cnt += 1;
            if large_num < 5 { break; }
        } then with msg in {
            String::from(msg) + " Yayy!"
        } otherwise {
            String::from("I got broken!")
        });
        assert_eq!(cnt, 10);
        assert_eq!(message, String::from("I'm done! Yayy!"));
    }
}
