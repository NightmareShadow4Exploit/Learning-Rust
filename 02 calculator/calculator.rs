use std::io;

fn main() {
    let mut input = String::new();

    println!("Please choose the operation you want: \n
    1. Addition \n
    2. Subtraction \n
    3. Multiplication \n
    4. Division
    ");

    io::stdin().read_line(&mut input)
        .expect("Failed to get input!");

    let input = input.trim();

    let mut number1 = String::new();
    let mut number2 = String::new();

    println!("number 1: ");
    io::stdin().read_line(&mut number1)
        .expect("Failed to get input!");
    let number1: i32 = number1.trim().parse()
        .expect("Please enter a valid number!");

    println!("number 2: ");
    io::stdin().read_line(&mut number2)
        .expect("Failed to get input!");
    let number2: i32 = number2.trim().parse()
        .expect("Please enter a valid number!");

    match input {
        "1" => sum(number1, number2),
        "2" => sub(number1, number2),
        "3" => product(number1, number2),
        "4" => division(number1, number2),
        _ => println!("Please choose 1, 2, 3, or 4"),
    }
}

fn sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn sub(x: i32, y: i32) {
    println!("{} - {} = {}", x, y, x - y);
}

fn product(x: i32, y: i32) {
    println!("{} * {} = {}", x, y, x * y);
}

fn division(x: i32, y: i32) {
    if y != 0 {
        println!("{} / {} = {}", x, y, x / y);
    } else {
        println!("Cannot divide by zero");
    }
}
