fn main() {
    const MAX_POINTS: u32 = 100_000;

    println!("Max points is: {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("Shadowed x is: {}", shadow(x));
}

fn shadow(x_value: i32) -> i32 {
    let x_value = x_value + 1;

    let x_value = x_value * 2;

    return x_value
}

fn space_count() {
    let spaces = "    ";
    let spaces = spaces.len();
}