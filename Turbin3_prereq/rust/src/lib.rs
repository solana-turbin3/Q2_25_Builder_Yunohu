mod programs;

use solana_client::rpc_client::RpcClient;
use solana_program::{
    hash::hash,message::Message,pubkey::Pubkey,system_instruction::transfer,system_program,
};
use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},transaction::Transaction,
};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs::Turbin3_prereq::{TurbinePrereqProgram, CompleteArgs};

    #[test]
    fn keygen() {
        // creating a new keygen pair for airdrop
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        // Import the generated keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Connecting to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);
        // Request airdrop of 2 SOL (2 billion lamports) to my generated keygen pair
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // With the imported Keypair we are signing the message
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());
        // Verify the signature that we signed before 
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
        // should put my Turbin3 public key 
        let to_pubkey = Pubkey::from_str("GtdVVRzKKBhqTp1VxPvEbCs8iuFKWgJCiuTGNZB9fapm").unwrap();
        // Creating a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        // Get recent blockhash for the transaction
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        // Transfering 0.1 SOL  to my turbin3 wallet
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        // Send the transaction through the rpc url
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        // Print the transaction in the terminal for reference
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );

        // Emptying the devnet wallet into Turbin3 wallet by following the same as above but with putting balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Devnet wallet emptied. Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn submit_enrollment() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = TurbinePrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs { github: b"NVN404".to_vec() }; 
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let transaction = TurbinePrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
}