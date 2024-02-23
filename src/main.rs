#[allow(unused)]
fn lesson1() {
    enum IpAddrKind {
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
        fn call(&self) {
            println!("call method");
        }
    }

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

#[allow(unused)]
fn lesson2() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    fn roll() {
        let dice_roll = 9;
        match dice_roll {
            3 => println!("THREE"),
            7 => println!("SEVEN"),
            _ => (),
        }
    }

    roll();
}

#[allow(unused)]
fn lesson3() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;

    // match
    match config_max {
        Some(_) => println!("TRUE"),
        _ => count += 1,
    };

    // if let
    if let Some(max) = config_max {
        println!("TRUE");
    } else {
        count += 1;
    };
}

fn main() {
    lesson3();
}
