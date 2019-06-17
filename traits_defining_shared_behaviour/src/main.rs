// trait: group together method signatures that define behaviours

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// it's possible to specify a return value by traits implemented.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Some content!"),
        reply: false,
        retweet: false,
    }
}

pub trait Display {}

// this accepts any parameter that implements the Summary trait
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// more verbose way of writing the above
pub fn notify_verbose<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// multiple trait bounds with + syntax
pub fn something(item: impl Summary + Display) {}
pub fn trait_bound_on_generic<T: Summary + Display>(item: T) {}

// where clauses
// rather than writing this..
pub trait Debug {}
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 { 0i32 }

// write this
fn some_function_clean<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0i32
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("writer: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Some fancy headline"),
        location: String::from("Some location"),
        author: String::from("Whoever"),
        content: String::from("Some interesting content"),
    };

    println!("New article available! {}", article.summarize());

    find_largest();
}


// Fixing largest() using trait bounds

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_largest() {
    let number_list = vec![34,50,25,100,65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// using trait bounds to implement methods

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}