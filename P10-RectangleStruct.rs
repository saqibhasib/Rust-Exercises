struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_contain(&self, other: Rectangle) -> bool {
        return (self.width > other.width) && (self.height > other.height);
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is: {}", rect.area());
    let other_rect = Rectangle {width: 29, height: 55};
    println!("{}", rect.can_contain(other_rect));
}
