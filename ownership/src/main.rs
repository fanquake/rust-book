fn main() {
    println!("Ownership");

    let mut s = String::from("hello");

    s.push_str(", world!"); // append a literal to a string

    println!("{}", s);

    //bad();

    clone();

    stack_only_copy();

    ownership();

    return_tuple();
}

fn string_copy() {
    // new string created
    // ptr (to heap), len, capacity onto stack
    let s1 = String::from("hello");

    // second string created
    // heap data is not copied, only data on the stack: ptr, len, capacity
    // s1 is "moved" into s2, s1 is invalidated.
    let s2 = s1;
}

fn bad() {
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world", s1); // unhappy borrow checker
}

fn clone() {
    let s1 = String::from("hello");
    // heap data does get copied
    let s2 = s1.clone();
    
    // s1 & s2 now have ptr, len & cap on the stack
    // each pointing to their own copy of the data on the heap

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_only_copy() {
    // integers have known sizes at compile time, and are stored completely on the stack
    // thus, copies of the actual values are quick to make
    let x = 5;
    let y = x;
    let z = x.clone(); // should be equivalent to the line above

    println!("x = {}, y = {}, z = {}", x, y, z);
}

fn ownership() {
    let s = String::from("hello");

    takes_ownership(s);

    // compile time error, s was moved into takes_ownership
    // thus is no longer valid here
    // println!("{}", s);

    let x = 5;

    makes_copy(x);

    // perfectly fine to do something with x here because i32 is Copy
    println!("{}", x + 1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn return_values_and_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    // implicitly returned
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn return_tuple() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len)
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}