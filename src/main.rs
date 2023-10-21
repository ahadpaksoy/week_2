/*
    using this std lib for taking user input from keyboard
*/
use std::io;

fn main() {
    println!("Enter the first number : ");
    let mut first = String::new();
    io::stdin().read_line(&mut first).expect("Failed to read"); // reading intput
    let first : f64 = first.trim().parse().expect("Please enter a number!"); // parsing input string to float

    println!("Enter the opearation type (+,-,*,/) : "); 
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read");
    let operation = operation.trim(); // don't need to parse 

    println!("Enter the second number : ");
    let mut second = String::new();
    io::stdin().read_line(&mut second).expect("Failed to read");
    let second : f64 = second.trim().parse().expect("Please enter a number!"); // parsing input string to float

/*
    Created an operation enum instance with the parsed input
*/
    let operation_type = match operation { 
        "+" => Operations::Add { first: first, second: second },
        "-" => Operations::Substract { first: first, second: second },
        "*" => Operations::Multiply { first: first, second: second },
        "/" => Operations::Divide { first: first, second: second },
        _ => {
            println!("Invalid Operation!");
            return;
        }
    };
/*
    calling the calculate fn
*/
    let calculation = calculate(&operation_type);

/*
    printing result to the console
*/
    println!("Result is : {}", calculation);
}

/*
    defining the operation enum
*/
enum Operations{
    Add {first : f64, second : f64 },
    Substract {first : f64, second : f64 },
    Multiply {first : f64, second : f64 },
    Divide {first : f64, second : f64 },
}

/*
    calculate function signature
*/
fn calculate(calc: &Operations) -> f64{
    match calc{
        Operations::Add { first, second } => first + second,
        Operations::Substract { first, second } => first - second,
        Operations::Multiply { first, second } => first * second,
        Operations::Divide { first, second } => {
            if *second != 0.0 {
                first / second
            }else {
                println!("Division by zero is not allowed");
                std::f64::NAN
            }
        },  
    }
}