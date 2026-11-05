/* -------------------------------------------------------------------------- */
/* Enums                                                                      */
/* -------------------------------------------------------------------------- */
//  Custom data type enumerating possible variants.

#[derive(Debug)]
enum IpAddrKind {
    // Variants namespaced under enum name
    // To invoke, use [enum name]::[variant]
    V4,
    V6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[derive(Debug)]
enum IpAddr {
    // - rather than enum inside struct -> can put data directly into each variant
    // - different variants can now have different types + amounts of associated
    //   data
    // - can put any kind of data inside an enum variant
    // - this makes each variant into a function that constructs an instance of
    //   the enum
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?} {:?}", four, six);

    /* ---------------------------------------------------------------------- */

    // using struct IpAddr
    // let home = IpAddr {
    //     kind: four,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: six,
    //     address: String::from("::1"),
    // };

    /* ---------------------------------------------------------------------- */

    // using enum IpAddr
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}\n{:?}", home, loopback);

    /* ---------------------------------------------------------------------- */

    let m = Message::Write(String::from("hello"));
    m.call();
}
