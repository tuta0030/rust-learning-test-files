use std::fs::{File, read_to_string};
use std::io;
use std::io::Read;

fn read_username_from_file () -> Result<String, io::Error> {

    // 最终简化的代码
    let mut s = String::new();
    let mut f = File::open("Hello.txt")?.read_to_string(&mut s)?;


    // let mut f = File::open("Hello.txt")?; // 加 ？ 等于下面的代码
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();
    // f.read_to_string(&mut s)?; //等于下main的代码
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    Ok(s)
}

pub fn main () {
    println!("Hello World");
    let result = read_username_from_file();
}