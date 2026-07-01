# Banking System

A simple Rust console banking application for managing accounts through two roles:

- User: create an account, log in, deposit, withdraw, and view balance
- Manager: view all accounts and modify an existing account balance

The project is intentionally lightweight and runs entirely in the terminal.

## Features

- Create a new account with an initial deposit
- Log in using an account ID
- Deposit money into an account
- Withdraw money from an account
- View the current balance
- Manager portal for listing all accounts
- Manager access to an account by ID for balance operations

## Project Structure

- `src/main.rs` - application entry point and role selection menu
- `src/modules/bank.rs` - in-memory bank and account logic
- `src/modules/user.rs` - user portal actions
- `src/modules/manager.rs` - manager portal actions
- `src/modules/input.rs` - numeric input helper
- `src/modules/mod.rs` - module exports

## Requirements

- Rust toolchain installed
- Cargo available in your PATH

## Run The Project

From the project root, run:

```bash
cargo run
```

## How It Works

When the app starts, it shows a main menu with three choices:

1. User
2. Manager
3. Exit

### User Flow

- Create a new account with an initial deposit
- Receive a generated account ID
- Log in later using that account ID
- Deposit, withdraw, or view balance

### Manager Flow

- View all accounts currently stored in memory
- Enter an account ID to access balance operations
- Deposit, withdraw, or view balance for that account

## Current Limitations

- Account data is stored in memory only and resets when the program exits
- Input handling only accepts whole numbers
- Accounts currently track only an ID and balance

## Next Update Plan

Planned upgrade for the next iteration:

- Add owner name fields to accounts
- Support multiple owners for a single account
- Add account types after the manager flow, such as savings or current
- Extend manager tools to display and manage the new account metadata

## Notes

- The project is designed to be simple and educational, focusing on Rust programming concepts.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
