use rand::Rng; // Rng trait defines methods that random number generators implement
use std::cmp::Ordering;
use std::io; // ability to accept user input // enum (Less, greater, equal)

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //rand::thread_rng -> fn gives the particular random no. generator

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); // :: -> associated fn (implemented on a type, here String) -> here it's creating a new, empty string

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        /*  .read_line(&mut guess) calls read_line method on std input to get input from user
        -> we're passing &mut guess as the argument to read_line to tell it what string to store the user input in.
        -> the full job of read_line is to take whatever the user types into standard input and append that into a string (w/o overwriting it's contents)
        -> the string arg needs to be mutable so the method can change the string's content
        -> & indicates that this arg is a reference (references are immutable by default, hence &mut guess and not &guess) */

        /* read_line also returns a Result value. result is an enum.
        -> Results variants are Ok (op is success) and Err (op failed and the reason)
        -> Result also has methods defined -> eg. expect method (if result is err-> program will crash and display the text)
         */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catch-all value
        };
        // shadowing prev guess here.
        // trim eliminates \r\n or \n
        // parse method on strings converts a string to another type (here, to a number u32) -> parse also returns a Result type

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            //cmp-> compare
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
