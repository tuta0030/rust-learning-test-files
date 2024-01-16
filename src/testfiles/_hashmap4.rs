use std::collections::HashMap;

pub fn main () {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // 传入引用，不会消耗掉变量，后面可以接着使用
    map.insert(&field_name, &field_value );

    println!("{}: {}", field_name, field_value);
}