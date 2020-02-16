#[allow(unused_variables)]

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    //let absent_number: std::option::Option<i32> = None;

    let some_number_plus_one = plus_one(some_number);
    println!(
        "Some number {:?} plus one is equal to {:?}",
        some_number, some_number_plus_one
    );
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("The method will be implemented here");
    }
}

fn plus_one(i: Option<i32>) -> Option<i32> {
    match i {
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
