#[derive(Debug)]
enum IpAddrType {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrComplexType {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrComplexType {
    fn call(&self) {
        println!("Calling method on enum {:?}",self);
    }
}

pub fn main() {
    let home = IpAddrType::V4(String::from("127.0.0.1"));
    let loopback = IpAddrType::V6(String::from("::1"));
    let home_complex = IpAddrComplexType::V4(127, 0, 0, 1);
    let loopback_complex = IpAddrComplexType::V6(String::from("::1"));

    home_complex.call();

    println!("Simple: {:?}, Complex: {:?}", home, home_complex);

    println!("Simple: {:?}, Complex: {:?}", loopback, loopback_complex);

}