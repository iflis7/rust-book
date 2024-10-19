#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn example() {
    println!("\n*** Structs ***");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectangle: {rect1:?}\n"); // adding the debug macro, allows to print the Rectangle.

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
