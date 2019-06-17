// Vector: store a variable number of values next to each other

// String: store a collection of characters

// HashMap: associate a value with a particular key

fn main() {
    // new vector to hold i32s
    // specify type because we haven't given it any values
    let v: Vec<i32> = Vec::new();

    // vec! macro for convinient way to create new vectors
    // stored type is inferred
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(4) {
        Some(fourth) => println!("The fourth element is {}", fourth),
        None => println!("There is no fourth element"),
    }

    // can't have mutable and immutable references in the same scope
    let mut v = vec![1, 2, 3, 4];
    let first = &v[0]; // immutable
                       //v.push(6); // mutable
    println!("The first element is {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // dereference operator
        *i += 50;
        println!("{}", i);
    }

    // use an Enum to store multiple types
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}
