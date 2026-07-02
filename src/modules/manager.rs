use crate::modules::account_menu::{account_menu, create_account};
use crate::modules::bank::Bank;
use crate::modules::input::input_num;
pub fn manager(bank: &mut Bank) {
    println!("--- Manager Portal ---");
    loop {
        println!("what task you want to perform");
        println!("1. View all accounts\n2. Create Account \n3. Modify account\n4. Exit");

        let task = input_num();
        match task {
            1 => {
                bank.print_all_accounts();
                continue;
            }
            2 => {
                create_account(bank);
                continue;
            }
            3 => {
                manager_modify(bank);
                continue;
            }
            4 => {
                println!("Going to main menu");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }
}

fn manager_modify(bank: &mut Bank) {
    println!("Enter the Account ID you want to modify:");
    let user_id = input_num();
    match bank.get_account_mut(user_id) {
        Some(account) => {
            println!("Welcome back! Accessing account {}.", account.id);
            account_menu(account);
        }
        None => {
            println!("Error: Account ID {} not found in the system.", user_id);
        }
    }
}
