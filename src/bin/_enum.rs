pub fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(four);
    route(six);
    route(IpAddrKind::v6);
}

fn route (ip_kind: IpAddrKind) {
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind {
    v4,
    v6,
}