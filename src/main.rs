use std::io;
use rand::Rng;

fn main() {
    println!("Hello, Ferris here!");
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let mut secret_number = rand::thread_rng().gen_range(1..=3);
    let guess :i32 = guess
        .trim().parse()
        .expect("Please type a number!");
    secret_number += guess;


    println!("The secret number is: {secret_number}");
    println!("You guessed: {guess}");
    println!("You lost!");
} 
