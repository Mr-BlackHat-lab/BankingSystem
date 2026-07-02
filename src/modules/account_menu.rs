use crate::modules::bank::{Account, Bank};
use crate::modules::input::{input_account_type, input_num, input_str};

pub fn create_account(bank: &mut Bank) {
    println!("Let's open a new account");
    println!("Account Owner Name:");
    let name = input_str();
    println!("Enter Account type:");
    let accounttype = input_account_type();
    println!("Enter your initial deposit");
    let money = input_num() as f64;
    let gernated_id = bank.create_account(name, money, accounttype);
    println!("Success! Your new Account ID is: {}", gernated_id);
    println!("Please write this down to log in later.");
}
pub fn account_menu(account: &mut Account) {
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
