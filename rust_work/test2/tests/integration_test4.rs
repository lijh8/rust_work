use test2::add;
use test2::add2::add2;

#[test]
fn it_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn it_add2() {
    assert_eq!(add2(1, 2), -3); // false test
}
