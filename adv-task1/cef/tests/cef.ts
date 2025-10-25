import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CommunityEmergencyFund } from "../target/types/community_emergency_fund";
import { assert } from "chai";

describe("Community Emergency Fund", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.CommunityEmergencyFund as Program<CommunityEmergencyFund>;

  // Test wallets
  const creator = provider.wallet;
  const donor = anchor.web3.Keypair.generate();

  // Fund PDA and bump
  let fundPda: anchor.web3.PublicKey;
  let bump: number;

  before(async () => {
    // Airdrop SOL to donor for testing
    const sig = await provider.connection.requestAirdrop(donor.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(sig);
  });

  it("Initializes a community emergency fund", async () => {
    [fundPda, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("fund"), creator.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeFund("Flood Relief Fund", "A transparent emergency fund for flood victims")
      .accounts({
        fund: fundPda,
        creator: creator.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const fundAccount = await program.account.fund.fetch(fundPda);

    assert.equal(fundAccount.name, "Flood Relief Fund");
    assert.equal(fundAccount.creator.toBase58(), creator.publicKey.toBase58());
    assert.equal(fundAccount.totalDonations.toNumber(), 0);
  });

  it("Allows donors to contribute to the fund", async () => {
    const donationAmount = 0.5 * anchor.web3.LAMPORTS_PER_SOL;

    await program.methods
      .donate(new anchor.BN(donationAmount))
      .accounts({
        fund: fundPda,
        donor: donor.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([donor])
      .rpc();

    const fundAccount = await program.account.fund.fetch(fundPda);

    assert.equal(fundAccount.totalDonations.toNumber(), donationAmount);
    assert.equal(fundAccount.donorCount.toNumber(), 1);
  });

  it("Allows the fund maintainer to withdraw", async () => {
    const withdrawAmount = 0.2 * anchor.web3.LAMPORTS_PER_SOL;
    const recipient = anchor.web3.Keypair.generate();

    const preBalance = await provider.connection.getBalance(recipient.publicKey);

    await program.methods
      .withdraw(new anchor.BN(withdrawAmount), "Emergency medical supplies")
      .accounts({
        fund: fundPda,
        creator: creator.publicKey,
        recipient: recipient.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const postBalance = await provider.connection.getBalance(recipient.publicKey);
    const fundAccount = await program.account.fund.fetch(fundPda);

    assert.isTrue(postBalance > preBalance);
    assert.equal(
      fundAccount.totalWithdrawals.toNumber(),
      withdrawAmount,
      "Withdrawal should be recorded on-chain"
    );
  });

  it("Prevents unauthorized withdrawals", async () => {
    const malicious = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .withdraw(new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL), "Unauthorized test")
        .accounts({
          fund: fundPda,
          creator: malicious.publicKey,
          recipient: malicious.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([malicious])
        .rpc();
      assert.fail("Unauthorized withdrawal should fail");
    } catch (err) {
      assert.include(err.toString(), "ConstraintSeeds");
    }
  });
});
