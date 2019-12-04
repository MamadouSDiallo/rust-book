fn main() {
    let x = 5;
    println!("The value of x is {}", x);

    another_fn(x);

    a_second_fn(5, 33);

    let g = a_third_fn(55.0, 32.0);
    println!("The sum is {}", g)
}

fn another_fn(x: u32) {
    println!("The variable passed to the function is {}", x);
}

fn a_second_fn(x: u32, y: u32) {
    println!("You provided {} and {}", x, y);
}

fn a_third_fn(x: f32, y: f32) -> f32 {
    x + y
}
