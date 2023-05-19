use std::io;

fn main() {
    println!("Give temperature in Fahrenheit: ");
    let mut input = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read temperature in Fahrenheit.");

        let fahrenheit: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let celsius: f64 = fahrenheit - 32.0 * 5.0 / 9.0;
        println!("{fahrenheit}F is {celsius}C");
        break;
    }
}
