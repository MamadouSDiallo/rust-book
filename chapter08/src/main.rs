fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let v3: Vec<i64> = vec![1, 2, 3];

    println!("{:?}, {:?}, {:?}", v1, v2, v3);

    let mut v: Vec<f64> = Vec::new();

    v.push(5.0);
    v.push(10.);
    v.push(15.0);
    v.push(20.);
    v.push(25.0);
    v.push(30.);

    println!("Vector v holds the following data: {:?}", v);

    let third = v[2];
    println!("The third element is {}", third);

    v.push(8.);

    match v.get(2) {
        Some(third) => println!("This is the third element: {}", third),
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("Value is {}", i);
    }
    for i in &mut v {
        *i += 50.;
    }
    println!("{:?}", v);

    let len = String::from("Hola$%@$#@&@").len();
    println!("{}", len);

    for c in "Hello@#$@41".chars() {
        println!("{}", c)
    }

    for b in "Hello@#$@41".bytes() {
        println!("{}", b)
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 25);

    println!("{:?} \n", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores)
}
