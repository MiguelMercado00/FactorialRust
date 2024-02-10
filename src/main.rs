// This code is about something simple, is just practicing this language
// I'm trying to learn Rust, so I'm doing some simple things to get used to it
// I'm trying to make a simple program that takes a number and returns the factorial of that number

use std::io;

fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

fn main() {
    println!("Enter a number: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: u32 = number.trim().parse().expect("Please type a number");
    let factorial = factorial(number);
    println!("The factorial of {} is {}", number, factorial);
}

// This is a simple program that takes a number and returns the factorial of that number

