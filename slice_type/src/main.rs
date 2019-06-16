fn main() {
    println!("Hello, world!");

    array_slice();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let start = &s[0..2];
    // same as
    let start = &s[..2];

    let len = s.len();

    let end = &s[3..len];
    // same as
    let end = &s[3..];

    let entire = &s[0..len];
    // same as
    let entire = &s[..];
}

fn better_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//fn more_general_first_word(s: &str) -> &str {}

fn array_slice() {
    let a: [i32; 4] = [1,2,3,4];
    let slice = &a[..2];
    println!("{:?}", slice);
}
