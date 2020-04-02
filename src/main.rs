use std::io;
use std::cmp::Ordering; // Package to order the objects
use rand::Rng; // Package to create a random number
use colored::*; // Package to colorize the output

fn main() {
    println!("{}\n\n", "***___NUM GUESSS GAME___***".yellow()); // Print the title

    // Get the secret number range params
    println!("{}", "Enter the first random number param: ".blue()); // Ask the user to input the first param
    let mut first_param = String::new(); // Create the variable to store the first param
    io::stdin().read_line(&mut first_param).expect("Failed to read the param."); // Get the first param
    let first_param: u32 = first_param.trim().parse().expect("Please enter a number!"); // Convert the first_param to a number
    println!("{}", "Enter the second random number param: ".blue()); // Ask the user to input the second param
    let mut second_param = String::new(); // Create the variable to store the second param
    io::stdin().read_line(&mut second_param).expect("Failed to read the param."); // Get the second param
    let second_param: u32 = second_param.trim().parse().expect("Please enter a number!"); // Convert the second param to a number

    let secret_number = rand::thread_rng().gen_range(first_param, second_param); // Generate the random number

    // The main gameplay
    loop {

        println!("{}\n", "Please input your guess.".blue()); // Ask the user to input the number

        let mut guess = String::new(); // Create the variable to store the user guess
        io::stdin().read_line(&mut guess).expect("Failed to read the guess."); // Get the user guess
        let guess: u32 = match guess.trim().parse() { // Convert the user guess to a number
            Ok(num) => num,
            Err(_) => continue,
        };

        // Match user guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}\n", "Too small!".red()), // Alert if the user guess is to small
            Ordering::Greater => println!("{}\n", "Too big!".red()), // Alert if the user guess is too big
            Ordering::Equal => { // Alert if the user guess is equal to a secret num and break the loop
                println!("{}\n", "You win!".green());
                break;
            }
        }
    }
}
