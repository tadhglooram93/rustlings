#[derive(Debug)]
enum Message {
    Quit,
    Move,
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Write(String::from("Hello")));
    println!("{:?}", Message::ChangeColor(255, 0, 0));
}
