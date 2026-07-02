use crate::modules::bank::AccountType;
use std::io;

pub fn input_bool() -> bool {
    loop {
        println!("1. Yes");
        println!("2. No");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return true,
            "2" => return false,
            _ => println!("Invalid choice."),
        }
    }
}
pub fn input_account_type() -> AccountType {
    loop {
        println!("1. Current");
        println!("2. Saving");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return AccountType::Current,
            "2" => return AccountType::Saving,
            _ => println!("Invalid choice."),
        }
    }
}
pub fn input_str() -> String {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let name = input.trim();

        if !name.is_empty() && name.chars().all(|c| c.is_alphabetic() || c == ' ') {
            return name.to_string();
        }

        println!("Invalid input. Please enter letters only.");
    }
}
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
