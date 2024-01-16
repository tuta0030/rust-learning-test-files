use std::net::IpAddr;

pub fn main () {
    // 127.0.0.1 是个合理的 ip 地址，解析之后绝对不会出现恐慌，所以可以使用 unwrap
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}