use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Failed to parse number");

    println!("The square of {} is {}", number, number * number);
}
