# ADV — Novel Use Case, User Story & Architecture Diagram

## Novel use case: **Community Emergency Micro-Grant (CEM-G)**

**Short description:**
A neighborhood micro-grants system where neighbors can pool tokens into a local vault and release micro-grants via escrow to recipients after community verification or time-delays. Tokens represent local currency (NEIGHBORHOOD-AID). The escrow mechanism enforces that funds are only released after an arbiter signs release or a community vote period elapses (time-lock). Transparency: every action (mint, lock, escrow deposit, release) is on-chain and auditable.

### User story

* **Actors:**

  * **Alice (Organizer)** — mints NEIGHBORHOOD-AID for community incentives.
  * **Neighborhood Vault** (vault PDA) — holds pooled funds.
  * **Bob (Applicant)** — requests funds for urgent support (e.g., medical bill).
  * **Community Arbiter (3 volunteers)** — can sign to release funds if they approve.
* **Flow:**

  1. Alice mints NHAID and deposits into the Neighborhood Vault (locked).
  2. Bob requests a micro-grant; organizer creates an escrow with amount X, taker=Bob, unlock_at = now + 72 hours, arbiter = optional set of pubkeys (multisig).
  3. Bob submits evidence; arbiter signs or the 72-hour timer expires.
  4. If arbiter(s) approve before timer, they call `complete` with their signature and tokens move to Bob. If no arbiter signature, after 72 hours `complete` can be called automatically (or by Bob) to release funds.
  5. All actions recorded on-chain; final tx hash inserted into repository.

### Architecture diagram (ASCII)

```
[ Alice Wallet ]                     [ Bob Wallet ]
      |                                    |
      | mint NHAID                          |
      v                                    v
[ NEIGHBORHOOD-AID Mint ]                 |
      |                                    |
      | deposit                          create escrow
      v                                    v
[ Neighborhood Vault (PDA, token acc) ] -> [ Escrow State (PDA) ]
           |                               | deposit tokens into
           |------------------------------>| escrow_token_account (PDA)
                                              |
                                   Condition: unlock_at OR arbiter sigs
                                              |
                               +--------------+---------------+
                               |                              |
                      [Arbiter Signatures]            [Time unlock expires]
                               |                              |
                               +--------------+---------------+
                                              |
                                        complete() -> transfer ->
                                              v
                                      [ Bob's Token Account ]
```

### impact

* Low-friction, transparent micro-grants for hyperlocal needs.
* Time-lock + arbiter signature gives flexible trust model (automatic fallback).
* Tokens minted represent local community credit and can be audited.
