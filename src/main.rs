pub mod chapters;
pub mod utils;

use utils::input_utils::input_as_integer_result;

fn main() {
    println!("Hello, Ferris here!");

    match input_as_integer_result().expect("Please enter a number") {
        2 => chapters::chapter_2::guessing_game(),
        3 => chapters::chapter_3::concepts_playground(),
        _ => println!("Invalid input"),
    }
}
