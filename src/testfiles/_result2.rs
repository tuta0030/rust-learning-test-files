use std::{fs::File, io::ErrorKind};

pub fn main () {
    let f = File::open("hellot.txt");

    // 旧版 使用 match 创建的
    // let f:File = match f {
    //     Ok(f) => f,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("Hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {:?}", e),
    //         },
    //         other_error => panic!("Error opening the file: {:?}", other_error),
    //     }
    // };

    let f = File::open("Hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello.txt").unwrap_or_else(|error| {
                panic!("Error opening file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });
}