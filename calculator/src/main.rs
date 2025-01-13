use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter an expression (e.g., 2 + 2): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        if input == "exit" {
            println!("Exiting calculator.");
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.len() != 3 {
            println!("Invalid input. Please enter an expression like '2 + 2'.");
            continue;
        }

        let num1: f64 = match tokens[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", tokens[0]);
                continue;
            }
        };

        let operator = tokens[1];

        let num2: f64 = match tokens[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", tokens[2]);
                continue;
            }
        };

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Division by zero is not allowed.");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("Invalid operator: {}", operator);
                continue;
            }
        };

        println!("Result: {}", result);
    }
}
