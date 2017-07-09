#![feature(never_type)]

#[macro_use]
extern crate o3;

use o3::iter::wrap::Wrap;


#[test]
fn infinite_range() {
    fn foo(q: i64) -> (usize, Vec<i64>) {
        let mut x = q;
        let mut v = vec![];
        each!((1..).wrap() => steps in {
            v.push(x);
            x = if x == 1 {
                return (steps, v)
            } else if x % 2 == 0 {
                x / 2
            } else {
                x * 3 + 1
            };
        }
        );
    }

    let (steps, values) = foo(10);

    assert_eq!(steps, values.len());
    assert_eq!(values, [10, 5, 16, 8, 4, 2, 1]);
}
