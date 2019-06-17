fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();
    // equivalent to
    let s = String::from("initial contents");

    // can put just about anything into strings
    let hello = String::from("السلام عليكم");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("Здравствуйте");

    let mut updatable = String::from("foo");
    updatable.push_str("bar");
    println!("{}", updatable);

    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 has been moved here, and can no longer be used
    println!("{}, {}", s2, s3);

    // if you need to concatenate multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // dont do this
    //let s = s1 + "-" + &s2 + "-" + &s3;
    //println!("{}", s);

    // do this instead...
    // format! also doesn't take any ownership of params
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // no string indexing in Rust
    let s1 = String::from("hello");
    //let h = s1[0]; // not allowed!


    // string slicing instead
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}",s); // first 4 bytes, which is Зд (2 bytes each)


    // trying to slice a single byte will panic
    // thread 'main' panicked at 'byte index 1 is not a char boundary; 
    // it is inside 'З' (bytes 0..2) of `Зд`', src/libcore/str/mod.rs:2027:5
    //println!("{}", &s[0..1])

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // म
    // स
    // ्
    // त
    // े

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // 224
    // 164
    // 168
    // 224
    // 164
    // 174
    // 224
    // 164
    // 184
    // 224
    // 165
    // 141
    // 224
    // 164
    // 164
    // 224
    // 165
    // 135
}
