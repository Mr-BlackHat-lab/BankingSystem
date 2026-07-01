use crate::modules::bank::Bank;
pub fn manager(bank: &mut Bank) {
    let id = bank.create_account(500.0);
}
