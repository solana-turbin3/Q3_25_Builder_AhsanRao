import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../turbin3-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("8dCdY8EYCbSzypjaicGbwmZHu6UiNTEzuX5wNATmhDxD");

(async () => {
    try {
        // Create an ATA
        const ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey, // Owner of the ATA
            true, // Allow owner to close the account
            commitment // Commitment level
        );
        console.log(`Your ata is: ${ata.address.toBase58()}`);

        // Mint to ATA
        const mintTx = await mintTo(
            connection,
            keypair,
            mint,
            ata.address, // ATA address
            keypair.publicKey, // Mint authority
            10n * token_decimals, // Amount to mint (1 token)
            [], // Signers (none for now)
        );
        console.log(`Your mint txid: ${mintTx}`);
        // 3DDSAK9NvTj7c9WFuWjgutickyMbzwG7RiUWLH3MR9HVebxwCZJnFMaxFxV6oGt5DEb1M1VpRJHfH5sG9rBRjTHi
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
