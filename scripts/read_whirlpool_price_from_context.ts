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

    const whirlpoolAddress = new PublicKey("3KBZiL2g8C7tiJ32hTv5v3KM7aK9htpqTw4cTXz1HvPt");
    
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