#[macro_use]
extern crate o3;

use o3::iter::wrap::Wrap;
use o3::map::ret::MapReturn;
use o3::race::Race;
use o3::either::Either;

#[test]
fn race() {
    let first = (0..5).wrap().map_return(|_| "first");
    let second = (0..10).wrap().map_return(|_| "second");
    let mut trace = vec![];
    let loser = each!(first.race(second) => i in {
        trace.push(i);
    } then with result in {
        match result {
            Either::Former((message, latter)) => {
                assert_eq!(message, "first");
                latter
            },
            _ => panic!("The first one should finish first")
        }
    });
    assert_eq!(trace, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4]);

    trace.clear();
    let message = each!(loser => i in {
        trace.push(i);
    });

    assert_eq!(trace, [5, 6, 7, 8, 9]);
    assert_eq!(message, "second");
}
