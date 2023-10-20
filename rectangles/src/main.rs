#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
    println!("The rectangle is {:?}", rect)
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
