# Banking System

A lightweight Rust console banking application with role-based access.

- User portal: create an account, log in, and perform account transactions.
- Manager portal: view all accounts, create accounts, and modify existing accounts.

The application runs entirely in the terminal and stores all data in memory.

## Features

- Role-based main menu (User or Manager)
- Account creation with:
  - Primary owner name
  - Account type (Current or Saving)
  - Initial deposit
  - Optional shared mode
- Optional secondary owner for shared accounts
- Login by account ID
- Account operations:
  - Deposit
  - Withdraw (with insufficient funds check)
  - View balance
  - View full account details
- Manager ledger view for all existing accounts
- Manager ownership controls:
  - Convert single-owner accounts to shared
  - Add secondary owners after creation

## Tech Stack

- Language: Rust (Edition 2024)
- Build tool: Cargo

## Project Structure

- `src/main.rs` - app entry point and top-level role menu
- `src/modules/mod.rs` - module registration
- `src/modules/bank.rs` - core banking domain models and in-memory storage
- `src/modules/user.rs` - user portal flow
- `src/modules/manager.rs` - manager portal flow
- `src/modules/account_menu.rs` - account creation and account operation menu
- `src/modules/input.rs` - reusable terminal input helpers

## Requirements

- Rust toolchain installed
- Cargo available in PATH

## Run

From the project root:

```bash
cargo run
```

## Application Flow

### Main Menu

1. User
2. Manager
3. Exit

### User Portal

1. Login
2. Create Account
3. Exit to Main Menu

After login, account operations are:

1. Withdraw
2. Deposit
3. View Balance
4. Exit

### Manager Portal

1. View all accounts
2. Create Account
3. Modify account (by account ID)
4. Exit to Main Menu

The same account operations menu is used when the manager modifies an account.

## How The Project Works Internally

This section explains the complete runtime flow and how each module collaborates.

### 1. Startup and App Loop

- The program starts in `src/main.rs`.
- A single `Bank` instance is created with `Bank::new()`.
- The app enters a loop and shows the main role menu:
  - User
  - Manager
  - Exit
- The same `Bank` instance is passed to both portals, so every action updates shared in-memory state.

### 2. Core Domain and Storage

- `src/modules/bank.rs` defines the banking model:
  - `AccountType` enum (`Current` or `Saving`)
  - `Account` struct (owner, id, balance, account type, optional secondary owners)
  - `Bank` struct (`HashMap<u32, Account>` + incremental `next_id`)
- Account IDs are generated sequentially starting from `1001`.
- All account records live in memory only while the process is running.

### 3. Account Creation Path

- Both User and Manager portals can create accounts.
- The shared account creation logic lives in `src/modules/account_menu.rs` (`create_account`).
- The creation sequence is:
  1. Read primary owner name
  2. Read account type
  3. Read initial deposit
  4. Ask whether account is shared
  5. Create account through `Bank::create_account`
  6. Optionally attach a secondary owner for shared accounts

### 4. Login and Account Access

- Login/modify operations request an account ID and call `Bank::get_account_mut(id)`.
- If found, the user gets a mutable reference to that account.
- If not found, an error message is printed and control returns to the portal menu.

### 5. Transaction Operations

- Transactions are centralized in `account_menu(account: &mut Account)`.
- Available operations:
  - Withdraw
  - Deposit
  - View Balance
  - Exit
- `deposit` increases balance directly.
- `withdraw` checks for insufficient funds before subtracting.
- Because the account is modified through a mutable reference, updates are immediately reflected in the `Bank` ledger.

### 6. Input Handling and Validation

- `src/modules/input.rs` contains reusable terminal input functions.
- `input_num()` accepts positive whole numbers.
- `input_str()` allows alphabetic names and spaces.
- `input_bool()` and `input_account_type()` enforce menu-based choices.
- Invalid input repeats the prompt until a valid value is entered.

### 7. Role Separation

- `src/modules/user.rs` handles user-facing menu flow.
- `src/modules/manager.rs` handles manager-facing menu flow.
- Both roles reuse the same domain model and transaction logic, which keeps behavior consistent.

### 8. End-of-Process Behavior

- Choosing Exit only ends the running process.
- Since there is no file or database persistence layer yet, all account data is cleared on exit.

## Coming Next

Planned improvements in order:

1. Binary serialization with a custom file type

- Translate Rust structs directly into raw machine bytes instead of text.
- Store account data in a custom binary file that is fast to read and write.
- If the file is opened in a text editor like Notepad, it will look like unreadable garbled characters.

2. Persistent data storage

- Save account data to disk/database so records do not get erased when the app exits.

3. Account statement history

- Track per-account transaction history (deposits, withdrawals, and timestamps) and display statements.

## Current Limitations

- Data is not persisted; all accounts are lost when the app exits
- Money input currently uses positive whole numbers only
- No authentication beyond account ID lookup

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
- [ ] Part 9: Binary serialization using a custom file type
- [ ] Part 10: Statement history for each account
