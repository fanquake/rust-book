fn main() {
    let some_u8_value = Some(0u8);

    // rather than using
    match some_u8_value {
        Some(3) => println!("three"),
        _ => ()
    }

    // use if let
    if let Some(3) = some_u8_value {
        println!("three!");
    } else {
        println!("Do some other thing!");
    }

    let coin = Coin::Quarter(State::Alaska);
    dont_be_so_verbose(coin);
}

#[derive(Debug)]
enum State {
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Quarter(State)
}

fn dont_be_so_verbose(coin: Coin) {

    let mut count = 0;

    // match coin {
    //     Coin::Quarter(state) => println!("State quater from {:?}!", state),
    //     _ => count += 1
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}