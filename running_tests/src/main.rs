fn main() {
    println!("Hello, world!");
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_withh_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100))
    }

    #[test]
    #[ignore]
    fn some_expensive_test() {
        // sooo long
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}