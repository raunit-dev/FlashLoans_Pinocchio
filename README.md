# FlashLoans_Pinocchio

> A Rust smart contract for flash loan operations on the Pinocchio blockchain.

## Overview

**FlashLoans_Pinocchio** is a Rust-based smart contract designed to facilitate flash loan operations on the Pinocchio blockchain. It enables borrowers to instantly obtain and repay loans within a single transaction, supporting complex DeFi use cases such as arbitrage, liquidation, or collateral swaps. The contract manages the lifecycle of a flash loan, including loan creation, token transfers, and secure repayment logic.

## Features

- **Flash Loan Issuance:** Instantly borrow tokens with no upfront collateral, provided the loan is repaid in the same transaction.
- **Repayment Validation:** Ensures loans are repaid with the correct amount and closes loan accounts securely.
- **Account Management:** Uses structured account grouping for borrowers, protocols, loan data, and token accounts.
- **Token Transfers:** Integrates with Pinocchio token instructions for secure token movement between protocol and borrower.
- **Fee Application:** Supports configurable loan fees applied during the issuance process.

## How It Works

### Loan Creation

- The borrower requests a flash loan by invoking the loan instruction.
- The contract verifies accounts, calculates fees, and transfers tokens from the protocol to the borrower.
- Loan details (protocol token account and balance) are stored in a temporary loan account.

### Repayment

- The borrower repays the loan within the same transaction.
- The contract validates the repayment amount and the integrity of each protocol token account.
- Upon successful repayment, the loan account is closed, and any rent-exempt lamports are returned to the borrower.

## Core Structures

- `LoanAccounts`: Groups accounts for borrower, protocol, loan, instruction sysvar, and token accounts.
- `LoanData`: Stores protocol token account and balance for each loan entry.
- `RepayAccounts`: Groups accounts for borrower, loan, and token accounts.
- `LoanInstructionData`: Encapsulates loan parameters, including amounts and fees.

## Usage

### Prerequisites

- Pinocchio blockchain node and toolchain
- Rust (nightly recommended)
- Pinocchio SDK and `pinocchio_token` crate

### Building

```sh
cargo build --release
```

### Deploying

Deploy the program using Pinocchio's deployment tools. Example:

```sh
pinocchio program deploy --program target/release/flash_loans_pinocchio.so
```

### Example Flow

1. **Instantiate Loan:**
   - Call the loan instruction with required accounts and token pairs.
2. **Repay Loan:**
   - Call the repay instruction within the same transaction, ensuring all tokens are returned plus any required fees.

## Code Structure

```
src/
  instructions/
    loan.rs       # Loan issuance logic
    repay.rs      # Loan repayment logic
    helpers.rs    # Utility structs and functions
  lib.rs          # Entrypoint and program ID
```

## Security

- All logic enforces strict account checks.
- Repayment must occur in the same transaction to avoid risk of default.
- Lamports and token balances are validated before closing accounts.

## License

*No license specified. Please add a license if you intend to open source or reuse this code.*

## Contributing

Pull requests and issues are welcome! Please open an issue for feature requests or bug reports.

## Author

- [raunit-dev](https://github.com/raunit-dev)

---

> **Note:** This contract is intended for educational and prototyping purposes. Use with caution in production environments.
