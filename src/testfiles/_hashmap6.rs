use std::collections::HashMap;

pub fn main () {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
}