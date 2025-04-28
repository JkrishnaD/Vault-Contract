import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

describe("vault", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vault as Program<Vault>;
  const user = provider.wallet;

  const [vaultPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    program.programId
  )

  const [vaultStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), user.publicKey.toBuffer()],
    program.programId
  )

  it("should initialize the vault", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      user: user.publicKey,
    }).rpc();

    console.log("vault initialization signature", tx);

    const vaultState = await program.account.vaultState.fetch(vaultStatePda);

    expect(vaultState).to.not.be.null;

    expect(vaultState.vaultBump).to.be.greaterThan(0);
    expect(vaultState.stateBump).to.be.greaterThan(0);
  });

  it("should allow to deposite in vault", async () => {
    
    const amount = new anchor.BN(1_000_000_000); // 1 SOL

    const initialUserBalance = await provider.connection.getBalance(user.publicKey);
    const initialVaultBalance = await provider.connection.getBalance(vaultPda);

    try {
      const tx1 = await program.methods.deposite(amount).accounts({
        user: user.publicKey,
      }).rpc();

      console.log("vault deposite signature", tx1);

      const updatedBalance = await provider.connection.getBalance(user.publicKey);
      const updatedVaultBalance = await provider.connection.getBalance(vaultPda);

      expect(initialUserBalance).to.be.greaterThan(updatedBalance);
      expect(updatedVaultBalance).to.equal(initialVaultBalance + amount.toNumber());

      expect(updatedBalance - initialUserBalance).to.equal(amount.toNumber());
    } catch (error) {
      console.error("Error during deposit:", error);
      // If there are logs available, print them for debugging
      if (error.logs) {
        console.error("Transaction logs:", error.logs);
      }
      throw error;
    }
  })
});
