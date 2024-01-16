pub fn main () {
    let s = String::from("赵钱孙李");
    let len = s.len();
    let chars = s.chars();
    let c_len = chars.clone().count(); // 调用 count 会把 chars 移动然后消耗掉，所以需要克隆一份

    println!("字符串的长度: {}", len);
    println!("字符串包含的文字长度: {}", c_len);

    for (index, each_char) in chars.enumerate() {
        println!("第 {} 个字是：{}", &index + 1,&each_char);
    }
}