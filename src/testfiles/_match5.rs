pub fn main () {
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => println!("Not 3 "),
    }

    // 放弃了穷举的可能
    if let Some(3) = v {
        println!("This is three");
    } else {
        println!("Other than three");
    }

}