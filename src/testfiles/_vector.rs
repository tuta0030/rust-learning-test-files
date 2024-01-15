pub fn main () {
    let _v1: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];
    let mut _v3  = Vec::new();
    let _v4 = vec![1, 2, 3, 4, 5];

    // 这里注释掉的方法可以让程序 panic 报错
    // let third: &i32 = &_v4 [100];
    // println!("The third element is {}", third);

    // 下面用 match 的方法可以把错误的结果进行处理
    match _v4.get(100) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    _v3.push("Some String");
}