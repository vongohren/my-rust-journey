fn main() {
    let var = loop {
        println!("again!");
        break 1;
    };
    println!("The result is {}", var);
    whileme();
    forloop();
}


fn whileme() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

}

fn forloop () {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}