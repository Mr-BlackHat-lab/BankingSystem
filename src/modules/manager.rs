use crate::modules::bank::Bank;
pub fn manager(bank: &mut Bank) {
    println!("Manager Portal: Creating a new account...");
    bank.create_account(101, 500.0);
}
