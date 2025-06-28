#[cfg(test)]
mod tests {
    use std::io;
    use std::io::BufRead;

    use solana_sdk::hash::hash;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::system_program;
    use solana_sdk::{
        message::Message,
        signature::{Keypair, read_keypair_file},
        signer::Signer,
        transaction::Transaction,
    };

    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};

    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";
    // "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

    // const pk = [
    //     52,
    //     9,
    //     105,
    //     246,
    //     198,
    //     241,
    //     56,
    //     207,
    //     32,
    //     172,
    //     78,
    //     64,
    //     191,
    //     70,
    //     122,
    //     83,
    //     28,
    //     199,
    //     183,
    //     239,
    //     22,
    //     156,
    //     21,
    //     252,
    //     155,
    //     162,
    //     32,
    //     25,
    //     237,
    //     155,
    //     104,
    //     50,
    //     221,
    //     181,
    //     218,
    //     6,
    //     10,
    //     150,
    //     184,
    //     102,
    //     8,
    //     152,
    //     64,
    //     108,
    //     252,
    //     234,
    //     14,
    //     105,
    //     53,
    //     95,
    //     193,
    //     237,
    //     48,
    //     186,
    //     10,
    //     22,
    //     223,
    //     174,
    //     5,
    //     103,
    //     129,
    //     174,
    //     85,
    //     119
    // ];

    // pubkey: FvTw3LodRKjzPBKToHhgXX7AaSZKAa1kLTqZgkfhjceS
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");

        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
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
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("couldn't find the file!");
        // we'll establish a connection to Solana devnet using the const we defined above
        let client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
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
    fn transfer_sol() {
        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Generate a signature from the keypair
        let pubkey = keypair.pubkey();

        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());

        // Verify the signature using the public key
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
        // Step 4: Define the destination (Turbin3) address
        let to_pubkey = Pubkey::from_str("Ajkkona22hnTBepa4WwCY9LFTih3nFRJYT8nafHRTbP4").unwrap();

        // Step 5: Connect to devnet
        let rpc_client = RpcClient::new(RPC_URL);

        // Step 6: Fetch recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Step 7: Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        // Step 8: Send the transaction and print tx
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn transfer_remaining_sol() {
        // step 1: Add Message to imports
        // use solana_sdk::{message::Message, signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};

        let keypair = read_keypair_file("dev-wallet.json").expect("couldn't find the file!");
        // we'll establish a connection to Solana devnet using the const we defined above
        let rpc_client = RpcClient::new(RPC_URL);

        // Step 2: Get current balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // 2.1 Define the destination (Turbin3) address
        let to_pubkey = Pubkey::from_str("Ajkkona22hnTBepa4WwCY9LFTih3nFRJYT8nafHRTbP4").unwrap();

        // 2.2 : Fetch recent blockhash
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
        // Step 5: Create final transaction with balance minus fee
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        // This ensures we leave zero lamports behind.
        // Step 6: Send transaction and verify
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");
        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);

        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        let mint = Keypair::new();

        let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();

        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();

        // get PDA
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];

        // Step 4: Get the PDA (Program Derived Address)
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

        println!("preq_pda => {} and bump {}", prereq_pda, _bump);

        // Step 5: Prepare the instruction data (discriminator)
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206]; // dicrimator

        let (authority, _bump) = Pubkey::find_program_address(&[b"collection", collection.as_ref()], &turbin3_prereq_program);

        println!("authority_pda => {} and bump {}", authority, _bump);

        // Step 6: Define the accounts metadata
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true), // user signer
            AccountMeta::new(prereq_pda, false), // PDA account
            AccountMeta::new(mint.pubkey(), true), // mint keypair
            AccountMeta::new(collection, false), // collection
            AccountMeta::new_readonly(authority, false), // authority (PDA)
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false), // system program
        ];

        // Step 7: Get the recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        println!("-------------------------recent_blockhash --------------------------");

        // Step 8: Build the instruction
        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };

        println!("-------------------------instruction => --------------------------");

        // Step 9: Create and sign the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            recent_blockhash,
        );

        println!("-------------------------55555555555555555555 --------------------------");

        // Step 10: Send and confirm the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        
        println!("Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet", signature);

    }
}
