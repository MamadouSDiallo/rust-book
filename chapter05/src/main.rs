// USING STRUCTS TO STRUCTURE RELATED DATA

fn main() {
    let mut user1 = User {
        username: String::from("Rust"),
        email: String::from("rust@rust.org"),
        sign_in_count: 0,
        active: true,
    };
    user1.sign_in_count = 1;
    println!(
        "The first user is: {} with {} number of connections. \n",
        user1.username, user1.sign_in_count
    );

    let username2 = String::from("Rust2");
    let email2 = String::from("rust2@rust.org");
    let user2 = build_user(username2, email2);

    println!(
        "The second user is '{}' and the associated email is '{}'. \n",
        user2.username, user2.email
    );

    let green = Color(0, 255, 0);
    let origin = Point(0, 0, 0);
    println!(
        "The point {:#?} is green with sRBG equal to {:#?}. \n",
        origin, green
    );

    let rect1 = Rectangle {
        name: String::from("rectangle1"),
        length: 3.0,
        width: 5.0,
    };
    println!(
        "The area of {:?} is equal to {} same as {}. \nThe perimeter is {}.",
        rect1,
        rect1.area_method(),
        area_fct(&rect1),
        rect1.perimeter()
    );
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

#[derive(Debug)]
struct Rectangle {
    name: String,
    length: f64,
    width: f64,
}

impl Rectangle {
    fn perimeter(&self) -> f64 {
        self.length + self.width
    }
    fn area_method(&self) -> f64 {
        self.length * self.width
    }
}
fn area_fct(rect: &Rectangle) -> f64 {
    rect.length * rect.width
}
