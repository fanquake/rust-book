fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    main_tuples();

    main_structs();
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// V2

fn main_tuples() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// V3
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area3(&rect1)
    );

    display_rectangle(rect1);
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn display_rectangle(rect: Rectangle) {
    // requires #[derive(Debug)] on struct
    // {:?} or {:#?}
    println!("{:?}", rect);
}
