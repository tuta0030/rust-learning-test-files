use std::{fs::File, error::Error};

pub fn main () -> Result<(), Box<dyn Error>>{
    //                        ^ Box<dyn Error> 代表所有可能的错误对象
    let f = File::open("Hello.txt")?;
    Ok(())
}