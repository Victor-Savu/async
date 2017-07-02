#[macro_use]
extern crate o3;

use o3::iter::wrap::Wrap;
use o3::comb::all::All;
use o3::map::ret::MapReturn;

#[test]
fn all() {
    let first = (0..5).wrap().map_return(|_| "first");
    let second = (0..10).wrap().map_return(|_| "second");

    let mut trace = vec![];
    let both = each!(first.all(second) => i in {
        trace.push(i);
    });

    assert_eq!(trace, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);
    assert_eq!(both, ("first", "second"));
}
