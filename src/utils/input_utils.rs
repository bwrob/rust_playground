use std::io::stdin;
use std::str::FromStr;

pub fn input_as_integer_result() -> Result<isize, <isize as FromStr>::Err> {
    println!("Please enter an integer number:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    // Trim the input and parse it to an i32 number.
    // This is infered by the compiler from function signature.
    input.trim().parse()
}

pub fn input_as_integer() -> isize {
    // Calling the result function from the previous function
    input_as_integer_result().expect("Not a number!")
}
