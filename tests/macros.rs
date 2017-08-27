#[macro_use]
extern crate o3;

#[test]
fn each_works() {
    use o3::iter::wrap::Wrap;

    let mut v = vec![];
    each!((3..10).wrap() => i in {
        v.push(i);
    });

    assert_eq!(v, [3, 4, 5, 6, 7, 8, 9]);
}
