fn main() {
    test_string_example1();
    test_string_example3();
}


fn test_string_example1 (){
    let mut s = String::from("Hello");
    s.push_str(", World");
    println!("{}", s);

}

// 下面的例子展示了 rust 的 move 操作，把 s1 的栈中的内容 复制到了 s2，同时让 s1 失效，所以叫做 move
// fn test_string_example2 () {
//     let s1 = String::from("Hello");
//     let s2 = s1;

//     println!("{}", s1);

// }


// 下面的例子展示了 clone 函数，
fn test_string_example3 () {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{}, {}", s1, s2);
}
