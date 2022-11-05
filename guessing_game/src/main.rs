use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("====== GUESS THE NUMBER ======");

    // Generating a random number betweem 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // Looping until the user guesses the random secret number
    loop {
        println!("Please input your guess.");

        // Creating a mutable string variable
        let mut guess = String::new();

        // Taking the string input from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converting the input string to Unsigned 8-bit integer
        let guess: u8 = match guess.trim().parse() {
            // Return the integer if string is parsed without error
            Ok(num) => num,
            // Restart the loop if string parsing raises an error
            Err(_) => {
                println!("Please enter a numerical value");
                continue;
            } 
        };
        

        println!("You guessed: {}", guess);

        // Using ordering crate to print different messages on the console and break when the number is guesses correctly
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
