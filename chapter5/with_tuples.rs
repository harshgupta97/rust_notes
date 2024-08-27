fn main() {
    let rectangle = (3, 2);

    println!("Area is {}", area(rectangle));
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
