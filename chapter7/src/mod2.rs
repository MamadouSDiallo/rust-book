pub fn mod2_fn1() {
    println!("This is function 1 from module 2");
}

pub fn mod2_fn2(a: i32, b: i32) -> i64 {
    let sum = a + b;
    sum as i64
}
