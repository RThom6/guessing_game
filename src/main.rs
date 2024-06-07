use std::io; // Standard io library
use std::cmp::Ordering;
use rand::Rng; 

fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    
    loop{
        println!("Please input your guess.");
        let mut guess = String::new(); // mut means that the variable is mutable, e.g. it can change, bound to empty string instance

        io::stdin()
            .read_line(&mut guess) // readline method of stdin, &mut guess passes guess in to read_line
            .expect("Failed to read line");
        
        /* 
        * Trim trims any whitespace at start and end, 
        * Parse converts to another type defined with {}: [type]
        * Match allows for error handling instead of error crashing
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct guess!");
                break;
            },
        }
    }
}
