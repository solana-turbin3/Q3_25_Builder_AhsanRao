import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const token_decimals = 1_000_000n;

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("8dCdY8EYCbSzypjaicGbwmZHu6UiNTEzuX5wNATmhDxD");

// Recipient address
const to = new PublicKey("8ZnGm4MKRnbyWoqM7WoFw2GNguQUv4Tt2HDaK4FMbp2d");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        // const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
        //     connection,
        //     keypair,
        //     mint,
        //     keypair.publicKey, // Owner of the token account
        //     true, // Allow owner to close the account
        //     commitment // Commitment level
        // );
        // console.log(`Your fromTokenAccount is: ${fromTokenAccount.address.toBase58()}`);

        const fromTokenAccount = new PublicKey("5aD4X7sxppWULq9cBsM69Dgv6rbSes7Qn3gUfvqGeheR");

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to, // Owner of the token account
            true, // Allow owner to close the account
            commitment // Commitment level
        );
        console.log(`Your toTokenAccount is: ${toTokenAccount.address.toBase58()}`);

        // Transfer the new token to the "toTokenAccount" we just created
        const transferTx = await transfer(
            connection,
            keypair,
            fromTokenAccount, // Source token account
            // fromTokenAccount.address,
            toTokenAccount.address, // Destination token account
            keypair.publicKey, // Owner of the source token account
            1n * token_decimals, // Amount to transfer (1 token)
            [], // Signers (none for now)
        );
        console.log(`Your transfer txid: ${transferTx}`);
        // 2tLxoUTMSdzEn2cjCz5oXzjUjpVsWuSDGjdC73wAKXJqc2sPg5kwbgrZev6YShLJR8bchAY1Bu7DGZDUP8PSsSRv

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();