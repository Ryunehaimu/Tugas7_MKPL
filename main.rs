fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtract_numbers(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn divide_numbers(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        panic!("Cannot divide by zero!");
    }
    num1 / num2
}

fn main() {
    println!("Hello World");

    // Example usage of the functions
    let sum = add_numbers(5, 3);
    println!("5 + 3 = {}", sum);

    let difference = subtract_numbers(10, 4);
    println!("10 - 4 = {}", difference);

    let quotient = divide_numbers(20, 5);
    println!("20 / 5 = {}", quotient);
}
