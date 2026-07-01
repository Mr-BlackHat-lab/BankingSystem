use crate::modules::bank::Bank;
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
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }
}

fn manager_modify_balance(bank: &mut Bank) {}
