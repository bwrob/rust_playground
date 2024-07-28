use crate::utils::input_utils::input_as_integer;

pub const SOME_CONSTANT: f64 = 2.1;
pub const SECONDS_IN_A_DAY: i32 = 60 * 60 * 24;

pub fn concepts_playground() {
    let mut x = input_as_integer();

    println!("The value of x is: {x}");
    x += 1;
    println!("The incremented value of x is: {x}");

    //Constants live for the lifetime of the program
    println!("The value of constant PI is: {SOME_CONSTANT}");

    //Shadowing can change type
    let spaces = " ".repeat(x as usize);
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    //Tuples
    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {y}");
    let x = tup.0;
    println!("The value of x is: {x}");

    //Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The value of first is: {first}");
    let index = (input_as_integer() - 1) as usize;
    let value = a[index];
    println!("The value at index {index} is: {value}");

    //Functions are objects
    let _function_object = input_as_integer;

    //Control flow
    //If statements
    if value < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // Trenary operator
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    //Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");

    //While loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //For loop over arrays
    for element in a {
        println!("The value is: {element}");
    }
}
