#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(6 * scale),
        height: 8,
    };

    println!("Details {} {:#?}", "rect1", rect1);

    println!("Area is {}", area(&rect1));

    dbg!(&rect1);

    println!("Test {}", rect1.width);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
