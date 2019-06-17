enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn bad_plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        //None => None,
    }
}

// Match placeholder
fn placeholder() {
    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // match everything else. 0,2,4,6,8,9...255
    }
}

fn main() {
    println!("Hello, world!");
    let coin = Coin::Quarter(UsState::Alaska);

    println!("The value is {}", value_in_cents(coin));

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
