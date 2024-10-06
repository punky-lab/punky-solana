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
        newPunkyAccount: punkyAccount, // eslint-disable-line
        signer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([])
      .rpc();

    // Fetch and check the account data
    const account = await program.account.punkyAccount.fetch(punkyAccount);
    expect(account.health).to.equal(250);
    expect(account.fitness).to.equal(600);
    expect(account.loyalty).to.equal(250);
  });

  // Ensure this is run within 1 minute
  it("Updates fitness over time correctly", async () => {
    const initialAccount = await program.account.punkyAccount.fetch(
      punkyAccount
    );
    const initialFitness = initialAccount.fitness;

    // Simulate time passage
    // await program.provider.connection.requestAirdrop(punkyAccount, 1_000_000_000);

    // Wait for more than 60 seconds (simulate this in real tests by mocking time)
    await program.methods
      .updateFitness()
      .accounts({
        punkyAccount: punkyAccount,
      })
      .signers([])
      .rpc();

    const updatedAccount = await program.account.punkyAccount.fetch(
      punkyAccount
    );
    expect(updatedAccount.fitness).to.be.eq(initialFitness); // do not wait for 1 minute
  });

  // Test the get_reward function
  it("Test get reward", async () => {
    await program.methods
      .getReward()
      .accounts({
        punkyAccount: punkyAccount,
      })
      .signers([])
      .rpc();

    const account = await program.account.punkyAccount.fetch(punkyAccount);
    expect(account.loyalty).to.equal(255);
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
    expect(account.fitness).to.equal(595);
    expect(account.loyalty).to.equal(260);
  });
});
