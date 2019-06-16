fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    in_in_let();

    //forever();

    break_loop();

    while_loop();

    for_loop();

    range_loop();
}

fn in_in_let() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

fn forever() {
    loop {
        println!("again!");
    }
}

fn break_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("the value is: {}", elem);
    }
}

fn range_loop() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!");
}