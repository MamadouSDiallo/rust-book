use chapter11::add_two_integers;
use chapter11::difference::difference;

#[test]
fn test_add() {
    assert!(add_two_integers(3, 5) == 8);
}

#[test]
fn test_difference() {
    assert!(difference(3, 6) == -3);
}
