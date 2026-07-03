use crate::modules::bank::{Account, Bank};
use crate::modules::input::{input_account_type, input_bool, input_num, input_str};

pub fn create_account(bank: &mut Bank) {
    println!("Let's open a new account");
    println!("Account Owner Name:");
    let name = input_str();
    println!("Enter Account type:");
    let account_type = input_account_type();
    println!("Enter your initial deposit");
    let money = input_num() as f64;
    println!("Is Account is shared:");
    let is_shared = input_bool();
    let gernated_id = bank.create_account(name, money, account_type, is_shared);
    if is_shared {
        println!("Do you want to add secondary user");
        let yn = input_bool();
        if yn {
            match bank.get_account_mut(gernated_id) {
                Some(account) => {
                    let new_owner = input_str();
                    account.add_secondary_owner(new_owner);
                }
                None => {
                    println!("Error: Account ID {} not found in the system.", gernated_id);
                }
            }
        }
    }
    println!("Success! Your new Account ID is: {}", gernated_id);
    println!("Please write this down to log in later.");
}
pub fn account_menu(account: &mut Account) {
    loop {
        println!("\nWhat task do you want to perform?");
        println!("1. Withdraw \n2. Deposit\n3. View Balance\n4. Add second user\n5. Account Details\n0. Exit");
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
                println!("Enter the Name:");
                let name = input_str();
                account.add_secondary_owner(name);
            }
            5 => {
                account.account_detail();
            }
            0 => {
                println!("Logging out...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 0, 1, 2, 3, 4 or 5.");
            }
        }
    }
}
