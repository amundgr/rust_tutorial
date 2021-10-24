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
        (self.width > other.width) & (self.height > other.height)
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 1;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle{
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle{
        width: 10,
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    dbg!(&rect1);

    println!(
        "Can rect1 hold rec2? {}", rect1.can_hold(&rect2)
    );
    println!(
        "Can rect1 hold rec3? {}", rect1.can_hold(&rect3)
    );

    let square1 = Rectangle::square(32);
    dbg!(&square1);

}
