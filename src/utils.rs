use std::io::{self};

pub fn read_input() -> String {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).expect("Unable to read from standard input; try again");
    buffer.to_string()
}