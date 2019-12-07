// Understanding OWNERSHIP

fn main() {
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    //s2.push_str(&s1);
    println!("s2 = {} is the same as s1 = {}", s2, s1);

    let n1 = 6;
    let n2 = n1;
    println!("n1 = {} is equal to n2 = {}", n1, n2);

    take_ownership(s);
    //s.push_str(&s1); error: ^ value borrowed here after move

    makes_copy(n1, n2);
    println!("n1 and n2 sum to {} \n", n2 + n1);

    let s3 = gives_ownership();
    let s4 = takes_and_gives_ownership(s3);
    println!("We have we regain owership of s3 with s4 = '{}'", s4);

    let (mut s4, s4_length) = calculate_length(s4);
    println!("The length of s4 = '{}' is {}", s4, s4_length);

    let s4_length2 = calculate_length2(&s4);
    println!(
        "Using the reference method, the length of s4 = '{}' is {}",
        s4, s4_length2
    );

    change_string(&mut s4);

    {
        let r1 = &mut s4;
        println!("We have a reference {}", r1);
        r1.push_str(" Eve more text!");
    }
    {
        let r2 = &mut s4;
        println!("We have a reference {}", r2);
    }

    println!("What is in s4 at this point? \n s4 is '{}' \n", s4);

    let sentence = String::from("Hello World, this is your friend RUST!");
    let world_one = first_word(&sentence);
    println!("The first world of '{}' is '{}'", sentence, world_one)
}

// Functions
fn take_ownership(some_string: String) {
    println!("You passed this string: '{}'", some_string);
}

fn makes_copy(number1: u32, number2: u32) {
    println!("The some is equal to {}", number2 + number1);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello from fn!");
    some_string
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let s_size = s.len();
    (s, s_size)
}

fn calculate_length2(s: &String) -> usize {
    //s.push_str("Some additional text"); cannot borrow as mutable
    let s_size = s.len();
    s_size
}

fn change_string(s: &mut String) {
    s.push_str(" Some additional text");
    println!("This is the new string: '{}'", s)
}

fn first_word(sentence: &String) -> &str {
    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[0..i];
        }
    }
    &sentence[..]
}
