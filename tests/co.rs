#![feature(never_type)]

#[macro_use]
extern crate o3;

use o3::iter::wrap::Wrap;
use o3::map::ret::MapReturn;
use o3::map::yld::MapYield;

#[test]
fn full_each() {
    let bart = (3..10).wrap().map_return(|_| "I'm done!");
    let mut cnt = 3;
    let large_num = 1000;
    let message = each!(bart => i in {
        assert_eq!(i, cnt);
        cnt += 1;
        if large_num < 5 { break; }
    } then with msg in {
        String::from(msg) + " Yayy!"
    } else {
        String::from("I got broken!")
    });
    assert_eq!(cnt, 10);
    assert_eq!(message, String::from("I'm done! Yayy!"));
}

#[test]
fn full_each_with_break() {
    let bart = (3..10).wrap().map_return(|_| "I'm done!");
    let mut cnt = 3;
    let message = each!(bart => i in {
        assert_eq!(i, cnt);
        cnt += 1;
        break;
    } then with msg in {
        msg
    } else {
        "I got broken!"
    });
    assert_eq!(cnt, 4);
    assert_eq!(message, "I got broken!");
}

#[test]
fn full_each_with_capture_patterns() {
    let bart = (3..10).wrap().map_yield(|i| (i, 10)).map_return(|_| ("I'm done!", 10));
    let mut cnt = 3;
    let large_num = 1000;
    let (message, lim) = each!(bart => (i, lim) in {
        assert_eq!(i, cnt);
        assert_eq!(lim, 10);
        cnt += 1;
        if large_num < 5 { break; }
    } then with (msg, lim) in {
        (String::from(msg) + " Yayy!", lim)
    } else {
        (String::from("I got broken!"), -1)
    });
    assert_eq!(cnt, 10);
    assert_eq!(message, String::from("I'm done! Yayy!"));
    assert_eq!(lim, 10);
}

#[test]
fn no_with() {
    let bart = (3..10).wrap().map_return(|_| "I'm done!");
    let mut cnt = 3;
    let large_number = 1000;
    let message = each!(bart => i in {
        assert_eq!(i, cnt);
        cnt += 1;
        if large_number < 5 { break; }
    } then {
        "At last!"
    } else {
        "I got broken!"
    });
    assert_eq!(cnt, 10);
    assert_eq!(message, "At last!");
}

#[test]
fn no_else() {
    let bart = (3..10).wrap().map_return(|_| "I'm done!");
    let mut cnt = 3;
    let message = each!(bart => i in {
        assert_eq!(i, cnt);
        cnt += 1;
    } then with msg in {
        String::from(msg) + " Yayy!"
    });
    assert_eq!(cnt, 10);
    assert_eq!(message, String::from("I'm done! Yayy!"));
}

#[test]
fn no_with_else() {
    let bart = (3..10).wrap().map_return(|_| "I'm done!");
    let mut cnt = 3;
    let message = each!(bart => i in {
        assert_eq!(i, cnt);
        cnt += 1;
    } then {
        "At last!"
    });
    assert_eq!(cnt, 10);
    assert_eq!(message, "At last!");
}


#[test]
fn no_then() {
    let bart = (3..10).wrap().map_return(|_| "I'm done!");
    let mut cnt = 3;
    let message = each!(bart => i in {
        assert_eq!(i, cnt);
        cnt += 1;
        if cnt > 100 { break; }
    } else {
        "bogus"
    });
    assert_eq!(cnt, 10);
    assert_eq!(message, "I'm done!");
}

#[test]
fn no_then_with_break() {
    let bart = (3..).wrap().map_return(|_| unreachable!("An infinite series should not return"));
    let mut cnt = 3;
    let message = each!(bart => i in {
        assert_eq!(i, cnt);
        cnt += 1;
        if cnt >= 20 { break; }
    } else {
        "I got broken!"
    });
    assert_eq!(cnt, 20);
    assert_eq!(message, "I got broken!");
}

#[test]
fn no_then_else() {
    let bart = (3..10).wrap().map_return(|_| "I'm done!");
    let mut cnt = 3;
    let message = each!(bart => i in {
        assert_eq!(i, cnt);
        cnt += 1;
    });
    assert_eq!(cnt, 10);
    assert_eq!(message, "I'm done!");
}
