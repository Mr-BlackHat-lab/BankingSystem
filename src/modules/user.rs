use crate::modules::bank::Bank;
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
                //create fn
                continue;
            }
            3 => {
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
        // println!("Enter your Account ID:");
        // let user_id = 101;
        // match bank.get_account_mut(user_id) {
        //     Some(account) => {
        //         println!("Welcome back! Accessing account {}.", account.id);
        //         account.deposit(50.0);
        //     }
        //     None => {
        //         println!("Error: Account ID {} not found in the system.", user_id);
        //     }
        // }
    }
}
fn user_login(bank: &mut Bank) {
    println!("Enter your Account ID:");
    let user_id = input_num();
    match bank.get_account_mut(user_id) {
        Some(account) => {
            println!("Welcome back! Accessing account {}.", account.id);
            user_menu(bank, user_id);
        }
        None => {
            println!("Error: Account ID {} not found in the system.", user_id);
        }
    }
}
fn user_menu(bank: &mut Bank, user_id: u32) {
    println!("what task you want to prferom?");
    loop {
        println!("1. Withdrawl \n2. Deposite\n3. View Balance\n4. Exit");
        let opt = input_num();
        match opt {
            1 => {
                continue;
            }
            2 => {
                continue;
            }
            3 => {
                continue;
            }
            4 => {
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, 3, or 4.");
            }
        }
    }
}
