import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../turbin3-wallet.json"
import base58 from "bs58";

// const RPC_ENDPOINT = "https://api.devnet.solana.com";
const RPC_ENDPOINT = "https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e"; // Helius RPC endpoint
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi, {
        mint,
        name: "Jrug",
        symbol: "JPR",
        uri: "https://gateway.irys.xyz/3xEz8ge1wf5RdYNzBAuFMcFr91hVy4Y1dTkeFzBP7No8",
        sellerFeeBasisPoints: percentAmount(2), // 2% royalty
        isMutable: true,
    });
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);

    // https://explorer.solana.com/tx/3KWDJtsvpPL6o2cmf31vV79BwT3msfWRcbSeemZDGRBfB4J243WbzpQ1D2NZSZsXWUH62YKtV7SMTytQHsZACBX5?cluster=devnet
    // Mint Address:  CbPw3oFU1WhDGDPnqwqtqbN2MiurdVbRBCizU9KjNbQU

    // JPrug NFT
    // https://explorer.solana.com/tx/3SAZE32SNmDgmyh8oTm1sFt3uywR85dr6n6WLrdvaSaciRUMxAfjSSoriYceJNkKUxKj3wy1aL6sPDsys1sn5gps?cluster=devnet
    // Mint Address:  C87HtvJH8NWWPAgDziE14aGTHpniAoaussMUtpVmx5ws
})();