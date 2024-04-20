fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtract_numbers(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn divide_numbers(num1: i32, num2: i32) -> i32 {
    num1 / num2
}

fn multiply_numbers(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn modulo_numbers(num1: i32, num2: i32) -> i32 {
    num1 % num2
}

fn main() {
    println!("Welcom Stable V1");

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
}
