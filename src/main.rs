use std::io;
use std::cmp::Ordering;
use rand::Rng;

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

     //convert our input into integer 
     let guess: u32 = guess.trim().parse().expect("Please Type a number");

     //compare two numbers
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
   }
}
