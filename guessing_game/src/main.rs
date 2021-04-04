use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;
mod guess;



fn main() {
    println!("Guess the number");
    

    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Input your guess");
        let mut guess_string = String::new();

        stdin().read_line(&mut guess_string).expect("Failed line to read");
        
        let guessed = guess::Guess::new(guess_string);
        
        let f = guessed.unwrap_or_else(|error| {
            panic!("{:#?}",error);
        });
    
        println!("You guessed {}", f.value());
    
        match f.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }



}
