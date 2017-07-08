#[macro_use]
extern crate o3;

use o3::iter::{Iterate, Wrap};

#[test]
fn iterate_over_coroutine() {
    let mut cnt = 3;
    let lim = 10;
    let bart = (cnt..lim).wrap();
    for i in bart.iterate() {
        assert_eq!(i, cnt);
        cnt += 1;
    }
    assert_eq!(cnt, lim);
}

#[test]
fn wrap_iterator() {
    let mut cnt = 1;
    let message = each!((1..10).wrap() => i in {
        assert_eq!(i, cnt);
        cnt += 1;
    } then with mut iter in {
        assert_eq!(iter.next(), None);
        assert_eq!(cnt, 10);
        42
    });
    assert_eq!(message, 42);
}
