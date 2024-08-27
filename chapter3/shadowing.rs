fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Number in scope {x}");
    }

    println!("Number out of scope {x}")
}
