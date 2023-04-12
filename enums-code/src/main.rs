fn main() {
    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Some(5);
    let new_number = plus_one(None);
    println!("{:?} Option New Number", new_number);
    enum ip_addr_kind {
        v4,
        v6,
    }
    let four = ip_addr_kind::v4;
    let six = ip_addr_kind::v6;
    fn route(ip_kind: ip_addr_kind) {}
    route(ip_addr_kind::v4);

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    enums_new();
    coins();
}

fn enums_new() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct''
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    impl Message {
        fn call(&self) {
            // method body would be defined here
            QuitMessage;
            MoveMessage { x: 1, y: 2 };
            WriteMessage(String::from("hello"));
            ChangeColorMessage(1, 2, 3);
        }
    }

    let m = Message::Write(String::from("hello"));
    // let m = Message::Move { x: 10, y: 10 };
    // let m = Message::Quit;
    // let m = Message::ChangeColor(1, 2, 3);
    // let new = m.call("hello"");
    // println!("{:?}", new);
}

fn coins() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    value_in_cents(Coin::Penny);
    let val = value_in_cents(Coin::Dime);
    println!("{}", val);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
