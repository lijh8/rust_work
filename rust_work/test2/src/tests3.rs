use super::add2::*;
use super::*;

#[test]
fn ut_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn ut_add2() {
    assert_eq!(add2(1, 2), -3); // false test
}
