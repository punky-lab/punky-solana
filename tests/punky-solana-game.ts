import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PunkySolanaAnchor } from "../target/types/punky_solana_anchor";
import { expect } from "chai";

describe("punky-solana-anchor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .PunkySolanaAnchor as Program<PunkySolanaAnchor>;

  // Define accounts and keypairs for testing
  const signer = provider.wallet;
  let punkyAccount: PublicKey;

  // Test the initialize function
  it("Initializes a Punky account", async () => {
    // Find the PDA for the Punky account
    [punkyAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("punky"), signer.publicKey.toBuffer()],
      program.programId
    );

    // Invoke the initialize method
    await program.methods
      .initialize()
      .accounts({
        newPunkyAccount: punkyAccount,
        signer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([])
      .rpc();

    // Fetch and check the account data
    const account = await program.account.punkyAccount.fetch(punkyAccount);
    expect(account.health).to.equal(25);
    expect(account.fitness).to.equal(50);
    expect(account.loyalty).to.equal(25);
  });

  // Test the get_reward function
  it("Increases loyalty by 5", async () => {
    await program.methods
      .getReward()
      .accounts({
        punkyAccount: punkyAccount,
      })
      .signers([])
      .rpc();

    const account = await program.account.punkyAccount.fetch(punkyAccount);
    expect(account.loyalty).to.equal(30);
  });

  // Test the run_one_second function
  it("Modifies fitness and loyalty correctly", async () => {
    await program.methods
      .runOneSecond()
      .accounts({
        punkyAccount: punkyAccount,
      })
      .signers([])
      .rpc();

    const account = await program.account.punkyAccount.fetch(punkyAccount);
    expect(account.fitness).to.equal(45);
    expect(account.loyalty).to.equal(35);
  });

  // Additional tests to cover edge cases
  it("Handles loyalty exceeding the limit", async () => {
    // Assuming current loyalty is 95
    for (let i = 0; i < 14; i++) {
      await program.methods
        .getReward()
        .accounts({
          punkyAccount: punkyAccount,
        })
        .signers([])
        .rpc();
    }

    const account = await program.account.punkyAccount.fetch(punkyAccount);
    expect(account.loyalty).to.equal(100);
  });

  it("Ensures fitness does not go below zero", async () => {
    // Assuming current fitness is low enough to trigger the condition
    for (let i = 0; i < 10; i++) {
      await program.methods
      .runOneSecond()
      .accounts({
        punkyAccount: punkyAccount,
      })
      .signers([])
      .rpc();
    }

    const account = await program.account.punkyAccount.fetch(punkyAccount);
    expect(account.fitness).to.be.gte(0);
  });
});
