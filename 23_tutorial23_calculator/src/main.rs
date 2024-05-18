use std::io::{self, Write}; 
use std::str::FromStr;

fn main() {
    loop {
        let mut buffer = String::new(); // a new String called buffer is created to store the user input.
        println!(" ");
        print!("Enter an expression: ");
        io::stdout().flush().unwrap(); // the prompt is displayed immediately
        io::stdin().read_line(&mut buffer).unwrap(); // read a line of input from the user and store it in the buffer variable

        let parts: Vec<&str> = buffer.trim().split_whitespace().collect(); // &str is a reference to a string slice.
        // the type of parts must be Vec<&str> to match the return type of split_whitespace().collect()
        // collect() method is used to convert the iterator into a collection.
        // The Vec<&str> type is used to represent a collection of string slices.
        // it's used to store the individual tokens (words) extracted from the user input.
        // String Slices is a reference to a contiguous sequence of bytes of the string. 
        
        if parts.len() != 3 {
            println!("Error: Invalid expression");
            continue;
        }

        // using f64::from_str()is necessary because the +, -, *, and / operators are not defined for &str.
        let operand1 = f64::from_str(parts[0]).expect("Error: Invalid number");
        let operator = parts[1];
        let operand2 = f64::from_str(parts[2]).expect("Error: Invalid number");

        let result = match operator {
            "+" => operand1 + operand2,
            "-" => operand1 - operand2,
            "*" => operand1 * operand2,
            "/" => operand1 / operand2,
            "%" => operand1 % operand2,
            _ => {
                println!("Error: Unknown operator symbol");
                continue;
            }
        };
        println!("-> Result: {}", result);
    }
}