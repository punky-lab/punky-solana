import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Punkystatus } from "../target/types/punkystatus";
import { RunnerGameReward } from "../target/types/runner_game_reward";
import {
    InitializeNewWorld,
    AddEntity,
    InitializeComponent,
    ApplySystem,
} from "@magicblock-labs/bolt-sdk"
import {expect} from "chai";

describe("PunkySolana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Constants used to test the program.
  let worldPda: PublicKey;
  let entityPda: PublicKey;
  let componentPda: PublicKey;

  const punkyStatus = anchor.workspace.punkystatus as Program<Punkystatus>;
  const runnerGameReward = anchor.workspace.RunnerGameReward as Program<RunnerGameReward>;

  it("InitializeNewWorld", async () => {
    const initNewWorld = await InitializeNewWorld({
      payer: provider.wallet.publicKey,
      connection: provider.connection,
    });
    const txSign = await provider.sendAndConfirm(initNewWorld.transaction);
    worldPda = initNewWorld.worldPda;
    console.log(`Initialized a new world (ID=${worldPda}). Initialization signature: ${txSign}`);
  });

  it("Add an entity", async () => {
    const addEntity = await AddEntity({
      payer: provider.wallet.publicKey,
      world: worldPda,
      connection: provider.connection,
    });
    const txSign = await provider.sendAndConfirm(addEntity.transaction);
    entityPda = addEntity.entityPda;
    console.log(`Initialized a new Entity (ID=${addEntity.entityPda}). Initialization signature: ${txSign}`);
  });

  it("Add a component", async () => {
    const initializeComponent = await InitializeComponent({
      payer: provider.wallet.publicKey,
      entity: entityPda,
      componentId: punkyStatus.programId,
    });
    const txSign = await provider.sendAndConfirm(initializeComponent.transaction);
    componentPda = initializeComponent.componentPda;
    console.log(`Initialized the grid component. Initialization signature: ${txSign}`);
  });

  it("Apply a system", async () => {
    // Check that the component has been initialized and x is 0
    const positionBefore = await punkyStatus.account.punkyStatus.fetch(
      componentPda
    );
    expect(positionBefore.x.toNumber()).to.equal(0);

    // Run the movement system
    const applySystem = await ApplySystem({
      authority: provider.wallet.publicKey,
      systemId: runnerGameReward.programId,
      entities: [{
        entity: entityPda,
        components: [{ componentId: punkyStatus.programId }],
      }],
      world: worldPda,
    });
    const txSign = await provider.sendAndConfirm(applySystem.transaction);
    console.log(`Applied a system. Signature: ${txSign}`);

    // Check that the system has been applied and x is > 0
    const positionAfter = await punkyStatus.account.punkyStatus.fetch(
      componentPda
    );
    expect(positionAfter.x.toNumber()).to.gt(0);
  });

});
