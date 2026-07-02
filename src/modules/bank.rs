use std::collections::HashMap;
use std::string;

enum AccountType {
    Current,
    Saving,
}

pub struct Account {
    pub primary_owner: String,
    pub id: u32,
    balance: f64,
    pub accounttype: AccountType,
    pub secondary_owner: Option<Vec<String>>,
}
impl Account {
    pub fn new(
        owner: String,
        id: u32,
        initial_balance: f64,
        accouttype: AccountType,
        shared: bool,
    ) -> Self {
        let owner_list = if shared { Some(Vec::new()) } else { None };
        Account {
            primary_owner: owner,
            id,
            balance: initial_balance,
            accounttype: accouttype,
            secondary_owner: owner_list,
        }
    }
    pub fn add_secondary_owner(&mut self, new_owner: String) {
        if let Some(ref mut list) = self.secondary_owner {
            list.push(new_owner.clone());
            println!("{} was added to the shared account.", new_owner)
        } else {
            println!("Error: This is a single-user account. You cannot add secondary owners.");
        }
    }
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited {}, new Balance:{}\n", amount, self.balance);
    }
    pub fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Insufficient funds! You only have ${}.", self.balance);
        } else {
            self.balance -= amount;
            println!(
                "Successfully withdrew ${}. New balance: ${}",
                amount, self.balance
            );
        }
    }
    pub fn get_balance(&self) -> f64 {
        self.balance
    }
}
pub struct Bank {
    accounts: HashMap<u32, Account>,
    next_id: u32,
}
impl Bank {
    pub fn new() -> Self {
        Bank {
            accounts: HashMap::new(),
            next_id: 1001,
        }
    }
    pub fn create_account(
        &mut self,
        owner: String,
        initial_balance: f64,
        accouttype: AccountType,
        shared: bool,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let new_account = Account::new(owner, id, initial_balance, accouttype, shared);
        self.accounts.insert(id, new_account);
        id
    }
    pub fn print_all_accounts(&self) {
        if self.accounts.is_empty() {
            println!("No accounts exits in the bank yet");
            return;
        }
        println!("--- Bank Ledger ---");
        for (id, account) in &self.accounts {
            println!("Account ID:{} | Balance ${}", id, account.get_balance());
        }
        println!("-------------------");
    }
    pub fn get_account_mut(&mut self, id: u32) -> Option<&mut Account> {
        self.accounts.get_mut(&id)
    }
}
