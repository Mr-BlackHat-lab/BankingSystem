use crate::modules::bank::{Account, Bank};
use crate::modules::input::input_num;
pub fn manager(bank: &mut Bank) {
    println!("--- Manager Portal ---");
    loop {
        println!("what task you want to perform");
        println!("1. View all accounts \n2. Modify account\n3. Exit");

        let task = input_num();
        match task {
            1 => {
                bank.print_all_accounts();
                continue;
            }
            2 => {
                manager_modify_balance(bank);
                continue;
            }
            3 => {
                println!("Going to main menu");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }
}

fn manager_modify_balance(bank: &mut Bank) {
    println!("Enter the Account ID you want to modify:");
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

fn account_menu(account: &mut Account) {
    loop {
        println!("\nWhat task do you want to perform?");
        println!("1. Withdraw \n2. Deposit\n3. View Balance\n4. Exit");
        let opt = input_num();

        match opt {
            1 => {
                println!("Enter amount to withdraw:");
                let amount = input_num() as f64;
                account.withdraw(amount);
            }
            2 => {
                println!("Enter amount to deposit:");
                let amount = input_num() as f64;
                account.deposit(amount);
            }
            3 => {
                println!("Your current balance is: ${}", account.get_balance());
            }
            4 => {
                println!("Logging out...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, 3, or 4.");
            }
        }
    }
}
