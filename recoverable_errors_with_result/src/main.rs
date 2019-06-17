use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let f = File::open("hello.txt");

    // // match => match => match...
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         },
    //         other_error => panic!("There was a problem opening the file: {:?}", other_error),
    //     },
    // };

    main3();
}

// same behaviour as main(), but far less match =>
fn main2() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}


// shortuts for panic! on Error

fn main3() {
    //let f = File::open("hello.txt").unwrap();

    // expect lets you choose the panic! error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}


// Propagating errors

use std::io::{self, Read};

// return Result<T,E>
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// shortcut for propogating errors
// ? is used to eliminate boilerplate error handling code
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// even shorter again...
fn read_username_from_file_shorter_x2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// even even shorter
fn read_username_from_file_shorter_x2_some_more() -> Result<String, io::Error> {
    // opens a file, createa a new String, reads the contents of the file,
    // puts that into the new string, returns the string
    std::fs::read_to_string("hello.txt")
}