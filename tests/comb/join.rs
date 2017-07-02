#[macro_use]
extern crate o3;

use o3::comb::join::Join;
use o3::map::ret::MapReturn;
use o3::iter::wrap::Wrap;

#[test]
fn chain_integers() {
    let first = (1..9).wrap();
    let second = (1..3).wrap();

    let msg = "This is the end";
    let chain = first.map_return(|_| second.map_return(|_| msg)).join();
    let mut elem = 1;
    let message = each!(chain => i in {
        assert_eq!(i, elem);
        elem = if elem == 8 { 1 } else { elem + 1 };
    });
    assert_eq!(elem, 3);
    assert_eq!(message, msg);
}
