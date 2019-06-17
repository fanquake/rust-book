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

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T,U> {
    x: T,
    y: U,
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


fn main() {
    let number_list = vec![34,50,25,100,65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // wont work
    //let wont_work = Point { x: 1, y: 2.0 };

    let both_integer = Point { x: 1, y: 10 };
    let both_float = Point { x: 1.2, y: 4.3 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("px. = {}", p.x());
}

// performance using generics

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn some_main() {
    let integer = Some(5); // Option_i32::Some(5);
    let float = Some(5.0) // Option_f64::Some(5.0);
}
