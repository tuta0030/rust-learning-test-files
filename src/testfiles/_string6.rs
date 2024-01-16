pub fn main () {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    // 使用 + 连接的两个字符串， 第一个是作为拥有所有权的 string 进去的，调用完它之后，会失效
    // let s3 = s1 + &s2;

    let s3 = format!("{}{}", s1, s2);

    println!("{}", s3);
    println!("{}", s1);
    println!("{}", s2);
}