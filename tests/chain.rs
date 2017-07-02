#[macro_use]
extern crate o3;

use o3::iter::wrap::Wrap;
use o3::comb::chain::Chain;


#[test]
fn chain_integers() {
    let first = (1..9).wrap();
    let second = (1..3).wrap();

    let the_chain = first.chain(|_| second);
    let msg = "This is the end";
    let mut elem = 1;
    let message = each!(the_chain => i in {
        assert_eq!(i, elem);
        elem = if elem == 8 { 1 } else { elem + 1 };
    } then {
        msg
    });
    assert_eq!(elem, 3);
    assert_eq!(message, msg);
}
