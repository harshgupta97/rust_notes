use std::io;

fn main() {
    println!("Enter temperature in Fahrenheit (degree F)");

    let mut temperature: String = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read input, Please try again");

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    println!(
        "Convert Fahrenheit To Celsius {}",
        convert_fahrenheit_to_celsius(temperature)
    );
}

fn convert_fahrenheit_to_celsius(temperature: f32) -> f32 {
    const ROOM_TEMPERATURE: f32 = 32.0;
    let celsius = (temperature - ROOM_TEMPERATURE) * (5.0/9.0);
    celsius
}
