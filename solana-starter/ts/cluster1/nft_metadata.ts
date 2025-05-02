import wallet from "./wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader({ providerUrl: 'https://devnet.irys.xyz' }));
umi.use(signerIdentity(signer));

(async () => {
    try {
        const image = "https://devnet.irys.xyz/88mysNsRaaq1BQs2gQTW3WBfiqF7kqi6vN49N69RRmgk";

        const metadata = {
            name: "YUNOHU",
            symbol: "YNHU",
            description: "2025* limited edition",
            image: "https://devnet.irys.xyz/88mysNsRaaq1BQs2gQTW3WBfiqF7kqi6vN49N69RRmgk",
            attributes: [
                { trait_type: 'trait1', value: 'theone' }
            ],
            properties: {
                files: [
                    {
                        type: "image/jpeg",
                        uri: "https://devnet.irys.xyz/88mysNsRaaq1BQs2gQTW3WBfiqF7kqi6vN49N69RRmgk"
                    }
                ]
            },
            creators: []
        };

        // Debug print
        console.log("Metadata JSON:", JSON.stringify(metadata, null, 2));

        // Upload
        const [myUri] = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI:", myUri);
        const [data] = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI:", data);
        const [ravi] = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI:", ravi);
        const [juju] = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI:", juju);

    }

    catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
