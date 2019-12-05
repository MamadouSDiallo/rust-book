fn main() {
    let g = temp_convert(32.0, 'f');
    println!("The sum is {}", g)
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
