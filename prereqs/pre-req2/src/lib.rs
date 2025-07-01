#[cfg(test)]
mod tests {
    use solana_sdk;
    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
        
        // Create a new keypair
        let kp = Keypair::new();
        
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::{
            signature::{Keypair, Signer, read_keypair_file},
        };
        
        
        // Import our keypair
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        
        // Establish connection to Solana devnet
        let client = RpcClient::new(RPC_URL);
        
        // Claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn check_balance() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
        use solana_sdk::signature::read_keypair_file;
        
        // const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
        // const RPC_URL: &str = "https://api.devnet.solana.com";        
        // 
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        
        match client.get_balance(&keypair.pubkey()) {
            Ok(balance) => {
                println!("Wallet balance: {} lamports ({} SOL)", balance, balance as f64 / 1_000_000_000.0);
            }
            Err(err) => {
                println!("Error getting balance: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;
        
        // const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
        
        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        
        // Define the destination address
        let to_pubkey = Pubkey::from_str("87eaezi5Nou5d5MFH2DStENzWZ6iHNroDHZSbSca4RDu").unwrap();
        
        // Connect to devnet
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Fetch recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        // Create and sign the transaction (transfer 0.1 SOL = 100,000,000 lamports)
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        
        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn empty_wallet() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            message::Message,
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;
                
        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
        
        // Define the destination address
        let to_pubkey = Pubkey::from_str("87eaezi5Nou5d5MFH2DStENzWZ6iHNroDHZSbSca4RDu").unwrap();
        
        // Connect to devnet
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Step 2: Get current balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        
        println!("Current balance: {} lamports ({} SOL)", balance, balance as f64 / 1_000_000_000.0);
        
        if balance == 0 {
            println!("Wallet is already empty!");
            return;
        }
        
        // Fetch recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        // Step 3: Build a mock transaction to calculate fee
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        
        // Step 4: Estimate transaction fee
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        
        println!("Transaction fee: {} lamports", fee);
        
        // Check if we have enough balance to cover the fee
        if balance <= fee {
            println!("Balance ({} lamports) is not enough to cover transaction fee ({} lamports)", balance, fee);
            return;
        }
        
        let transfer_amount = balance - fee;
        println!("Transferring: {} lamports ({} SOL)", transfer_amount, transfer_amount as f64 / 1_000_000_000.0);
        
        // Step 5: Create final transaction with balance minus fee
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, transfer_amount)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        
        // Step 6: Send transaction and verify
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");
        
        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
        
        // Verify the wallet is now empty
        let final_balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get final balance");
        
        println!("Final wallet balance: {} lamports", final_balance);
    }

    #[test]
    fn base58_to_wallet() {
        use bs58;
        use std::io::{self, BufRead};
        
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        use bs58;
        use std::io::{self, BufRead};
        
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        
        println!("Your Base58-encoded private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn submit() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_program};
        use solana_sdk::{
            instruction::{AccountMeta, Instruction},
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;
                
        // Step 1: Create Solana RPC client
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Step 2: Load your signer keypair
        let signer = read_keypair_file("dev_wallet.json")
            .expect("Couldn't find wallet file");
        
        println!("Using wallet: {}", signer.pubkey());
        
        // Step 3: Define program and account public keys
        let mint = Keypair::new();
        let turbin3_prereq_program = 
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = 
            Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = 
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();
        
        // Step 4: Get the PDA (Program Derived Address)
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
        
        println!("PDA Account: {}", prereq_pda);
        
        // Get authority PDA (this might be needed for the instruction)
        let authority_seeds: &[&[u8]] = &[b"collection", collection.as_ref()];
        let (authority, _authority_bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);
        println!("Authority: {}", authority);

        // Step 5: Prepare the instruction data (discriminator for submit_rs)
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
        
        // Step 6: Define the accounts metadata
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),           // user (signer)
            AccountMeta::new(prereq_pda, false),              // PDA account
            AccountMeta::new(mint.pubkey(), true),            // mint keypair
            AccountMeta::new(collection, false),              // collection
            AccountMeta::new_readonly(authority, false),      // authority (PDA)
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false), // system program
        ];
        
        // Step 7: Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        // Step 8: Build the instruction
        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };
        
        // Step 9: Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );
        
        // Step 10: Send and confirm the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}