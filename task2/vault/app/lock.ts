import * as anchor from '@project-serum/anchor';
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress, createTransferInstruction } from '@solana/spl-token';
import { PublicKey, Keypair } from '@solana/web3.js';


async function main() {
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const programId = new PublicKey('734juxRyhGmtjUXAXeLYLUG8ctGWULvyhnyhHb2WZhDG');


const authority = provider.wallet.publicKey; // this is the wallet that created vault
const mintPubkey = new PublicKey(process.env.MINT_ADDRESS!);


// derive PDAs same as program
const [vaultStatePda, _vaultStateBump] = await PublicKey.findProgramAddress([
Buffer.from('vault-state'), authority.toBuffer(), mintPubkey.toBuffer()
], programId);
const [vaultPda, vaultBump] = await PublicKey.findProgramAddress([
Buffer.from('vault'), authority.toBuffer(), mintPubkey.toBuffer()
], programId);


const user = provider.wallet.publicKey;
const userTokenAccount = await getAssociatedTokenAddress(mintPubkey, user);
const vaultTokenAccount = await getAssociatedTokenAddress(mintPubkey, vaultPda, true);


// amount to lock (in raw smallest units)
const amount = BigInt(process.env.LOCK_AMOUNT || '1000000000');


// build transfer instruction from userTokenAccount -> vaultTokenAccount
const transferIx = createTransferInstruction(userTokenAccount, vaultTokenAccount, user, Number(amount));


const tx = new anchor.web3.Transaction().add(transferIx);
tx.feePayer = user;
tx.recentBlockhash = (await provider.connection.getLatestBlockhash()).blockhash;


const signed = await provider.wallet.signTransaction(tx);
const sig = await provider.connection.sendRawTransaction(signed.serialize());
console.log('Lock tx signature:', sig);
await provider.connection.confirmTransaction(sig);
}


main().catch(e => { console.error(e); process.exit(1); });