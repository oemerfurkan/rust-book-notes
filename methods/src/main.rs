/*
METHODS
*/

/// Structs let you create custom types that are meaningful 
/// for your domain. By using structs, you can keep 
/// associated pieces of data connected to each other and 
/// name each piece to make your code clear. In impl blocks, 
/// you can define functions that are associated with your type, 
/// and methods are a kind of associated function that let you 
/// specify the behavior that instances of your structs have.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        self.width >= r.width && self.height >= r.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::square(32);

    println!("{square1:?}");

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}