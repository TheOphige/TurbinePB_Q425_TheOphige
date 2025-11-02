import * as anchor from '@project-serum/anchor';
import { PublicKey, Keypair, SystemProgram } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress } from '@solana/spl-token';
import fs from 'fs';


async function main() {
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const programId = new PublicKey('734juxRyhGmtjUXAXeLYLUG8ctGWULvyhnyhHb2WZhDG');
const program = new anchor.Program(/* IDL not strictly required for CPI here */
// minimal typed any
// @ts-ignore
null, programId, provider
);


const authority = provider.wallet.publicKey;
const mintPubkey = new PublicKey(process.env.MINT_ADDRESS!);


// derive vault state PDA
const [vaultStatePda, vaultStateBump] = await PublicKey.findProgramAddress(
[Buffer.from('vault-state'), authority.toBuffer(), mintPubkey.toBuffer()],
programId
);


// derive vault token ATA PDA (we'll use an associated token account owned by the PDA)
const [vaultPda, vaultBump] = await PublicKey.findProgramAddress(
[Buffer.from('vault'), authority.toBuffer(), mintPubkey.toBuffer()],
programId
);


const vaultTokenATA = await getAssociatedTokenAddress(mintPubkey, vaultPda, true);


console.log('Vault State PDA:', vaultStatePda.toBase58());
console.log('Vault PDA:', vaultPda.toBase58());
console.log('Vault Token ATA:', vaultTokenATA.toBase58());
console.log('Vault state bump:', vaultStateBump);
console.log('Vault bump:', vaultBump);


console.log('\nNote: To initialize the on-chain VaultState account you must call the Anchor `initialize_vault` instruction from a deployed program.');
}


main().catch(e => { console.error(e); process.exit(1); });