Excellent — that’s exactly the level of detail you’d include in a **production-ready design doc** 👏

Here’s your **updated `DESIGN.md`** with the **enhanced User Flow** and **Detailed System Flow** sections fully integrated (everything else stays consistent).
You can copy-paste this directly into your repo — or I can regenerate your downloadable ZIP after you confirm.

---

# 🧱 DESIGN.md

**Project:** Solana Counter dApp
**Version:** 1.2
**Last Updated:** 2025-10-29

---

## 1️⃣ User Story — On-chain Counter App (Product Layer)

### **Title:**

On-chain Counter App

### **As a:**

Solana user (Bob)

### **I want to:**

Create and increment a personal counter on the Solana blockchain.

### **So that:**

I can track a persistent count value that lives on-chain and verify it through my wallet.

---

### **Acceptance Criteria (End-to-End Flow)**

#### **1. Connect to Solana**

* **Given** Bob opens the dApp in his browser,
* **When** he clicks **“Connect Wallet”**,
* **Then** the app prompts him to connect using a Solana wallet (e.g., Phantom, Solflare, Backpack).
* **And** once connected, the UI shows his wallet address and a “Create Counter” button.

#### **2. Create Counter Account**

* **Given** Bob’s wallet is connected,
* **When** he clicks **“Create Counter”**,
* **Then** the dApp sends a transaction to the Solana program to:

  * Create a **Program Derived Account (PDA)** to store Bob’s counter.
  * Initialize `count = 0`.
* **And** once confirmed,

  * The UI displays “Counter created successfully” and shows `Count: 0`.

#### **3. Increment Counter**

* **Given** Bob has an existing counter account,
* **When** he clicks **“Increment”**,
* **Then** the app sends a transaction to the program’s `increment` instruction.
* **And** the program:

  * Loads Bob’s counter account.
  * Adds `+1` to the `count` field.
  * Saves the updated value on-chain.
* **And** after confirmation,

  * The UI fetches the new `count` value via `getAccountInfo`
  * And updates the display in real-time (`Count: 1` → `Count: 2`, etc.).

#### **4. Error Handling**

* **If** Bob tries to increment before creating a counter,

  * The UI displays: “No counter found. Please create one first.”
* **If** a transaction fails (e.g., insufficient SOL or network issue),

  * The UI shows the specific error message from the Solana RPC response.

---

## 2️⃣ Overall User Flow

```
+---------------------+
|  User opens dApp    |
+----------+----------+
           |
           v
+---------------------------+
| Connect Wallet (Phantom) |
+-----------+---------------+
            |
     [Wallet Connected]
            |
            v
+-----------------------------+
| Create Counter Account Btn  |
+-------------+---------------+
              |
              v
+----------------------------------------------+
| Send "create" transaction to Solana Program  |
| - Creates account (PDA)                      |
| - Initializes count = 0                      |
+-----------------+----------------------------+
                  |
           [Transaction confirmed]
                  |
                  v
+-----------------------------+
| UI displays Count = 0       |
+-------------+---------------+
              |
              v
+-----------------------------+
| Click "Increment" button    |
+-------------+---------------+
              |
              v
+-------------------------------------------+
| Send "increment" transaction to Program   |
| - Loads Bob's counter                     |
| - Adds +1 to count                        |
| - Saves updated value                     |
+-----------------+-------------------------+
                  |
           [Transaction confirmed]
                  |
                  v
+-----------------------------+
| UI fetches and updates UI:  |
| Count: 1 → Count: 2 → ...   |
+-----------------------------+
```

---

## 3️⃣ Detailed System Flow (Actors + Actions)

```
┌────────────┐         ┌──────────────┐         ┌─────────────────────┐         ┌────────────────────┐
│   User     │         │  Web Client  │         │ Solana Wallet (e.g. │         │ Solana Program     │
│ (Bob)      │         │  (React app) │         │ Phantom / Solflare) │         │ (Anchor / Rust)    │
└─────┬──────┘         └──────┬──────┘         └──────────┬──────────┘         └──────────┬─────────┘
      │                        │                            │                            │
      │ Open dApp               │                            │                            │
      ├────────────────────────>│                            │                            │
      │ Click "Connect Wallet"  │                            │                            │
      ├────────────────────────>│                            │                            │
      │                        │ Request wallet connection  │                            │
      │                        ├───────────────────────────>│                            │
      │                        │                            │ User approves connection   │
      │                        │<───────────────────────────┤                            │
      │                        │                            │                            │
      │ Click "Create Counter" │                            │                            │
      ├────────────────────────>│ Build "create" transaction │                            │
      │                        ├───────────────────────────>│ Request signature          │
      │                        │                            │ User approves transaction  │
      │                        │<───────────────────────────┤                            │
      │                        │                            │                            │
      │                        │ Send tx to Solana RPC      │                            │
      │                        ├────────────────────────────────────────────────────────>│
      │                        │                            │ Create PDA, count=0        │
      │                        │<────────────────────────────────────────────────────────┤
      │                        │                            │                            │
      │                        │ Update UI: "Count = 0"     │                            │
      │                        │                            │                            │
      │ Click "Increment"      │                            │                            │
      ├────────────────────────>│ Build "increment" tx       │                            │
      │                        ├───────────────────────────>│ Request signature          │
      │                        │                            │ User approves              │
      │                        │<───────────────────────────┤                            │
      │                        │ Send tx to Solana RPC      │                            │
      │                        ├────────────────────────────────────────────────────────>│
      │                        │                            │ count = count + 1          │
      │                        │<────────────────────────────────────────────────────────┤
      │                        │ Fetch updated count        │                            │
      │                        │ Update UI: "Count = 1"     │                            │
      │                        │                            │                            │
```

---

## 4️⃣ Data Model

```
CounterAccount {
  user_pubkey: Pubkey,
  count: u64,
}
```

---

## 5️⃣ API / Smart Contract Interface

| Instruction   | Input           | Output              | Notes     |
| ------------- | --------------- | ------------------- | --------- |
| `create()`    | signer (user)   | counter initialized | count = 0 |
| `increment()` | counter_account | updated count       | count++   |

---

## 6️⃣ Error & Edge Cases

| Condition               | Error Code              | Expected UI Response          |
| ----------------------- | ----------------------- | ----------------------------- |
| Wallet not connected    | `WalletNotFound`        | Prompt to connect             |
| Account not initialized | `AccountNotInitialized` | Ask to create one first       |
| Insufficient SOL        | `InsufficientFunds`     | Show “Top up SOL to continue” |
| RPC timeout             | `NetworkError`          | Show retry option             |

---

## 7️⃣ Security Checklist

☑ Wallet connection verified
☑ Signer validation in smart contract
☑ PDA seeds verified
☑ Ownership + rent exemption enforced
☑ No unchecked arithmetic (use `checked_add`)
☑ RPC endpoint is trusted and rate-limited

---

## 8️⃣ QA Plan

| Scenario       | Steps                   | Expected Result            |
| -------------- | ----------------------- | -------------------------- |
| Create counter | Connect → Create        | count = 0                  |
| Increment      | Click “Increment”       | count += 1                 |
| Reload app     | Refresh page            | count persists             |
| Network error  | Disconnect RPC          | “Network Error” message    |
| Invalid wallet | Connect invalid keypair | Error displayed gracefully |

---

## 9️⃣ Summary

This document describes the **end-to-end functional design** of the Solana Counter dApp.
It defines user goals, detailed user/system flows, data structure, and interaction patterns between the **UI**, **wallet**, and **on-chain program**.

`ARCHITECTURE.md` should be read next for deeper technical implementation details and deployment pipeline.

---

Would you like me to regenerate your ZIP (`solana_counter_docs.zip`) with this new, more detailed `DESIGN.md` included so you can download it again?
