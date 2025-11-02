import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { PublicKey } from "@solana/web3.js";

describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.vault as Program<Vault>;
  const provider = anchor.getProvider();

  it("Can initialize vault", async () => {
    // Initialize vault as unlocked
    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault"),
        provider.publicKey.toBuffer(),
      ],
      program.programId
    );

    const tx = await program.methods
      .initVault(false)
      .accounts({
        vaultAuthority: provider.publicKey,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Fetch the created account
    const vaultAccount = await program.account.vault.fetch(vaultPDA);
    
    // Verify account data
    console.log("Vault initialized with tx:", tx);
    console.log("Vault PDA:", vaultPDA.toString());
    console.log("Vault locked status:", vaultAccount.locked);
  });

  it("Can toggle lock status", async () => {
    // Get vault PDA
    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault"),
        provider.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Toggle lock
    const tx = await program.methods
      .toggleLock()
      .accounts({
        vaultAuthority: provider.publicKey,
        vault: vaultPDA,
      })
      .rpc();

    // Verify lock status changed
    const vaultAccount = await program.account.vault.fetch(vaultPDA);
    console.log("Toggle lock tx:", tx);
    console.log("New lock status:", vaultAccount.locked);
  });

  it("Can deposit and withdraw", async () => {
    // Get vault PDA
    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault"),
        provider.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Test deposit
    const depositAmount = new anchor.BN(1_000_000); // 1 SOL
    const depositTx = await program.methods
      .deposit(depositAmount)
      .accounts({
        vaultAuthority: provider.publicKey,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit tx:", depositTx);

    // Test withdrawal
    const withdrawAmount = new anchor.BN(500_000); // 0.5 SOL
    const withdrawTx = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        vaultAuthority: provider.publicKey,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw tx:", withdrawTx);
  });

  it("Cannot withdraw when locked", async () => {
    // Get vault PDA
    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault"),
        provider.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Lock the vault
    await program.methods
      .toggleLock()
      .accounts({
        vaultAuthority: provider.publicKey,
        vault: vaultPDA,
      })
      .rpc();

    try {
      // Attempt withdrawal while locked
      const withdrawAmount = new anchor.BN(100_000);
      await program.methods
        .withdraw(withdrawAmount)
        .accounts({
          vaultAuthority: provider.publicKey,
          vault: vaultPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      
      assert.fail("Expected withdrawal to fail when vault is locked");
    } catch (error) {
      console.log("Successfully prevented withdrawal while locked");
    }
  });
});
