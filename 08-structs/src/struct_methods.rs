#[derive(Debug)]

// 1. Defining a Struct:
struct Rectangle {
    width: u32,
    height: u32,
}

// 2. Implementing Methods:
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Adding More Methods
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn struct_meths() {
    println!("\n*** Structs Methods ***");

    // 3. Calling Methods:
    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() //rect1.area() calls the area method on the rect1 instance.
    );

    let rect2 = Rectangle {
        width: 15,
        height: 45,
    };
    let rect3 = Rectangle {
        width: 5,
        height: 35,
    };

    println!("\nCan rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false
}
