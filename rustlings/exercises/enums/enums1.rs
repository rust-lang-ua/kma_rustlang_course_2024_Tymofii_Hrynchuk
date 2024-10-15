// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// I AM DONE

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(String),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Hello world!")));
    println!("{:?}", Message::Move { x: 1, y: 1 });
    println!("{:?}", Message::ChangeColor(String::from("#00ff00")));
}
