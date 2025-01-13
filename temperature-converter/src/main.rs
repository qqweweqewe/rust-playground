use std::io;

fn main() {
    println!("enter temperature and add suffix (e. g. '15f' for farenheit, or '32c' for celsius)");

    let io_handle = io::stdin();
    let mut input: String = String::new();
    
    loop {
        io_handle.read_line(&mut input).expect("unexpected err");
        
        input = String::from(input.trim());

        let number: i32 = String::from(&input[..input.len() - 1]).parse().expect("probably not a number!!"); 
        let suffix: &str = &input[input.len() - 1..];

        let res: f32 = match suffix {
            "f" => (number as f32 - 32f32)*5f32/9f32,
            "c" => number as f32 * 9f32/5f32 + 32f32,
            _ => 0.0
        };
        
        println!("{}", res);
        input.clear();
    }
}
