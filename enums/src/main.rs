/*
//TODO:WORK ON THIS
ENUMS
    - 
*/
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Ipv4Addr {
    zero: u8,
    one: u8,
    two: u8,
    three: u8,
}

impl Ipv4Addr {
    fn new(zero: u8, one: u8, two: u8, three: u8) -> Self {
        Self {
            zero,
            one,
            two,
            three
        }
    }
}

#[derive(Debug)]
struct Ipv6Addr {
    main: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum IpAddr2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    ip: String
}


///////////////////////////////////////////////////////////
#[derive(Debug)]
enum Message {
    Write(String),
    Write2(Text),
    Quit
}

#[derive(Debug)]
enum Text {
    Content(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(deneme) => println!("{deneme}"),
            Message::Write2(text) => {
                match text {
                    Text::Content(deneme) => println!("{deneme}")
                }
            }
            Message::Quit => println!("Quitting"),
        }
    }
}

fn main() {
    let ip = IpAddr {
        kind: IpAddrKind::V4,
        ip: String::from("127.0.0.1")
    };

    let msg = Message::Write(String::from("Deneme"));

    msg.call();

    let msg = Message::Quit;

    msg.call();

    let msg2 = Message::Write2(Text::Content(String::from("Hahahahhaa")));

    msg2.call();

    if let Message::Write2(Text::Content(val)) = msg2 {
        println!("{val}");
    } else {
        println!("no");
    }

    let dns_resolver = IpAddr2::V4(Ipv4Addr::new(127, 0, 0, 1));

    let abc = Option::Some(5);

    println!("{:?}", plus_one(abc));

    println!("{:?} {:?} {:?}", ip, dns_resolver, msg);
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}