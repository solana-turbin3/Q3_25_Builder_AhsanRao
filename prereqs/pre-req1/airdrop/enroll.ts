import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json"

const SYSTEM_PROGRAM_ID = SystemProgram.programId;
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

// Create anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {commitment: "confirmed"});

// Create our program
const program : Program<Turbin3Prereq> = new Program(IDL, provider);

// Create the PDA for prereq account
const account_seeds = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer(),
];

// Collection mint address
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");

// Generate new mint for the NFT
const mintTs = Keypair.generate();

const [account_key, _account_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId);

// Create PDA for authority
const authority_seeds = [
    Buffer.from("collection"),
    mintCollection.toBuffer(),
];
const [authority_key, _authority_bump] = PublicKey.findProgramAddressSync(authority_seeds, program.programId);


// Initialize transaction
async function initialize() {
    try {
        const txhash = await program.methods
            .initialize("AhsanRao")
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .signers([keypair])
            .rpc();
        
        console.log(`Initialize Success! Check out your TX here:`);
        console.log(`https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Initialize failed: ${e}`);
    }
}

// Submit TypeScript prereqs transaction
async function submitTs() {
    try {
                console.log("Available methods:", Object.keys(program.methods));

        const txhash = await program.methods
            .submitTs()
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                mint: mintTs.publicKey,
                collection: mintCollection,
                authority: authority_key,
                mpl_core_program: MPL_CORE_PROGRAM_ID,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .signers([keypair, mintTs])
            .rpc();
            
        console.log(`Submit TS Success! Check out your TX here:`);
        console.log(`https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Submit TS failed: ${e}`);
    }
}

// Run the functions
async function main() {
    console.log("Starting Turbin3 enrollment...");
    console.log("Wallet:", keypair.publicKey.toBase58());
    console.log("Program ID:", program.programId.toBase58());
    console.log("Account PDA:", account_key.toBase58());
    console.log("Authority PDA:", authority_key.toBase58());
    
    // Run initialize first
    console.log("\n=== Running Initialize ===");
    await initialize();
    
    // Wait a bit before running submitTs
    console.log("\nWaiting 5 seconds before submitting...");
    await new Promise(resolve => setTimeout(resolve, 5000));
    
    // Run submitTs
    console.log("\n=== Running Submit TS ===");
    await submitTs();
}

// Uncomment to run both transactions
// main();

// Or run them separately by uncommenting one at a time:
// initialize();
submitTs();