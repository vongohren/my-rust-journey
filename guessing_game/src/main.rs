use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    println!("Input your guess");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("Failed line to read");
    
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }



}
