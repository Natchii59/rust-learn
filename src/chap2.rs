/*
    The Guessing Game
    1. Generate a random number between 1 and 100
    2. Ask the user to guess the number
    3. If the user's input is not a number, it should print "Please type a number" and continue the loop
    4. If the user's guess is less than the number, it should print "Too small!"
    5. If the user's guess is greater than the number, it should print "Too big!"
    6. If the user's guess is equal to the number, it should print "You win!" and exit
*/

// Import the rand modules
use rand::Rng;
// Import the std modules (cmp and io)
use std::{cmp::Ordering, io};

pub fn chap2() {
    println!("Guesst the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Create an infinite loop
    loop {
        println!("Please input your guess.");

        // Create a mutable variable to store the user input
        let mut guess = String::new();

        // Read the user input and store it in the guess variable
        io::stdin()
            .read_line(&mut guess) // Read the user input and store it in the guess variable
            .expect("Failed to read line"); // Handle the error

        // Convert the user input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If the user input is a number, store it in the guess variable
            Err(_) => {
                // If the user input is not a number, print an error message and continue the loop
                println!("Please type a number");
                continue;
            }
        };

        println!("You guess {}", guess);

        // Compare the user input to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // If the user input is less than the secret number, print "Too small!"
            Ordering::Greater => println!("Too big!"), // If the user input is greater than the secret number, print "Too big!"
            Ordering::Equal => {
                // If the user input is equal to the secret number, print "You win!" and break the loop
                println!("You win!");
                break;
            }
        }
    }
}
