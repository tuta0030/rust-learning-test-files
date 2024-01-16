pub fn main () {
    let w = "कार्यक्रम लेखन";
    let w_len = w.len();
    let c = w.chars();
    let c_len = c.clone().count();

    for (i, b) in w.bytes().enumerate() {
        println!("第 {} 个 byte 是 {} ", i , b);
    }
    for (i, c) in c.enumerate() {
        println!("第 {} 个 字符是： {}", i , c);
    }

    println!("这个字符串 {} 的长度是:{}", w, w_len);
    println!("这个字符 {} 的长度是:{}", w, c_len);
}