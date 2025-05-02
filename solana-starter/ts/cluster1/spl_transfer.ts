import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "./wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint addressspl

const mint = new PublicKey("DrPgbm1CdMe88wELFQknZt1nB2fQNmcRRG2YtkP1j2cJ");


// Recipient address
const to = new PublicKey("GtdVVRzKKBhqTp1VxPvEbCs8iuFKWgJCiuTGNZB9fapm");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromata = getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey)
        // Get the token account of the toWallet address, and if it does not exist, create it
        const toata = getOrCreateAssociatedTokenAccount(connection, keypair, mint, to)
        console.log("fromata", (await fromata).address)
        console.log("toata", (await toata).address)

        // Transfer the new token to the "toTokenAccount" we just created
        const tx = transfer(connection, keypair, (await fromata).address, (await toata).address, keypair, 100000000000 * 2)
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();