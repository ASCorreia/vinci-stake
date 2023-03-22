import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VinciStake } from "../target/types/vinci_stake";

describe("vinci-stake", () => {
  // Configure the client to use the local cluster.
  const key = anchor.AnchorProvider.env()
  anchor.setProvider(key);

  const program = anchor.workspace.VinciStake as Program<VinciStake>;

  it("Is initialized!", async () => {
    const [vinciWorldStake, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("VinciWorldStakePool"),
        key.wallet.publicKey.toBuffer(),
      ],
      program.programId
      );
    // Add your test here.
    const tx = await program.methods.initializeStakePool().accounts({
      stakePool: vinciWorldStake,
      user: key.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
