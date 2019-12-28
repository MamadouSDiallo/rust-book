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

    let location = Point {
        x: 5,
        y: 10.1,
        z: 15.5,
    };
    println!("The location is {:#?}", location);
}

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

// fn largest<T: PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

#[derive(Debug, PartialEq, PartialOrd, Mul)]
struct Point<T, U, V> {
    x: T,
    y: U,
    z: V,
}

impl<T, U, V> Point<T, U, V> {
    fn distance_to_origin(self) -> T
    where
        T: std::ops::Mul,
        U: std::ops::Mul,
        V: std::ops::Mul,
    {
        let distance = self.x * self.y * self.z;
        distance
    }
}
