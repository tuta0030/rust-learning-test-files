pub fn main () {
    let v = 0u8;
    match v {
        1 => println!("One"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Not 1..3 "),
    }
}