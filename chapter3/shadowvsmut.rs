fn main() {
    // remove mute to make it shadow variable
    // let mut x = "Hello";

    // let x = x.len();

    // println!("Value {x}");

    let mut owned_string: String = "Hello".to_owned();
    let borrwed_string: &str = "world";

    owned_string.push_str(borrwed_string);

    println!("{owned_string}");
}
