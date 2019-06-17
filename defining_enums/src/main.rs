enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// more concise

enum IpAddrEmbeddedData {
    V4(String),
    V6(String),
}

enum IpAddrDiff {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // some method body
    }
}


// enum Option<T> {
//     Some(T),
//     None,
// }


fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home1 = IpAddrEmbeddedData::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddrEmbeddedData::V6(String::from("::1"));

    let home2 = IpAddrDiff::V4(127, 0, 0, 1);
    let loopback2 = IpAddrDiff::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello!"));
    m.call();

    let some_numer = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

}

// fn route(ip_kind: IpAddrKind) {}
// route(IpAddrKind::V4)
// route(IpAddrKind::V6)
