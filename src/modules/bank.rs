use std::collections::HashMap;

pub struct Account {
    pub id: u32,
    balance: f64,
}
impl Account {
    pub fn new(id: u32, initial_balance: f64) -> Self {
        Account {
            id,
            balance: initial_balance,
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
    pub fn create_account(&mut self, initial_balance: f64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let new_account = Account::new(id, initial_balance);
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
