fn main() {
    test_example_1();
    // test_example_2();
}


fn test_example_2 () {
    let s = String::from("Hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

}


fn test_example_1 () {
    let s = String::from("Hello world");
    let word_index = first_world(&s);

    println!("{}", word_index);

}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return  &s[..i];
        }
    }
    
    &s[..]
}

