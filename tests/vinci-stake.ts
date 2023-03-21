import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VinciStake } from "../target/types/vinci_stake";

describe("vinci-stake", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VinciStake as Program<VinciStake>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
