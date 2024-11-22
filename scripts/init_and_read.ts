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

    // Derive invest tracker PDA
    const [investTracker] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("invest_tracker"),
      ],
      program.programId
    );

    console.log("Invest Tracker PDA:", investTracker.toBase58());

    // Initialize invest tracker
    console.log("Initializing invest tracker...");
    await program.methods
      .initInvestTracker()
      .accounts({
        signer: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    console.log("Invest tracker initialized successfully");

    // Read from remaining account
    console.log("Reading from remaining account...");
    await program.methods
      .readFromRemaining()
      .accounts({
        signer: admin.publicKey,
      })
      .remainingAccounts([
        {
          pubkey: investTracker,
          isWritable: true,
          isSigner: false,
        },
      ])
      .signers([admin])
      .rpc();

    console.log("Successfully read from remaining account");

  } catch (error) {
    console.error("Error occurred:", error);
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});