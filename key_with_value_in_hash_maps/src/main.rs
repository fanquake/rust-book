use std::collections::HashMap;

fn main() {

    // like vectors, hash maps store their data on the heap
    // in a hashmap, all keys and values must have the same type
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    // alternate construction
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let inital_scores = vec![10, 50];

    // _ type annotation is required
    let alternate_scores: HashMap<_,_> = teams.iter().zip(inital_scores.iter()).collect();
    println!("{:?}", alternate_scores);

    // access
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score); // Some(10)

    // iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores.get("Blue")); // Some(25)

    // only inserting if a key has no value
    scores.remove(&String::from("Yellow"));
    // blue is already 25
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
