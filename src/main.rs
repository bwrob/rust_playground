pub mod chapters;
pub mod utils;

use utils::input_utils::input_as_integer;

const DIVIDER: &str = "-------------------------";

fn main() {
    println!("Hello, Ferris here!");
    println!("{DIVIDER}");
    println!("Please choose a chapter to start with:");
    println!("2. Guessing Game");
    println!("3. Concepts Playground");
    println!("{DIVIDER}");

    match input_as_integer() {
        2 => chapters::chapter_2::guessing_game(),
        3 => chapters::chapter_3::concepts_playground(),
        _ => println!("Invalid input"),
    }
}
