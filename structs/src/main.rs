struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@something.com"),
        username: String::from("somethingcool"),
        active: true,
        sign_in_count: 42,
    };

    user1.sign_in_count += 1;

    println!("{}: {}", user1.username, user1.sign_in_count);

    let user2 = build_user("anothercoolemail@somewhere.com".to_string(), "yea_nah".to_string());
    println!("{}, {}", user2.username, user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}

fn build_struct_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn struct_update(user: User) -> User {
    User {
        email: String::from("someoneelse@something.com"),
        username: String::from("somethingelsecool"),
        active: user.active,
        sign_in_count: user.sign_in_count + 1,
    }
}

fn struct_update_shorthand(user: User) -> User {
    User {
        email: String::from("someoneelse@something.com"),
        username: String::from("somethingelsecool"),
        ..user
    }
}

struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

fn random_structs() {
    let black = Colour(0,0,0);
    let white = Colour(255,255,255);

    let origin = Point(0,0,0);
}

const SOMETHING_IMPORTANT: i32 = 100_000_000;