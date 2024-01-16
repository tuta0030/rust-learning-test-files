use std::collections::HashMap;
// HashMap 中的数据必须是同构的， key 需要是同一类型， value 也需要是同一类型

pub fn main () {
    let mut scores  = HashMap::new();
    scores.insert(1, String::from("Banana"));
    scores.insert(2, String::from("Apple"));

    let mut ss = String::new();

    for (k, v) in scores {
        ss.push_str(&k.to_string());
        ss.push(':');
        ss.push_str(&v.to_string());
        ss.push_str("; ");
    }

    println!("整个HashMap转换成自定义的字符串之后是： {}", ss);
}