// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// Enum
// type that can be any one of several variants.

// Enums in Rust are similar to those of other compiled languages like C, but have important differences that make them considerably more powerful. What Rust calls enums are more commonly known as Algebraic Data Types if you're coming from a functional programming background. The important detail is that each enum variant can have data to go along with it.

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
