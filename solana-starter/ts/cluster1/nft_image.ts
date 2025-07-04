import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
// const umi = createUmi('https://api.devnet.solana.com');
const umi = createUmi('https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e')

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        //2. Convert image to generic file.
        //3. Upload image

        const image = await readFile("/Users/ash/Downloads/JPrug.png");
        const file = createGenericFile(image, "genjeff.png", { contentType: "image/png" });
        const [myUri] = await umi.uploader.upload([file]);
        console.log("Your image URI: ", myUri);
        // https://gateway.irys.xyz/A9CtoAz6D1CoCoBKMCeNTE7QvyY7n17BHGgtnkMLeuAZ
        // https://gateway.irys.xyz/C3y5MBJ2m4JpSLLiKmwWUbuAkt4h9MMvNVqE7pzyaaiv
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
