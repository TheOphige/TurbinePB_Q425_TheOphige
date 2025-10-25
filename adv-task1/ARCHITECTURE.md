
# Architecture – Community Emergency Fund (CEF)

## System Overview

The Community Emergency Fund (CEF) consists of three main components:

1. **Frontend (UI Layer)** – A web interface where users create funds, donate SOL, and track activity.
2. **Anchor Program (On-chain Logic)** – A Solana smart contract that manages fund creation, donations, and withdrawals.
3. **Solana Blockchain (Storage Layer)** – Stores all fund states, transactions, and event logs immutably.

---

## Architecture Diagram



```
             ┌──────────────────────────┐
             │      Frontend UI         │
             │ (React + Wallet Adapter) │
             └────────────┬─────────────┘
                          │
                          ▼
             ┌──────────────────────────┐
             │   Anchor Smart Contract   │
             │  (initialize, donate,     │
             │   withdraw instructions)  │
             └────────────┬─────────────┘
                          │
                          ▼
             ┌──────────────────────────┐
             │     Solana Blockchain    │
             │ (PDAs, State, Logs, RPC) │
             └──────────────────────────┘
```



---

## Data Flow

1. **Fund Initialization**
   - Organizer connects their wallet and calls `initialize_fund`.
   - A Program Derived Account (PDA) is created with metadata (name, creator, total donations = 0).

2. **Donation**
   - Donor calls `donate` with SOL amount.
   - The program transfers SOL from donor to the Fund PDA and logs the transaction on-chain.

3. **Withdrawal**
   - Maintainer calls `withdraw`.
   - Program checks authority, transfers SOL to the maintainer’s wallet, and records the event.

4. **Auditing**
   - Any user can view fund balances and transaction history via RPC, Solana Explorer, or the frontend dashboard.

---

## Account Structure

| Account | Description |
|----------|-------------|
| **Fund** | PDA storing fund metadata, total donations, and maintainer address. |
| **Donations (Vec)** | List of donations with donor pubkey, amount, and timestamp. |
| **Maintainer** | Authorized account allowed to withdraw funds. |

---

## Security

- **PDA Authority** ensures only the maintainer can withdraw.
- **Immutable Ledger** provides public transparency.
- **Non-Custodial**: Users always control their keys and funds.

---

## Summary

The CEF architecture provides a simple, transparent crowdfunding framework:
- All actions (creation, donation, withdrawal) happen on-chain.  
- No intermediaries or off-chain custody.  
- Full auditability for donors, organizers, and observers.
```
