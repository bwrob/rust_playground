use std::io::stdin;
use std::str::FromStr;

pub fn input_as_integer_result() -> Result<i32, <i32 as FromStr>::Err> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse()
}
