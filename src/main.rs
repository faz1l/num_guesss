use std::io;
use std::cmp::Ordering; // Package to order the objects
use rand::Rng; // Package to create a random number
use colored::*; // Package to colorize the output

fn main() {
    println!("{}\n", "***___NUM GUESSS GAME___***".yellow());
    println!("{}\n\n", "Guess the number: ".blue());

    let secret_number = rand::thread_rng().gen_range(0, 5); // Generate the random number

    loop {

        println!("{}\n", "Please input your guess.".blue()); // Ask the user to input the number

        let mut guess = String::new(); // Create the variable to store the user guess
        io::stdin().read_line(&mut guess).expect("Failed to read the guess."); // Get the user guess
        let guess: u32 = match guess.trim().parse() { // Convert the user guess to a number
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}\n", "Too small!".red()), // Alert if the user guess is to small
            Ordering::Greater => println!("{}\n", "Too big!".red()), // Alert if the user guess is too big
            Ordering::Equal => { // Alert if the user guess is equal to secret num and break the loop
                println!("{}\n", "You win!".green());
                break;
            }
        }
    }
}
