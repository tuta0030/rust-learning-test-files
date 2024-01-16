use std::fs::File;

pub fn main () {
    // let f = File::open("Hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Error opening file {:?} ", error)
    //     }
    // };

    // 相当于上面的那些代码，如果成功，返回 值，如果失败，panic
    // let f = File::open("Hello.txt").unwrap();
    let f = File::open("Hello.txt").expect("没有找到文件 Hello.txt");
}