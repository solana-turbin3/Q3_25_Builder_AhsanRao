import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

// Define our Mint address
const mint = publicKey("8dCdY8EYCbSzypjaicGbwmZHu6UiNTEzuX5wNATmhDxD")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint: mint,
            mintAuthority: signer,
            updateAuthority: signer,
            payer: signer,
            systemProgram: publicKey("11111111111111111111111111111111"),
        }

        let data: DataV2Args = {
            name: "Ash Token",
            symbol: "MTK",
            uri: "https://example.com/metadata.json",
            sellerFeeBasisPoints: 500, // 5%
            creators: null, // No creators
            collection: null, // No collection
            uses: null, // No uses
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data: data,
            isMutable: true, // Metadata can be updated
            collectionDetails: null, // No collection details
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
