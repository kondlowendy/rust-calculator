use std::io;

fn main() {
    println!("Simple Rust Calculator");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut operation = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    println!("Enter second number:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");

    println!("Choose operation (+, -, *, /):");
    io::stdin().read_line(&mut operation).expect("Failed to read input");

    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");
    let op = operation.trim();

    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Cannot divide by zero");
                return;
            }
        }
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("Result: {}", result);
}