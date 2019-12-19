fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let nb_mean = numbers_mean(&numbers);
    println!("The mean of {:?} is {}", numbers, nb_mean)
}

fn numbers_mean(v: &Vec<i64>) -> i64 {
    let mut total = 0;
    for k in v {
        total += k
    }
    let len = v.len() as i64;
    let mean = total / len;
    mean
}
