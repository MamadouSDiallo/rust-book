fn main() {
    let g = temp_convert(32.0, 'f');
    println!("The sum is {}", g);

    for n in 0..20 {
        println!("The fibonacci number for {} is {}", n, fibonacci(n));
    }
}

fn temp_convert(temperature: f32, base: char) -> f32 {
    if base == 'C' {
        temperature * (9.0 / 5.0) + 32.0
    } else if base == 'F' {
        (temperature - 32.0) * (5.0 / 9.0)
    } else {
        println!("Only conversions between Celsuis (C) and Fahnrenheit (F) are possible.");
        -9999.0
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else if n > 1 {
        fibonacci(n - 1) + fibonacci(n - 2)
    } else {
        panic!("n must be a unsigned integer!")
    }
}
