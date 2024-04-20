fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtract_numbers(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn divide_numbers(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        panic!("Cannot divide by zero");
    }
    num1 / num2
}

fn multiply_numbers(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn modulo_numbers(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        panic!("Cannot modulo by zero");
    }
    num1 % num2
}

fn sin(angle: f64) -> f64 {
    angle.sin()
}

fn tan(angle: f64) -> f64 {
    angle.tan()
}
fn cos(angle: f64) -> f64 {
    angle.cos()
}
fn to_binary(decimal: u32) -> String {
    if decimal == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut num = decimal;

    while num > 0 {
        let rem = num % 2;
        result.push_str(&rem.to_string());
        num /= 2;
    }

    result.chars().rev().collect()
}
fn main() {
    println!("Welcome Stable V1.4");

    // Example usage of the functions
    let sum = add_numbers(5, 3);
    println!("5 + 3 = {}", sum);

    let difference = subtract_numbers(10, 4);
    println!("10 - 4 = {}", difference);

    let quotient = divide_numbers(20, 5);
    println!("20 / 5 = {}", quotient);

    let product = multiply_numbers(6, 7);
    println!("6 * 7 = {}", product);

    let remainder = modulo_numbers(17, 5);
    println!("17 % 5 = {}", remainder);

    // Example usage of sin and tan functions
    let angle = 45.0;
    let sin_value = sin(angle.to_radians());
    println!("sin({}) = {}", angle, sin_value);

    let tan_value = tan(angle.to_radians());
    println!("tan({}) = {}", angle, tan_value);

    let cos_value = cos(angle.to_radians());
    println!("cos({}) = {}", angle, cos_value);
}
