use std::collections::HashMap;

pub fn main () {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // 如果这里不克隆将会报错， 因为移动了 field_name, field_value 的值
    map.insert(field_name.clone(), field_value.clone());

    println!("{}:{}", field_name, field_value);
}