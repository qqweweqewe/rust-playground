use std::io;

fn main() {
    let a = match parse_f32(&mut get_input("Enter first operand: ")) {
        Ok(num) => num,
        Err(msg) => panic!("{}", msg)
    };

    let op = &get_input("Enter operation: ");

    let b = match parse_f32(&mut get_input("Enter second operand: ")) {
        Ok(num) => num,
        Err(msg) => panic!("{}", msg)
    };

    let result: f32 = match op.as_str() {
        "/" => if b != 0.0 { a / b } else { panic!("zero division permitted!") },
        "%" => a % b,
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        _ => panic!("invalid operator")
    };

    println!("{} {} {} = {}", a, op, b, result);
}

fn get_input(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    input.trim().to_string()
}

fn parse_f32(input: &mut str) -> Result<f32, std::num::ParseFloatError> {
    let normalized = input.replace(',', ".");
    normalized.trim().parse::<f32>()
}
