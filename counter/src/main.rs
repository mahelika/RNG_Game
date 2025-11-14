use std::io::{self, Write};

fn main() {
    let mut value: i64 = 0;
    println!("interactive counter");
    println!("commands:\n 
    i (increase)\n
    d (decrease)\n
    r (reset)\n
    q (quit)\n
    s (show)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); //flush() returns a Result, meaning it can succeed or fail hence unwrap
        /*print!() prints immediately in your code,
        but the terminal does NOT show it immediately unless:
        -> there is a newline \n OR
        -> you call flush() */

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("failed to read input. try again.");
            continue;
        }

        match input.trim() {
            "i" | "inc" |"increase" => {
                value += 1;
                println!("+1 => {}", value);
            }
            "d" | "dec" | "decrease" =>  {
                value -= 1;
                println!("-1 => {}", value);
            }
            "r" | "reset" => {
                value = 0;
                println!("reset => {}", value);
            }
            "s" | "show" => {
                println!("value => {}", value);
            }
            "q" | "quit" => {
                println!("bye! final value: {}", value);
                break;
            }
            other => {
                println!("unknown command: {}. user i,d,r,s,q", other);
            }
        }
    }
}