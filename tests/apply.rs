#[macro_use]
extern crate o3;

use o3::comb::done::Done;
use o3::comb::apply::Apply;


#[test]
fn apply() {
    let four = (|x| x+1).done().apply(3.done());
    let res = each!(four);
    assert_eq!(res, 4);
}
