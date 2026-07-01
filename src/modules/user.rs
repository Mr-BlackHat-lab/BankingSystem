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
                user_create_account(bank);
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
            user_menu(account);
        }
        None => {
            println!("Error: Account ID {} not found in the system.", user_id);
        }
    }
}

fn user_menu(account: &mut Account) {
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

fn user_create_account(bank: &mut Bank) {
    println!("Let's open a new account");
    println!("Enter your initial deposit");
    let money = input_num() as f64;
    let gernated_id = bank.create_account(money);
    println!("Success! Your new Account ID is: {}", gernated_id);
    println!("Please write this down to log in later.");
}
