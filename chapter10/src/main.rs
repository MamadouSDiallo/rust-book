use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::ops::Mul;

fn main() {
    let mut i32_list = vec![34, 50, 25, 100, 65];
    let mut i32_result = largest_i32(&i32_list);
    println!(
        "The largest number in array {:?} is {}",
        i32_list, i32_result
    );

    i32_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    i32_result = largest_i32(&i32_list);
    println!(
        "The largest number in array {:?} is {}",
        i32_list, i32_result
    );

    let char_list = vec!['s', 'a', 'b', 'S', 'w', 'f'];
    let char_result = largest_char(&char_list);
    println!(
        "The largest character in {:?} is {}",
        char_list, char_result
    );

    // let t_list = vec![2.04, 1.02, 87.01, 0.54, 54.2];
    // let t_result = largest(&t_list);
    // println!("The largest number in array {:?} is {}", t_list, t_result);

    let location = Point::<f32> {
        x: 5.3,
        y: 10.1,
        z: 15.5,
    };
    println!("The location is {:#?} \n", location);
    println!(
        "The distance to origin is {:#?} \n",
        location.distance_to_origin()
    );

    let seven_chars: &'static str = "abcdefg";
    println!(
        "The largest character in {:} is {:}",
        seven_chars,
        largest(&char_list)
    );

    let p21 = Point2 { x: 4, y: 6.7 };
    let p22 = Point2 { x: 4.3, y: 6 };
    println!("The mixup point is {:?}", p21.mixup(p22));

    let string1 = String::from("abcd");
    let string2 = String::from("abcde");
    println!("The longestt string is {:}", longest(&string1, &string2));
}

// Functions
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point<T>
where
    T: Mul<Output = T>,
{
    fn distance_to_origin(self) -> T {
        let distance = self.x * self.y * self.z;
        distance
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
