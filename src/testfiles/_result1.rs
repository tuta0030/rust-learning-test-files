use std::fs::File;

pub fn main () {
    let f = File::open("Hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("打开文件时发生错误 {:?}", err);
        }
    };
}