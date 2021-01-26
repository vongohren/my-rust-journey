#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


pub fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    ipprint(home);
    ipprint(loopback);
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}",ip_kind)
}


fn ipprint(ip: IpAddr) {
    println!("{:?}",ip)
}
