mod modules;
use modules::bank::Bank;
use modules::input;
use modules::manager;
use modules::user;

fn main() {
    real_logic();
}

fn real_logic() {
    println!("Welcome to NewBank:\n\n");

    let mut my_bank = Bank::new();
    println!("who are you? ");
    loop {
        println!("1. User \n2. Manager\n3. Exit");
        let inpu = input::input_num();

        match inpu {
            1 => {
                println!("user");
                user::user(&mut my_bank);
                continue;
            }
            2 => {
                println!("manager");
                manager::manager(&mut my_bank);
                continue;
            }
            3 => {
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }
}
