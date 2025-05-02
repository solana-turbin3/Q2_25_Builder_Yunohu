import {
    Transaction, SystemProgram, Connection, Keypair,
    LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey
} from
    "@solana/web3.js"
import wallet from "./dev-wallet.json"
const from = Keypair.fromSecretKey(new Uint8Array(wallet));
//my turbin3 wallet public key to transfer the sol from dev wallet to turbin3 wallet
const to = new
    PublicKey("GtdVVRzKKBhqTp1VxPvEbCs8iuFKWgJCiuTGNZB9fapm");
const connection = new Connection("https://api.devnet.solana.com");
(async () => {
    try {
        const balance = await connection.getBalance(from.publicKey)

        // to calculate fees (imp)
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance,
            })
        );
        transaction.recentBlockhash = (await
            connection.getLatestBlockhash('confirmed')).blockhash;
        transaction.feePayer = from.publicKey;
        // to Calculate exact fee rate to transfer entire SOL amount out of account minus fees

        const fee = (await
            connection.getFeeForMessage(transaction.compileMessage(),
                'confirmed')).value || 0;
        transaction.instructions.pop();

        transaction.add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance - fee,
            })
        );
        // Sign transaction, broadcast, and confirm
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        );
        console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${signature}?cluster=devnet`)
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
