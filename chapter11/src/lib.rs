pub mod difference;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
#[ignore]
fn another() {
    panic!("Make this test fail")
}

#[test]
fn another2() {
    assert_ne!(2 + 2, 3);
}

pub fn add_two_integers(x: i32, y: i32) -> i32 {
    x + y
}
