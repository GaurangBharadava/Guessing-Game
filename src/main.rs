#![allow(unused)]
use std::io; // for input and output
use rand::Rng; // to generate random numbers
use std::cmp::Ordering; // for comparing values
fn main() { 
    println!("Welcome to the Gueassing Game!");
    // generate random numbers
    let random_number: u16 = rand::thread_rng().gen_range(1..=1000); // thread_rng() returns the random number generator thread which generate random numbers between given range.

    // take user inputs
    loop {
        println!("Please input the Guess! :");
        let mut guess: String = String::new(); // creates the new empty string
        io::stdin()
            .read_line(&mut guess) // io::stdin().read_line() read the user input
            .expect("Failed to read input"); // handles errors
        
        // match the number 
        // match handles the case where parsing fails
        let guess: u16 = match guess.trim().parse() { // trim() remove the white space and parse() covert the string into the number
            Ok(num) => num,
            Err(_) => {
                println!("Please enter the valid number!");
                continue;
            }
        };

        println!("You guess the {guess}");

        match guess.cmp(&random_number) { // compare the guess with the generated random number
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it! Congratulations!!!");
                break;
            }
        }
    }
}