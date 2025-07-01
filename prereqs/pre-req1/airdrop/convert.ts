import bs58 from 'bs58';
import promptSync from 'prompt-sync';
import wallet from './dev-wallet.json';
import fs from 'fs';

const input = promptSync();

// Convert wallet file (byte array) to base58 string (for Phantom)
function walletToBase58() {
    const wallet = fs.readFileSync('dev-wallet.json', 'utf8');
    const walletParse = JSON.parse(wallet);
    const base58 = bs58.encode(new Uint8Array(walletParse));
    console.log("Your wallet in base58 format (for Phantom):");
    console.log(base58);
}

// Convert base58 string back to wallet file format
function base58ToWallet() {
    console.log("Enter your base58 private key:");
    const base58 = "input";
    const wallet = Array.from(bs58.decode(base58));
    console.log("Your wallet in array format:");
    console.log(`[${wallet.join(',')}]`);
}

// Run the conversions
console.log("=== Wallet Format Converter ===");
walletToBase58();
// console.log("\n=== Convert Base58 Back ===");
// base58ToWallet();