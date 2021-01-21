fn main() {
    let var = loop {
        println!("again!");
        break 1;
    };
    println!("The result is {}", var);
    whileme();
    forloop();
    range_usage();
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

fn range_usage() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}