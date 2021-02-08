#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


pub fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let coin2 = Coin::Dime;
    println!("{}",value_in_cents(coin));
    println!("{}",value_in_cents(coin2));

    // coin1 nad coin2 are dead on this line, as it was not references

    let some_u8_value = 3u8;
    println!("{}",some_u8_value);
    match_random_u8value(some_u8_value);

    if_let_test();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn match_random_u8value(value: u8) {
    match value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn if_let_test() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    
}
