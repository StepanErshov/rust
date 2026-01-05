use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    println!("Hello, world!");

    println!("Put first number: ");
    let mut number1: String = String::new();
    io::stdin()
        .read_line(&mut number1)
        .expect("Failed to read line");
    let number1: f32 = number1.trim().parse().expect("Please type a number!");

    println!("Put second number: ");
    let mut number2: String = String::new();
    io::stdin()
        .read_line(&mut number2)
        .expect("Failed to read line");
    let number2: f32 = number2.trim().parse().expect("Please type a number!");

    let result: f32 = multiply(number1, number2);
    let result_string = format!("multiply: {} * {}", number1, number2);
    let (dict, result_dict) = dict_add(&result_string.to_string(), &result.to_string());

    println!("Result: {}", result);
    println!("{:?}, {:?}", result_dict, dict);
}

fn multiply(x: f32, y: f32) -> f32 {
    return x * y;
}

fn dict_add(key: &str, value: &str) -> (HashMap<String, String>, Result<bool, bool>) {
    let mut result = HashMap::new();
    result.insert(key.to_string(), value.to_string());
    if result.len() > 0 {
        return (result, Result::Ok(true));
    } else {
        return (result, Result::Err(false));
    }
}
