use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);  // let rand num equal 1-100 inclusive

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();  // vars arent mutable by default in Rust. use 'mut' to make them
    
        // new is an associated function of 'String' type.
        // associated function = implemented on a type; creates a new empty string
        // new just makes new value of some kind
    
        // below could be a single line of code with .accessors (notice ; is at the end of the last function), but breaking it up is easy to read; dont get confused
        io::stdin()
            .read_line(&mut guess)      // pass where to store the line; pass by reference; also immutable and need 'mut'
            .expect("Failed to read line");     // exception handling. compiler warns you and shows how to write without. cool
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,     // _ : catch all
        };  // rust lets you 'shadow' vars. lets you convert types
        // trim(): trims whitespace from beginning and end
        // parse(): converts to another type
        // ... : u32 declares the type
        // since parse returns a Result enum, you can use above code to handle. its a match expression
    
    
        println!("You guessed: {guess}");   // {} is placeholder for values. if x = 2 and y = 2; x + y = {} and 4 would go in the brackets
    
        // match expression is made up of 'arms'. arms consist of 'patterns' to match against and run the code that matches.
        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("Too small!"),
            Ordering::Greater => println! ("Too large!"),
            Ordering::Equal => {
                println! ("You win!");
                break;      //adding break. kinda like a switch block
            }
        }
        
    }
}