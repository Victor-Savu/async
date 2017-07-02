// use co::{Coroutine, CoResult};
// use race::{CoRace, Race};


pub struct Append<A> {
    a: A
}

impl<A, I> FnOnce<I> for Append<A> {
    type Output = (I, A);

    extern "rust-call" fn call_once(self, i: I) -> Self::Output {
        (i, self.a)
    }
}
