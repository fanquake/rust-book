fn main() {
    println!("Hello, world!");

    another_function(5, 9.99);

    expression();

    let five = five();

    println!("The value of five is: {}", five);

    let incremented = plus_one(0);
    println!("{}", incremented)
}

fn another_function(x: i32, y: f64) {
    println!("The value of x is: {}.", x);
    println!("The value of y is: {}.", y);
}

fn expression() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}