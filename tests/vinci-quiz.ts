import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VinciQuiz } from "../target/types/vinci_quiz";
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { BN } from "bn.js";

describe("vinci-quiz", () => {
  // Configure the client to use the local cluster.
  const key = anchor.AnchorProvider.env();
  anchor.setProvider(key);

  const program = anchor.workspace.VinciQuiz as Program<VinciQuiz>;
  
  const keypair = Keypair.generate();
  const keypair2 = Keypair.generate();

  let vinciQuizPDA = findProgramAddressSync([anchor.utils.bytes.utf8.encode("VinciWorldQuiz")], program.programId);
  console.log("\n\nVinci Quiz account: ", vinciQuizPDA[0].toBase58());
  console.log("Vinci Quiz account bump: ", vinciQuizPDA[1]);

  it("Request Airdrop to Keypair 1", async () => {
    // Add your test here.
    console.log("\nRequesting airdrop for ", keypair.publicKey.toBase58());
    const airdropTx = await key.connection.requestAirdrop(keypair.publicKey, 0.5 * LAMPORTS_PER_SOL);
    let latestBlockHash = await key.connection.getLatestBlockhash();
    await key.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropTx,
    })
    console.log("Airdrop sent to player 1 - TxID: ", airdropTx);
  });

  it("Request Airdrop to Keypair 2", async () => {
    // Add your test here.
    console.log("\nRequesting airdrop for ", keypair2.publicKey.toBase58());
    const airdropTx2 = await key.connection.requestAirdrop(keypair2.publicKey, 0.5 * LAMPORTS_PER_SOL);
    let latestBlockHash2 = await key.connection.getLatestBlockhash();
    await key.connection.confirmTransaction({
      blockhash: latestBlockHash2.blockhash,
      lastValidBlockHeight: latestBlockHash2.lastValidBlockHeight,
      signature: airdropTx2,
    })
    console.log("Airdrop sent to player 2 - TxID: ", airdropTx2);
  });

  it("Initialize Vinci Quiz Season", async () => {
    console.log("\n\nInitialing Vinci Quiz Season\n");
    console.log("User: ", keypair.publicKey.toString());
    console.log("Vinci Quiz Season PDA: ", vinciQuizPDA[0].toString())

    const tx = await program.methods.initialize().accounts({
      vinciQuiz: vinciQuizPDA[0],
      user: keypair.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([keypair]).rpc({skipPreflight: true});

    console.log("\nVinci Quiz Season Successfully Initialized - TxID: ", tx);
  })

  it("Add player to Vinci Quiz Season", async () => {
    console.log("\n\nAdding player to Vinci Quiz Season\n");

    const tx = await program.methods.addPlayer().accounts({
      vinciQuiz: vinciQuizPDA[0],
      user: keypair.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([keypair]).rpc();

    console.log("Player 1 added to Vinci Quiz Season - TxID: ", tx);

    const tx2 = await program.methods.addPlayer().accounts({
      vinciQuiz: vinciQuizPDA[0],
      user: keypair2.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([keypair2]).rpc();

    console.log("Player 2 added to Vinci Quiz Season - TxID: ", tx2);

    const tournament = (await program.account.quizSeason.fetch(vinciQuizPDA[0])).tournament;
    const entries = (await program.account.quizSeason.fetch(vinciQuizPDA[0])).entries;

    for (let i = 0; i < entries; i++) {
      console.log("\nEntry number ", i +1, " user: ", tournament[i].user.toString());
      console.log("Entry number ", i +1, " score: ", tournament[i].score);
    }
  })

  it("Update player score", async () => {
    console.log("\n\nUpdating Player Score\n");

    const tx = await program.methods.updateScore(true).accounts({
      vinciQuiz: vinciQuizPDA[0],
      user: keypair.publicKey,
    }).rpc();

    console.log("Player 1 score updated - TxID: ", tx, "\n");

    const tx2 = await program.methods.updateScore(true).accounts({
      vinciQuiz: vinciQuizPDA[0],
      user: keypair2.publicKey,
    }).rpc();

    console.log("Player 2 score updated - TxID: ", tx2, "\n");

    const tx3 = await program.methods.updateScore(false).accounts({
      vinciQuiz: vinciQuizPDA[0],
      user: keypair2.publicKey,
    }).rpc();

    console.log("Player 2 score updated - TxID: ", tx3, "\n");

    const tournament = (await program.account.quizSeason.fetch(vinciQuizPDA[0])).tournament;
    const entries = (await program.account.quizSeason.fetch(vinciQuizPDA[0])).entries;

    for (let i = 0; i < entries; i++) {
      console.log("Entry number ", i +1, " user: ", tournament[i].user.toString());
      console.log("Entry number ", i +1, " score: ", tournament[i].score, "\n");
    } 
  })

  it("Close Vinci Quiz Season and refund rent to wallet", async () => {
    console.log("\n\nClosing Vinci Quiz Season");
    console.log("Wallet funds before closing account: ", await key.connection.getBalance(keypair.publicKey));
    const tx = await program.methods.closeSeason().accounts({
      vinciQuiz: vinciQuizPDA[0],
      destination: keypair.publicKey,
    }).rpc();
    console.log("\nVinci Quiz Season closed - TxID: ", tx);
    console.log("Wallet funds after closing account: ", await key.connection.getBalance(keypair.publicKey));
  })
});
