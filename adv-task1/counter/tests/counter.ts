import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { assert } from "chai";

describe("counter", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;

  let counterAccount = anchor.web3.Keypair.generate();

  it("Initializes the counter", async () => {
    await program.methods
      .initialize()
      .accounts({
        counter: counterAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterAccount])
      .rpc();

    const account = await program.account.counter.fetch(counterAccount.publicKey);
    assert.ok(account.count.toNumber() === 0);
  });

  it("Increments the counter", async () => {
    await program.methods
      .increment()
      .accounts({ counter: counterAccount.publicKey })
      .rpc();

    const account = await program.account.counter.fetch(counterAccount.publicKey);
    assert.ok(account.count.toNumber() === 1);
  });

  it("Decrements the counter", async () => {
    await program.methods
      .decrement()
      .accounts({ counter: counterAccount.publicKey })
      .rpc();

    const account = await program.account.counter.fetch(counterAccount.publicKey);
    assert.ok(account.count.toNumber() === 0);
  });
});
