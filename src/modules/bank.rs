use bincode::config::standard;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    transaction_type: TransactionType,
    amount: f64,
    remaining_amount: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    Current,
    Saving,
}
#[derive(Serialize, Deserialize)]
pub struct Account {
    pub primary_owner: String,
    pub id: u32,
    balance: f64,
    pub accounttype: AccountType,
    pub secondary_owner: Option<Vec<String>>,
    history: Vec<Transaction>,
}
impl Transaction {
    pub fn new(transaction_type: TransactionType, amount: f64, remaining_amount: f64) -> Self {
        Transaction {
            transaction_type,
            amount,
            remaining_amount,
        }
    }
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
            history: Vec::new(),
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
    pub fn convert_to_shared(&mut self) {
        match self.secondary_owner {
            Some(_) => {
                println!("This account is already shared");
            }
            None => {
                self.secondary_owner = Some(Vec::new());
                println!("Success! Account is coverted to a shared account");
            }
        }
    }
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited {}, new Balance:{}\n", amount, self.balance);
        let record = Transaction::new(TransactionType::Deposit, amount, self.balance);
        self.history.push(record);
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
            let record = Transaction::new(TransactionType::Withdrawal, amount, self.balance);
            self.history.push(record);
        }
    }
    pub fn account_detail(&self) {
        println!(
            "Account ID:{} | Owner {} | Balance ${} | AccountType {:?} | Secondary Owenr{:?} ",
            self.id,
            self.primary_owner,
            self.get_balance(),
            self.accounttype,
            self.secondary_owner
        );
    }
    pub fn get_balance(&self) -> f64 {
        self.balance
    }
    pub fn print_statement(&self) {
        println!("\n--- Bank Statement for Account {} ---", self.id);
        if self.history.is_empty() {
            println!("No transactions yet.");
        } else {
            for (index, transaction) in self.history.iter().enumerate() {
                println!(
                    "{}. {:?} | Amount: ${} | Remaining Amount: ${}",
                    index + 1,
                    transaction.transaction_type,
                    transaction.amount,
                    transaction.remaining_amount
                );
            }
        }
        println!("-------------------------------------\n");
    }
}
#[derive(Serialize, Deserialize)]
pub struct Bank {
    accounts: HashMap<u32, Account>,
    next_id: u32,
}
impl Bank {
    pub fn save_to_file(&self) {
        let encoded =
            bincode::serde::encode_to_vec(self, standard()).expect("Failed to serialize bank");

        fs::write("database.bank", encoded).expect("Failed to write data to file");

        println!("Bank data securely saved to database.bank");
    }
    pub fn load_from_file() -> Self {
        match fs::read("database.bank") {
            Ok(bytes) => match bincode::serde::decode_from_slice::<Bank, _>(&bytes, standard()) {
                Ok((bank, _)) => {
                    println!("Successfully loaded existing data.");
                    bank
                }
                Err(e) => {
                    println!("Failed to decode database: {}", e);
                    Bank::new()
                }
            },
            Err(_) => {
                println!("No existing save found. Starting a fresh bank.");
                Bank::new()
            }
        }
    }
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
            println!(
                "Account ID:{} | Owner: {} | Balance: ${} | AccountType: {:?} | Secondary Owenr{:?} ",
                id,
                account.primary_owner,
                account.get_balance(),
                account.accounttype,
                account.secondary_owner
            );
        }
        println!("-------------------");
    }
    pub fn get_account_mut(&mut self, id: u32) -> Option<&mut Account> {
        self.accounts.get_mut(&id)
    }
}
