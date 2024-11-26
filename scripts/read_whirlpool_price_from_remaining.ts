import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RemainingAccountRead } from "../target/types/remaining_account_read";
import * as fs from "fs";
import * as path from "path";
import { PublicKey } from "@solana/web3.js";

async function main() {
  try {
    // Setup Provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // Load admin keypair
    const secretKeyPath = path.resolve(
      process.env.HOME!,
      ".config/solana/id.json"
    );
    const secretKeyString = fs.readFileSync(secretKeyPath, "utf8");
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const admin = anchor.web3.Keypair.fromSecretKey(secretKey);

    console.log("Admin PublicKey:", admin.publicKey.toBase58());

    // Initialize program
    const program = anchor.workspace.RemainingAccountRead as Program<RemainingAccountRead>;
    console.log("Program ID:", program.programId.toBase58());

    const whirlPool_WSOL_DEVUSDC = new PublicKey("3KBZiL2g8C7tiJ32hTv5v3KM7aK9htpqTw4cTXz1HvPt");
    const whirlPool_TMAC_DEVUSDC = new PublicKey("H3xhLrSEyDFm6jjG42QezbvhSxF5YHW75VdGUnqeEg5y");
    const mint_a = new PublicKey("So11111111111111111111111111111111111111112"); // WSOL
    const mint_b = new PublicKey("BRjpCHtyQLNCo8gqRUr8jtdAj5AjPYQaoqbvcZiHok1k"); //dev USDC

    const whirlpoolAddress = whirlPool_WSOL_DEVUSDC;



    if (!whirlpoolAddress) {
      throw new Error("Please provide WHIRLPOOL_ADDRESS environment variable");
    }

    console.log("Reading whirlpool data from:", whirlpoolAddress.toBase58());

    // Call the read_whirlpool_price instruction
    await program.methods
      .readWhirlpoolPrice()
      .accounts({
        signer: admin.publicKey,
      })
      .remainingAccounts([
        {
          pubkey: whirlpoolAddress,
          isWritable: false,
          isSigner: false
        },
        {
          pubkey: mint_a,
          isWritable: false,
          isSigner: false
        },
        {
          pubkey: mint_b,
          isWritable: false,
          isSigner: false
        }
      ])
      .signers([admin])
      .rpc();

    // Since the program logs the data, you should see it in the transaction logs
    console.log("Successfully read whirlpool data");

  } catch (error) {
    console.error("Error occurred:", error);
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});