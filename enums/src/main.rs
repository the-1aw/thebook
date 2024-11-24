enum Message {
    Quit,
    Move { x: usize, y: usize },
    Write(String),
    ChangeColor(usize, usize, usize),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move {x} {y}"),
            Message::Write(str) => println!("write {str}"),
            Message::ChangeColor(r, g, b) => println!("change color to {},{},{}", r, g, b),
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cent(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn optional_plus_one(x: Option<usize>) -> Option<usize> {
    match x {
        Some(x) => Some(x + 1),
        _ => None,
    }
}

fn main() {
    println!("{}", value_in_cent(&Coin::Penny));
    println!("{}", value_in_cent(&Coin::Nickel));
    println!("{}", value_in_cent(&Coin::Dime));
    println!("{}", value_in_cent(&Coin::Quarter));
    let message_write = Message::Write(String::from("message"));
    let message_move = Message::Move { x: 1, y: 2 };
    let message_change_color = Message::ChangeColor(12, 12, 12);
    let message_quit = Message::Quit;
    message_write.call();
    message_move.call();
    message_change_color.call();
    message_quit.call();
    dbg!(&optional_plus_one(Some(5)));
    dbg!(&optional_plus_one(None));
}
