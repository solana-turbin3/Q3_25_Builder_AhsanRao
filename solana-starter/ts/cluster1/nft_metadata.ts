import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { read } from "fs";

// Create a devnet connection
// const umi = createUmi('https://api.devnet.solana.com');
const umi = createUmi('https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

// umi.use(irysUploader({address: "https://api.devnet.irys.network"}));
umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        // const image = "https://gateway.irys.xyz/C3y5MBJ2m4JpSLLiKmwWUbuAkt4h9MMvNVqE7pzyaaiv"
        const image = "https://gateway.irys.xyz/F5X59EHtNWe6vB2uB9yU6GCWXstXPxySAZbwgQo7ZJ4V"
        const metadata = {
            name: "JPrug",
            symbol: "JPR",
            description: "Turbin3 NFT rug day",
            image: image,
            attributes: [
                {trait_type: 'hat', value: 'missing'},
                {trait_type: 'shirt', value: 'grey'},
                {trait_type: 'face', value: 'sad'},
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
        // Jcat metadata URI:
        // https://gateway.irys.xyz/4wuWedwBBQsHVVokB8op9rt5hzAr91MDCRA2yhHh6gtS
        // JPrug metadata URI:
        // https://gateway.irys.xyz/3xEz8ge1wf5RdYNzBAuFMcFr91hVy4Y1dTkeFzBP7No8 
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
