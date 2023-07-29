import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { VinciSwap } from "../target/types/vinci_swap";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { Keypair, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import { BN } from "bn.js";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";

describe("vinci-swap", () => {
  // Configure the client to use the local cluster.
  const key = anchor.AnchorProvider.env();
  anchor.setProvider(key);

  const program = anchor.workspace.VinciSwap as Program<VinciSwap>;

  const keypair = anchor.web3.Keypair.generate();

  const vinciSwapPDA = findProgramAddressSync([anchor.utils.bytes.utf8.encode("VinciSwap")], program.programId);

  const mint_keypair = Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      vinciSwap: vinciSwapPDA[0],
      user: key.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId}).rpc();
    console.log("Vinci Swap Account Created - TxID: ", tx);
  });

  it("Add Token to Pool, provide liquidity and swap Tokens", async () => {
    // Add your test here.
    console.log("\nRequesting airdrop for ", keypair.publicKey.toBase58());
    const airdropTx = await key.connection.requestAirdrop(keypair.publicKey, 1 * LAMPORTS_PER_SOL);
    let latestBlockHash = await key.connection.getLatestBlockhash();
    await key.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropTx,
    })
    console.log("Airdrop sent - TxID: ", airdropTx);

    //Create first Token
    const mint = await spl.createMint(key.connection, keypair, keypair.publicKey, keypair.publicKey, 6);
    console.log("\nFirst Created mint is: ", mint.toBase58());
    const tx = await program.methods.addToken().accounts({
      vinciSwap: vinciSwapPDA[0],
      mint: mint,
      payer: key.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
    console.log("First Token mint added to Liquidity Pool - TxID: ", tx);

    //Create second Token
    const mint2 = await spl.createMint(key.connection, keypair, keypair.publicKey, keypair.publicKey, 6);
    console.log("Second Created mint is: ", mint2.toBase58());
    const tx2 = await program.methods.addToken().accounts({
      vinciSwap: vinciSwapPDA[0],
      mint: mint2,
      payer: key.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
    console.log("Second Token mint added to Liquidity Pool - TxID: ", tx2);

    //Crete ATA for first token and mint some tokens
    const ownerAta1 = await spl.getOrCreateAssociatedTokenAccount(key.connection, keypair, mint, keypair.publicKey);
    console.log("\nOwner ATA for mint 1 created: ", ownerAta1.address.toBase58());
    const mintTx1 = await spl.mintTo(key.connection, keypair, mint, ownerAta1.address, keypair, 2 * LAMPORTS_PER_SOL);
    console.log("Token 1 minted to Owner Account - TxID: ", mintTx1);
    
    //Crete ATA for second token and mint some tokens
    const ownerAta2 = await spl.getOrCreateAssociatedTokenAccount(key.connection, keypair, mint2, keypair.publicKey);
    console.log("\nOwner ATA for mint 2 created: ", ownerAta2.address.toBase58());
    const mintTx2 = await spl.mintTo(key.connection, keypair, mint2, ownerAta2.address, keypair, 2 * LAMPORTS_PER_SOL);
    console.log("Token 2 minted to Owner Account - TxID: ", mintTx2);

    //Get Pool ata for first Token
    const vaultAta1 = await spl.getAssociatedTokenAddress(mint, vinciSwapPDA[0], true);

    //Get Pool ata for second Token
    const vaultAta2 = await spl.getAssociatedTokenAddress(mint2, vinciSwapPDA[0], true);

    //Send Token 1 to liquidity pool
    console.log("\nSending tokens from first Token to Liquidity Pool");
    const liquidityTx = await program.methods.addLiquidity(new BN(1 * LAMPORTS_PER_SOL)).accounts({
      vinciSwap: vinciSwapPDA[0],
      ownerAta: ownerAta1.address,
      vaultAta: vaultAta1,
      tokenMint: mint,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      user: keypair.publicKey,
    }).signers([keypair]).rpc();
    console.log(1 * LAMPORTS_PER_SOL, " Tokens with mint ID ", mint.toBase58(), " Successfuly sent to the Program Vault / Liquidity Pool")
    console.log("TxID: ", liquidityTx);

    //Send Token 2 to liquidity pool
    console.log("\nSending tokens from second Token to Liquidity Pool");
    const liquidityTx2 = await program.methods.addLiquidity(new BN(1 * LAMPORTS_PER_SOL)).accounts({
      vinciSwap: vinciSwapPDA[0],
      ownerAta: ownerAta2.address,
      vaultAta: vaultAta2,
      tokenMint: mint2,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      user: keypair.publicKey,
    }).signers([keypair]).rpc();
    console.log(1 * LAMPORTS_PER_SOL, " Tokens with mint ID ", mint2.toBase58(), " Successfuly sent to the Program Vault / Liquidity Pool")
    console.log("TxID: ", liquidityTx2);

    //Check mints saved in liquidity pool
    let fetchedAccount = await program.account.vinciSwap.fetch(vinciSwapPDA[0]);
    console.log("");
    for (let i = 0; i < fetchedAccount.assets.length; i++) {
      console.log("Liquidity Pool mint ", i + 1, " is ", fetchedAccount.assets[i].toBase58());
    }

    //Perform Swap from Token 2 to Token 1
    console.log("\nPerforming Swap from Token 2 (", 0.5 * LAMPORTS_PER_SOL, ") to Token 1");
    let accountBalance1BeforeSwap = (await key.connection.getTokenAccountBalance(ownerAta1.address)).value.amount;
    console.log("User Token 1 SPL Balance before swap: ", accountBalance1BeforeSwap);
    let accountBalance2BeforeSwap = (await key.connection.getTokenAccountBalance(ownerAta2.address)).value.amount;
    console.log("User Token 2 SPL Balance before swap: ", accountBalance2BeforeSwap);
    let swapTx = await program.methods.swap(new BN(0.5 * LAMPORTS_PER_SOL)).accounts({
      vinciSwap: vinciSwapPDA[0],
      userReceiveMint: mint,
      userReceiveTokenAccount: ownerAta1.address,
      poolReceiveTokenAccount: vaultAta1,
      userPayMint: mint2,
      userPayTokenAccount: ownerAta2.address,
      poolPayTokenAccount: vaultAta2,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      user: keypair.publicKey,
    }).signers([keypair]).rpc();
    console.log("Swap from Token 2 to Token 1 performed");
    console.log("TxID: ", swapTx);
    let accountBalance1AfterSwap = (await key.connection.getTokenAccountBalance(ownerAta1.address)).value.amount;
    console.log("User Token 1 SPL Balance after swap: ", accountBalance1AfterSwap);
    let accountBalance2AfterSwap = (await key.connection.getTokenAccountBalance(ownerAta2.address)).value.amount;
    console.log("User Token 2 SPL Balance after swap: ", accountBalance2AfterSwap);
    console.log("Amount received from swap: ", (accountBalance1AfterSwap as any) - (accountBalance1BeforeSwap as any));

    //Perform another Swap from Token 2 to Token 1
    console.log("\nPerforming another Swap (same values) from Token 2 (", 0.5 * LAMPORTS_PER_SOL, ") to Token 1");
    let accountBalance1BeforeSwap2 = (await key.connection.getTokenAccountBalance(ownerAta1.address)).value.amount;
    console.log("User Token 1 SPL Balance before swap: ", accountBalance1BeforeSwap2);
    let accountBalance2BeforeSwap2 = (await key.connection.getTokenAccountBalance(ownerAta2.address)).value.amount;
    console.log("User Token 2 SPL Balance before swap: ", accountBalance2BeforeSwap2);
    let swapTx2 = await program.methods.swap(new BN(0.5 * LAMPORTS_PER_SOL)).accounts({
      vinciSwap: vinciSwapPDA[0],
      userReceiveMint: mint,
      userReceiveTokenAccount: ownerAta1.address,
      poolReceiveTokenAccount: vaultAta1,
      userPayMint: mint2,
      userPayTokenAccount: ownerAta2.address,
      poolPayTokenAccount: vaultAta2,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      user: keypair.publicKey,
    }).signers([keypair]).rpc();
    console.log("Swap from Token 2 to Token 1 performed");
    console.log("TxID: ", swapTx2);
    let accountBalance1AfterSwap2 = (await key.connection.getTokenAccountBalance(ownerAta1.address)).value.amount;
    console.log("User Token 1 SPL Balance after swap: ", accountBalance1AfterSwap2);
    let accountBalance2AfterSwap2 = (await key.connection.getTokenAccountBalance(ownerAta2.address)).value.amount;
    console.log("User Token 2 SPL Balance after swap: ", accountBalance2AfterSwap2);
    console.log("Amount received from swap: ", (accountBalance1AfterSwap2 as any) - (accountBalance1BeforeSwap2 as any));
  });

  it("Close account", async () => {
    let balance = await key.connection.getBalance(keypair.publicKey);
    console.log("Keypair balance before closing account: ", balance);
    const tx = await program.methods.close().accounts({
      vinciSwap: vinciSwapPDA[0],
      destination: keypair.publicKey,
    }).rpc();

    console.log("\nAccount successfully closed - TxID: ", tx);

    console.log("\nKeypair balance after closing account: ", await key.connection.getBalance(keypair.publicKey));
  });
});
