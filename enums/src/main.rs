enum IpAddressKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        println!("I'm inside call");
    }
}

fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    let home = IpAddressKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddressKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();
}