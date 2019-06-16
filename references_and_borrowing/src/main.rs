fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //change(&s1);

    let mut s2 = String::from("hello");

    mutable_reference(&mut s2);
    println!("{:?}", s2);

    scoping_reference()

    //let reference_to_nothing = dangle()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(some_string: &String) {
//     // cannot modify a borrowed value
//     some_string.push_str(", world");
// }

fn mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn not_allowed() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     // cannot have a second mutable reference to the same data in the same scope
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

// cant mix and match references
// fn not_ok() {
//     let mut s = String::from("hello");

//     let r1 = &s; // fine
//     let r2 = &s; //fine
//     let r3 = &mut s; // NO

//     println!("{}, {} and {}", r1, r2, r3);
// }

fn scoping_reference() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    // no longer use r1 or r2

    let r3 = &mut s;
    r3.push_str(", world!");
    println!("{}", r3);
}

// fn dangle() -> &String {
//     // s is created inside the dangle scope
//     let s = String::from("hello");

//     &s
// } // s is deallocated, so dangle cannot pass out a reference to it

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}