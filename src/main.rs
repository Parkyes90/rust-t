#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 40,
    };
    let square = Rectangle::square(32);
    println!("rect1: {:?}", rect1.area());
    println!("rect2은 rect1을 포함하는가??: {}", rect2.can_hold(&rect1));
    println!("square {:?}", square);
}
