use std::collections::HashMap;

pub fn main () {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = 
        teams.iter().zip(initial_scores.iter()).collect();
    
    for (t, s) in scores {
        print!("{}:", t);
        println!("{}", s);
    }
}