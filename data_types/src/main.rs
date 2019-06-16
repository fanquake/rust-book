fn main() {
    destructuring();
    tuple_access();
    five_threes();
    array_access();
}

fn floats() {
    let x = 2.0; // f64

    let y: f32 = 3.0; //f32
}

fn operations() {
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;
}

fn bools() {
    let t = true;

    let f: bool = false;
}

fn chars() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

fn destructuring() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

fn tuple_access() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The values are: {}, {} & {}", five_hundred, six_point_four, one);
}

fn five_threes() {
    let a = [3,3,3,3,3];
    let b: [i32; 5] = [3,3,3,3,3];
    let c = [3; 5];
    
    println!("Three ways to do three: \n{:?}\n{:?}\n{:?}", a,b,c);
}

fn array_access() {
    let a = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];
    // runtime error
    //let third = a[99];
}