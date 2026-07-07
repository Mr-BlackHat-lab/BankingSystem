This README was generated with AI assistance.
# Banking System

Banking System is a lightweight Rust terminal application for managing simple bank accounts with separate user and manager flows.

The app persists its data to a local binary file, `database.bank`, using `bincode` and `serde`. On startup it attempts to load existing accounts; on exit it saves the current bank state back to disk.

## Features

- Role-based main menu with `User`, `Manager`, and `Exit` options
- Account creation with:
  - Primary owner name
  - Account type (`Current` or `Saving`)
  - Initial deposit
  - Optional shared account mode
- Optional secondary owner when creating a shared account
- Login by account ID
- Account operations:
  - Withdraw with insufficient-funds protection
  - Deposit
  - View balance
  - View account details
  - Add a secondary owner to a shared account
- Manager tools:
  - View all accounts
  - Create accounts
  - Convert a single-owner account to shared
  - Modify an account by account ID

## Tech Stack

- Language: Rust 2024 edition
- Build tool: Cargo
- Serialization: `serde` + `bincode` 2

## Project Structure

- `src/main.rs` - application entry point and top-level role menu
- `src/modules/mod.rs` - module registration
- `src/modules/bank.rs` - account model, in-memory storage, load/save logic
- `src/modules/account_menu.rs` - account creation and per-account actions
- `src/modules/input.rs` - reusable terminal input helpers
- `src/modules/user.rs` - user portal flow
- `src/modules/manager.rs` - manager portal flow

## Requirements

- Rust toolchain installed
- Cargo available in PATH

## Run

From the project root:

```bash
cargo run
```

## Application Flow

### Startup

1. The program starts in `src/main.rs`.
2. It prints a welcome message and loads `database.bank` if it exists.
3. If loading fails or the file does not exist, the app starts with a fresh `Bank` instance.
4. The main menu is shown in a loop until the user exits.

### Main Menu

1. User
2. Manager
0. Exit

Selecting `0` saves the bank to `database.bank` and ends the process.

### User Portal

1. Login
2. Create Account
0. Exit

Login asks for an account ID. If the account exists, the user enters the account menu for that account.

### Manager Portal

1. View all accounts
2. Create Account
3. Convert account to shared
4. Modify account
0. Exit

The modify flow opens the same account menu used for logged-in users.

### Account Menu

1. Withdraw
2. Deposit
3. View Balance
4. Add second user
5. Account Details
6. Print Statements
0. Exit

## How It Works Internally

### Data Model

`src/modules/bank.rs` defines:

- `TransactionType` with `Deposit` and `Withdrawal`
- `Transaction` records for the transaction kind, amount, and remaining balance
- `AccountType` with `Current` and `Saving`
- `Account` with primary owner, account ID, `f64` balance, account type, optional secondary owners, and transaction history
- `Bank` with a `HashMap<u32, Account>` plus a `next_id` counter that starts at `1001`

Account IDs start at `1001` and increment sequentially.

### Persistence

- `Bank::load_from_file()` tries to deserialize `database.bank`.
- `Bank::save_to_file()` serializes the full bank state back to the same file.
- The saved file is binary data and is not meant to be edited manually.

### Account Creation

The shared creation flow in `src/modules/account_menu.rs` collects:

1. Primary owner name
2. Account type
3. Initial deposit
4. Shared account choice
5. Optional secondary owner if the account is shared

Both the user portal and manager portal reuse this same creation logic.

### Transactions and Account Access

- `Bank::get_account_mut(id)` returns a mutable reference to an account by ID.
- Deposits increase the balance immediately.
- Withdrawals reject requests larger than the current balance.
- Account detail and balance views are printed directly from the account object.

### Input Validation

`src/modules/input.rs` keeps the console prompts simple and consistent:

- `input_num()` accepts positive whole numbers only
- `input_str()` accepts alphabetic characters and spaces
- `input_bool()` is a `Yes` / `No` menu
- `input_account_type()` is a `Current` / `Saving` menu

Invalid input repeats the prompt until a valid value is entered.

## Current Limitations

- Amount input uses positive whole numbers only, so there is no decimal support
- Authentication is limited to account ID lookup
- No password or PIN is required for account access
- Reporting is basic and there is no filtering or search for statements

## Project Status

The project is functional and ready to run from the terminal. Core user and manager flows are implemented, account data is persisted to `database.bank`, and accounts support deposits, withdrawals, balance checks, shared ownership, account details, and printed statements.

The remaining work is focused on stronger authentication, richer reporting, and eventually moving to a GUI.

## Future Work

1. Stronger authentication and role checks
2. More detailed reporting and auditing
3. Shifting to GUI for better user experience

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Done So Far Checklist

- [x] Part 1: Role-based app flow (User, Manager, Exit)
- [x] Part 2: Account creation with owner, account type, and initial deposit
- [x] Part 3: Optional shared account mode with secondary owner support
- [x] Part 4: Login or account access by account ID
- [x] Part 5: Core account operations (withdraw, deposit, view balance)
- [x] Part 6: Manager features (view all accounts, create, modify)
- [x] Part 7: Internal architecture and runtime flow documented in README
- [x] Part 8: Manager conversion from single-owner to shared after creation
- [x] Part 9: Binary serialization using a custom file type
- [x] Part 10: Statement history for each account
- [ ] Part 11: Stronger authentication and role checks
- [ ] Part 12: More detailed reporting and auditing
- [ ] Part 13: Shifting to GUI for better user experience


