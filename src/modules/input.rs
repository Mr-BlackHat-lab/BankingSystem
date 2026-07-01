use std::io;

pub fn input_num() -> u32 {
    loop {
        let mut input = String::new();

        println!("Enter a number:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a positive whole number."),
        }
    }
}
