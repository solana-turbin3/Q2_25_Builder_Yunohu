import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "./dev-wallet.json"
// import my keypair from the dev wallet file 
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection =
    new Connection("https://api.devnet.solana.com");
(async () => {
    try {
        const txhash = await
            connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);
        //getting 2 sol to my dev wallet
        console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
