// 3. To use the IpAddr type provided by the standard library
use std::fmt;
use std::net::IpAddr;

struct MyIpAddr(IpAddr);

impl fmt::Display for MyIpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let home = MyIpAddr(IpAddr::from([127, 0, 0, 1]));
    let loopback = MyIpAddr(IpAddr::from([0, 0, 0, 0, 0, 0, 0, 1]));

    println!("home is: {}", home);
    println!("loopback is: {}", loopback);
}


/* 
// 2. more concise more concise way: representing the same concept using just an enum 
use std::fmt;

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddr::V4(a, b, c, d) => write!(f, "{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(s) => write!(f, "{}", s),
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home is: {}", home);
    println!("loopback is: {}", loopback);
}
 */

/* 
// 1. Defining an Enum
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn route(ip_kind: IpAddrKind) {}
 */