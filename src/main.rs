use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    //print macro
    println!("Guessing The number!");
   loop {
    println!("Please Input your guess");

    //generate random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //print macro
    println!("The secret number is: {} ", secret_number);

    //declare a mutable string variable
    let mut guess: String =  String::new();

    //mutable reference without taking ownership
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //print macro
    println!("You guessed: {guess}");

     //convert our input into integer and check for error incase of invalid input
     let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
     };

     //compare two numbers using match
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}", "Too big!".blue()),
        Ordering::Equal => {
            println!("{}", "You win!".green());
            break;
        },
    }
   }
}
