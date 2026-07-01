mod modules;
use modules::bank::Bank;
use modules::input;

fn main() {
    real_logic();
}

fn real_logic() {
    println!("Welcome to NewBank:\n\n");

    // 1. Create your central bank state here
    let mut my_bank = Bank::new();

    loop {
        println!("who are you? ");
        println!("1. User \n2. Manager\n3. Exit");
        let inpu = input::input_num();

        match inpu {
            1 => {
                println!("user");
                // 2. Pass a mutable reference so the user can modify things (like depositing money)
                user(&mut my_bank);
                break;
            }
            2 => {
                println!("manager");
                // Manager also gets access to the bank state
                manager(&mut my_bank);
                break;
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

// 3. Update signatures to accept the Bank state
fn manager(bank: &mut Bank) {
    // Manager logic here
}

fn user(bank: &mut Bank) {
    // User logic here
}
