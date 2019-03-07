fn main() {
    let fahrenheit = 40;
    println!("result: {:?}", convert_fahrenheit_to_celsius(fahrenheit)); 
}

fn convert_fahrenheit_to_celsius(fahrenheit: u32) -> u32 {
    fahrenheit - 32 * 5/9
}
