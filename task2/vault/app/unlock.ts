import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { getAssociatedTokenAddress, createTransferInstruction } from '@solana/spl-token';


async function main() {
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const programId = new PublicKey('734juxRyhGmtjUXAXeLYLUG8ctGWULvyhnyhHb2WZhDG');