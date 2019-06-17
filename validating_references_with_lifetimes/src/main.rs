fn main() {
    // will not compile
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    let string1 = String::from("LONG STRING IS LONG ");
    //let result;
    {
        let string2 = String::from("SHORT");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    //println!("The longest string is {}", result);

    main2();
}

// &i32 // a reference
// &'a i32 // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// expressing that all the references in the parameters and the return value
// must have the same lifetime.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn other_longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// lifetime annotations in struct definitions

// an instance of ImportantExcerpt can't outlive the reference it holds in its
// part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main2() {
    let novel = String::from("Call me Ishmael. Some years ago..");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// generic type params, trait bounds and lifetimes together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Annoucement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
