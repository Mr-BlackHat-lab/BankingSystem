use crate::modules::account_menu::{account_menu, create_account};
use crate::modules::bank::{Account, Bank};
use crate::modules::input::input_num;
pub fn user(bank: &mut Bank) {
    println!("--- User Portal ---");
    loop {
        println!("1. Login \n2. Create Account\n3. Exit");
        let inpu = input_num();

        match inpu {
            1 => {
                user_login(bank);
                continue;
            }
            2 => {
                create_account(bank);
                continue;
            }
            3 => {
                println!("Going Back to main menu");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }
}
fn user_login(bank: &mut Bank) {
    println!("Enter your Account ID:");
    let user_id = input_num();

    match bank.get_account_mut(user_id) {
        Some(account) => {
            println!("Welcome back! Accessing account {}.", account.id);
            // Pass the specific account reference, NOT the whole bank!
            account_menu(account);
        }
        None => {
            println!("Error: Account ID {} not found in the system.", user_id);
        }
    }
}
