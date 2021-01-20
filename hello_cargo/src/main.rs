fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);

    let (l, y, z) = tup;

    println!("The value of y is: {}", y);

    let a = [1, 2, 3, 4, 5];
    let index = 2;

    let element = a[index];

    println!("The value of element is: {}", element);

    let e = 5;

    let y = {
        let e = 3;
        e + 1
    };

    println!("The value of y is: {}", y);


    let five = five();

    println!("The value of five is: {}", five);
}

fn five() -> u32 {
    5
}