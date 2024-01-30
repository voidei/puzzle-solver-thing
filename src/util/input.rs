use std::io;

pub fn read() -> String {
    let mut input = String::new();
    println!("Enter the first line of the puzzle please:");

    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    input.trim().to_string()
}
