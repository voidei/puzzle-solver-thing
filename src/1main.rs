mod util;

use util::{input, validate};

fn main() {
    let input = input::read();
    if validate::validate(input.as_str()) {
        println!("Valid input!\n{}", input)
    } else {
        println!("Invalid input!\n{}", input)
    }
    //println!("{}", input)
}

#[allow(dead_code)]
fn temp() {
    println!(
        "
    Welcome to voidei's puzzle solver thing!
    Please input the puzzle's first line using 1 and 0 to represent states or colours:"
    );
}
