#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Area: {}", area(&rect1));
    println!("Rect: {:?}", rect1); // Still valid because we borrowed
}