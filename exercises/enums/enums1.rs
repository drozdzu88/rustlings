// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit(String),
    Echo(String),
    Move(String),
    ChangeColor(String),
}

fn main() {
    println!("{:?}", Message::Quit(String::from("hello")));
    println!("{:?}", Message::Echo(String::from("hello")));
    println!("{:?}", Message::Move(String::from("hello")));
    println!("{:?}", Message::ChangeColor(String::from("hello")));
}
