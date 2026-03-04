
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(message: Message) {
    match message {
        Message::Quit => { println!("Quit"); },
        Message::Write(s) => { println!("{s}") }
        default => { println!("default"); }
    }
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => { println!("Call Quit"); },
            _ => { println!("Call _"); }
        }
    }
}

pub fn main() {

    println!("Hello, world!");

    let mut m = Message::Write(String::from("Hi"));
    m.call();
    process_message(m);

    m = Message::Quit;
    m.call();
}