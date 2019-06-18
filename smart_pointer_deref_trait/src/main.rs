struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // deref will return a reference to the value we want to access
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}


fn main() {
    let x = 5; // x hold and i32 value
    let y = &x; // y is a reference to x.

    assert_eq!(5, x);
    // dereference: follow the reference to the data
    // *(y.deref())
    assert_eq!(5, *y);

    // The above should be the same as this:
    let a = 5;
    let b = MyBox::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);

    assert_eq!(*y, *b);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // if Rust didn't do deref coersion, the above would be:
    // hello(&(*m)[..]);
    // where we dereference with (*m), then use &[..] to take a string slice
    // (to match the function signature)
}
