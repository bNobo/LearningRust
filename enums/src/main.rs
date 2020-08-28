fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrV2::V4(127, 0, 0, 0);

    let loopback = IpAddrV2::V6(String::from("::1"));

    println!("{:?} {:?}", home, loopback);

    home.ping();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x = 5;
    let y = Some(6);
    let z = Some(7);

    let sum: i32 = x + match y {
        Some(t) => t,
        None => 0,
    };

    if let y = Some(6) {
        println!("6");
    }

    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

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

#[derive(Debug)]
enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrV2 {
    fn ping(&self) {
        println!("ping called on {:?}", self);
    }
}
