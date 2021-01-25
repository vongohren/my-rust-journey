#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 50,
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "Can hold the rect2: {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "Can hold the rect3: {}",
        rect1.can_hold(&rect3)
    );

    println!(
        "A square rect: {:#?}",
        Rectangle::square(32)
    );
}    



