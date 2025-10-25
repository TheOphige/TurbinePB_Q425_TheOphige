
# Community Emergency Fund (CEF)

---

## Overview

The **Community Emergency Fund (CEF)** is a decentralized crowdfunding platform built on the **Solana blockchain** using the **Anchor framework**.

It enables communities to create transparent on-chain emergency funds where anyone can donate and where authorized maintainers can withdraw under verifiable and auditable conditions.  
By leveraging Solana’s high performance, low transaction costs, and transparent architecture, CEF provides a trustless, corruption-resistant alternative to traditional emergency relief systems.

---

## Key Features

- **Fund Creation:** Verified community organizers can initialize fund accounts (PDAs) with metadata such as fund name, description, and creator’s public key.
- **Transparent Donations:** Donors can contribute SOL directly to a fund; all transactions are stored immutably on-chain.
- **Controlled Withdrawals:** Only authorized maintainers can withdraw funds, ensuring accountability.
- **Public Auditing:** Anyone can view donation and withdrawal history to verify legitimacy.
- **Event Logging:** All major actions (fund creation, donation, withdrawal) emit Anchor events for easy tracking.

---

## Folder Structure

```

community-emergency-fund/
│
├── programs/
│   └── cef/
│       └── src/
│           └── lib.rs                  # Anchor program logic
│
├── tests/
│   └── cef.test.ts                     # Integration tests (TypeScript)
│
├── app/                                # Optional frontend (React/Next.js)
│
├── USER_STORY.md                       # Detailed user stories and roles
├── ARCHITECTURE.md                     # Architecture diagram and explanation
├── README.md                           # Overview, setup, and usage instructions
│
├── Anchor.toml                         # Anchor configuration
├── Cargo.toml                          # Rust dependencies
└── migrations/                         # Deployment scripts (if any)

````

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://book.anchor-lang.com/getting_started/installation.html)
- Node.js (for testing and optional frontend)
- A Solana wallet (e.g., Phantom)

---

### 1. Clone the Repository

```bash
git clone https://github.com/<your-username>/community-emergency-fund.git
cd community-emergency-fund
````

---

### 2. Build the Program

```bash
anchor build
```

This compiles the smart contract (`lib.rs`) into a Solana-compatible binary.

---

### 3. Deploy to Devnet

```bash
solana config set --url https://api.devnet.solana.com
anchor deploy
```

Copy the deployed program ID and update it in your `Anchor.toml` and client code.

---

### 4. Run Tests

```bash
anchor test
```

This executes integration tests written in TypeScript to validate all program instructions.

---


## Program Design Summary

* **Fund PDA:** Derived using `["fund", creator_pubkey]`, ensures each organizer has a unique on-chain fund.
* **Donation Instruction:** Executes a direct transfer from the donor to the fund PDA using `SystemProgram::transfer`.
* **Withdrawal Instruction:** Verifies maintainer authority and transfers requested SOL from the PDA.
* **Events:** Each major action emits events for off-chain indexing and analytics.

---

## Example Commands

Initialize a fund:

```bash
anchor run initialize_fund --name "Flood Relief Fund"
```

Donate to a fund:

```bash
anchor run donate --fund <FUND_PDA> --amount 1
```

Withdraw from a fund:

```bash
anchor run withdraw --fund <FUND_PDA> --amount 0.5
```

---

## Documentation Links

* [USERSTORY.md](./USERSTORY.md)
* [ARCHITECTURE.md](./ARCHITECTURE.md)

---

## License

This project is released under the **MIT License**.
Feel free to use, modify, and extend it for educational or community purposes.