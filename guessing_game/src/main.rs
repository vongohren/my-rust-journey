use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    

    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Input your guess");
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed line to read");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, I need a number");
                continue;
            },
        };
    
        println!("You guessed {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }



}
