
# Escrow Project

A token escrow system built on Solana using the Anchor framework. This project demonstrates how to create a secure escrow service for depositing and withdrawing SPL tokens.

## Overview

This escrow program allows users to:

1. Deposit SPL tokens into an escrow account
2. Withdraw tokens from the escrow account
3. Track token balances in the escrow

## Architecture

### Program Structure

The program consists of two main instructions:
- `deposit`: Transfers tokens from a user's token account to an escrow token account
- `withdraw`: Transfers tokens from the escrow token account back to the user's token account

### Account Structure

- `Escrow`: Stores information about the escrow, including the authority (owner) and current token amount
- `EscrowTokenAccount`: A PDA-owned token account that holds the tokens while they're in escrow

### Token Flow

### **Deposit Flow:**

1. User initiates a deposit transaction
2. Tokens are transferred from user's token account to the escrow token account
3. The escrow account state is updated to reflect the new balance
4. The user can verify their deposit in the escrow account


### **Withdraw Flow:**


1. User (authority) initiates a withdrawal transaction
2. System verifies user is the authorized withdrawer
3. Tokens are transferred from escrow token account back to user's token account
4. The escrow account state is updated to reflect the reduced balance
5. Only the original depositor can withdraw their tokens

## Technical Details

### PDAs (Program Derived Addresses)

The program uses two PDAs:

1. **Escrow Account PDA**:
   - Seeds: `["escrow", authority.key()]`
   - Stores the escrow state (authority and amount)

2. **Escrow Token Account PDA**:
   - Seeds: `["escrow", authority.key(), mint.key()]`
   - Holds the actual tokens
   - Owned by itself (self-custodial)

### Security Considerations

- **Access Control**: Only the authority can withdraw tokens
- **Balance Verification**: Prevents withdrawing more tokens than available
- **Proper PDA Usage**: Uses PDAs with correct seeds and bumps for token account authority

## Development Setup

### Prerequisites

- Solana CLI
- Anchor Framework
- Node.js and Yarn
- Rust and Cargo

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/escrow-project.git
   cd escrow-project ```

2. Install dependencies:
```bash
yarn install
```

3. Generate a program keypair (if not already done):
```bash 
solana-keygen new -o target/deploy/program-keypair.json 
```

4. Get your program ID:
```bash
solana-keygen pubkey target/deploy/program-keypair.json
``` 

5. Update the program ID in:
   - `Anchor.toml`
   - `programs/basic-escrow/src/lib.rs` (in the `declare_id!` macro)

## Build and Deploy

1. Build the program:
   ```bash
   anchor build

2. Deploy to Solana devnet:
```bash
anchor deploy
```

3. Run tests:
``` bash
anchor test
```


## Testing
The test suite (tests/escrow-project.ts) includes tests for:

- Initializing the escrow account
- Depositing tokens into the escrow
- Withdrawing tokens from the escrow
- Handling error cases (insufficient funds)
- Withdrawing the remaining balance

## Common Issues and Solutions
### Program ID Mismatch:

- Symptom: "Transaction simulation failed: This program may not be used for executing instructions"
- Solution: Ensure the program ID in declare_id! matches the deployed program ID

### PDA Seed Mismatch:

- Symptom: "Error processing Instruction" during withdraw
- Solution: Ensure PDA seeds in the withdraw instruction match the seeds used to create the escrow token account

### Token Account Not Found:

- Symptom: "TokenAccountNotFoundError"
- Solution: Verify that token accounts are properly initialized before operations

## License

MIT License
