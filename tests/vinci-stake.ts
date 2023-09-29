import * as anchor from "@project-serum/anchor";
import { Program, AnchorProvider } from "@project-serum/anchor";
import { VinciStake } from "../target/types/vinci_stake";
import { VinciRewards } from "../target/types/vinci_rewards";
import { VinciAccounts } from "../target/types/vinci_accounts";
import { Metaplex, keypairIdentity, bundlrStorage, findNftsByOwnerOperation } from "@metaplex-foundation/js";
import {TOKEN_PROGRAM_ID, MINT_SIZE, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, getAccount, createInitializeMintInstruction, getOrCreateAssociatedTokenAccount} from "@solana/spl-token";

import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { Connection, clusterApiUrl, ConfirmOptions, PublicKey, SystemProgram} from "@solana/web3.js"; //used to test the metaplex findByMint function
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import { keypair } from "../wallet";
import assert from 'assert';

describe("vinci-stake", () => {
  // Configure the client to use the local cluster (environment variable).
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  /* Programs to be used (Vinci Stake Program, Vinci Rewards Program, Vinci Accounts Program) */
  const program = anchor.workspace.VinciStake as Program<VinciStake>;
  const rewardsProgram = anchor.workspace.VinciRewards as Program<VinciRewards>;
  const accountsProgram = anchor.workspace.VinciAccounts as Program<VinciAccounts>;

  /* --------------------------------- Derive the necessary PDAs ---------------------------------- */
  /* Derive a PDA for the vinci accounts program */
  const [vinciWorldPDA, accountBump] = findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("VinciWorldAccount1"),
      provider.wallet.publicKey.toBuffer(),
    ],
    accountsProgram.programId
  );
  console.log("Vinci World account: ", vinciWorldPDA.toBase58());
  console.log("Vinci World account bump: ", accountBump);

  /* Derive a PDA for a Vinci Stake Pool */
  const [vinciWorldStake, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("VinciWorldStakePool_28"),
      provider.wallet.publicKey.toBuffer(),
    ],
    program.programId
  );
  console.log("Vinci World Staking Pool account: ", vinciWorldStake.toBase58());

  /* Derive a PDA for a Vinci Stake Entry */
  const [vinciWorldStakeEntry, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("VinciWorldStakeEntry_28"),
      provider.wallet.publicKey.toBuffer(),
    ],
    program.programId
  )
  console.log("Vinci World Stake Entry account: ", vinciWorldStakeEntry.toBase58());
  /* --------------------------------- Derive the necessary PDAs ---------------------------------- */

  /* Provider public key logged for some reason that I cannot remember :) */
  console.log("\nProvider public key", provider.wallet.publicKey.toString());

  /* Metaplex function to retrieve metadata accounts PDA */
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const getMetadata = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    return (
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };
  /* Metaplex function to retrieve master edition accounts PDA */
  const getMasterEdition = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    return (
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };
  
  /* Create a PDA to receive to be used as seed to receive NFTs */
  const receivePDA = findProgramAddressSync([anchor.utils.bytes.utf8.encode("vault")], program.programId);

  const mintAddress = new anchor.web3.PublicKey("EK6fYHzcwfnvBj3Tfv54aWjLpg7LJzKzGbGkd8snMLbb"); //used for testing purposes only
  const ownerAddress = new anchor.web3.PublicKey("25wServiqrh2T7tXK9HrWb6KkhBegLXmPRtyQtWENnrR");  //AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C

  /* Let's test this :) */
  it("Initialize Stake Pool", async () => {
    const stakePoolTx = await program.methods.initializeStakePool().accounts({
      stakePool: vinciWorldStake,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Stake Pool address: ", vinciWorldStake.toBase58());
    console.log("Stake Pool Initialized");
    console.log("Your transaction signature", stakePoolTx);
  });

  it("Stake, freeze and do stuff", async () => {
    /* -------------------------------------------------------------------------------- */
    const metadataAddress = await getMetadata(mintAddress); //used for testing purposes only
    const masterEditionAcc = await getMasterEdition(mintAddress);

    /* Metaplex findByMint and metaDataAccount Tests */
    const connection = new Connection(clusterApiUrl("devnet"));
    const metaplex = new Metaplex(connection);
    const nft = await metaplex.nfts().findByMint({ mintAddress });
    console.log("\n");
    const allNFTs = ((await metaplex.nfts().findAllByOwner({ owner: ownerAddress})).forEach(nft => 
      console.log("Creator of the NFT: ", nft.creators[0].address.toBase58())));
    //console.log("NFT found: ", nft);
    //console.log("NFT json found: ", nft.json);
    console.log("Metada Account: ", metadataAddress.toString());
    console.log("Nfts owned by the user (Creator Address): ", allNFTs);

    /* --------------------------------------------------------------------------------- */
    const associatedTokenAccountFrom = await getAssociatedTokenAddress(mintAddress, ownerAddress);
    //const associatedTokenAccountFrom = await getOrCreateAssociatedTokenAccount(key.connection, payer, mintAddress, ownerAddress);
    const associatedTokenAccountTo = await getAssociatedTokenAddress(mintAddress, receivePDA[0], true);
    //const associatedTokenAccountTo = await getOrCreateAssociatedTokenAccount(key.connection, payer, mintAddress, key.wallet.publicKey);

    let receiverTokenAccount: any
    try {
      receiverTokenAccount = await getAccount(
        connection,
        associatedTokenAccountTo,
        "confirmed",
        TOKEN_PROGRAM_ID
      )
    } catch (e) {
      // If the account does not exist, add the create account instruction to the transaction
      //Fires a list of instructions
      const mint_tx = new anchor.web3.Transaction().add(        
        //Creates the ATA account that is associated with our mint on our anchor wallet (key)
        createAssociatedTokenAccountInstruction(provider.wallet.publicKey, associatedTokenAccountTo, provider.wallet.publicKey, mintAddress, TOKEN_PROGRAM_ID, ASSOCIATED_PROGRAM_ID),
      );
      console.log("Ata address: ", associatedTokenAccountTo.toString());
      console.log("Ata address: ", associatedTokenAccountFrom.toString());
      const ataTx = await provider.sendAndConfirm(mint_tx);
      console.log("Ata created: ", ataTx);
    } 

    const stakeEntryTx = await program.methods.initializeStakeEntry().accounts({
      user: provider.wallet.publicKey,

      stakeEntry: vinciWorldStakeEntry,
      stakePoolAccount: vinciWorldStake,

      originalMint: mintAddress,
      originalMintMetadata: metadataAddress,

      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Stake Entry address: ", vinciWorldStake.toBase58());
    console.log("Stake Entry created");
    console.log("Your transaction signature", stakeEntryTx);
  });
  //As metadataAddress matches the address for the metadata in the fetched NFT, this account shall be sent to the staking service
  //Refer to https://github.com/metaplex-foundation/js#findByMint
  
  /*it("Stake Custodial", async () => {
    const associatedTokenAccountFrom = await getAssociatedTokenAddress(mintAddress, ownerAddress);
    const associatedTokenAccountTo = await getAssociatedTokenAddress(mintAddress, key.wallet.publicKey);

    const stakeNFTtx = await program.methods.stake().accounts({
      stakeEntry: vinciWorldStakeEntry,
      stakePool: vinciWorldStake,

      originalMint: mintAddress,
      fromMintTokenAccount: associatedTokenAccountFrom,
      toMintTokenAccount: associatedTokenAccountTo,

      user: key.wallet.publicKey,

      tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();
    console.log("\n\nNFT Stacked - Transaction ID", stakeNFTtx);
  });*/

  /*it("Claim Custodial", async () => {
    const metadataAddress = await getMetadata(mintAddress); //used for testing purposes only
    const masterEditionAcc = await getMasterEdition(mintAddress);

    const associatedTokenAccountFrom = await getAssociatedTokenAddress(mintAddress, ownerAddress);
    const associatedTokenAccountTo = await getAssociatedTokenAddress(mintAddress, key.wallet.publicKey);

    const claimStakeTx = await program.methods.claimStake().accounts({
      user: key.wallet.publicKey,

      stakeEntry: vinciWorldStakeEntry,
      stakePool: vinciWorldStake,

      fromMintTokenAccount: associatedTokenAccountTo,
      toMintTokenAccount: associatedTokenAccountFrom,

      originalMint: mintAddress,

      masterEdition: masterEditionAcc,

      tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();
    console.log("\n\nMint Claimed - Transaction ID: ", claimStakeTx);
  });*/

  it("Stake Non Custodial", async () => {
    const associatedTokenAccountNonCust = await getAssociatedTokenAddress(mintAddress, vinciWorldStakeEntry, true, TOKEN_PROGRAM_ID, ASSOCIATED_PROGRAM_ID); //vinciWorldNonCustodial
    const metadataAddress = await getMetadata(mintAddress); //used for testing purposes only
    const masterEditionAcc = await getMasterEdition(mintAddress);

    const associatedTokenAccountFrom = await getAssociatedTokenAddress(mintAddress, ownerAddress);
    const associatedTokenAccountTo = await getAssociatedTokenAddress(mintAddress, provider.wallet.publicKey);

    let receiverTokenAccount2: any
    try {
      receiverTokenAccount2 = await getAccount(
        provider.connection,
        associatedTokenAccountNonCust,
        "confirmed",
        TOKEN_PROGRAM_ID
      )
    } catch (e) {
      // If the account does not exist, add the create account instruction to the transaction
      //Fires a list of instructions
      const mint_tx = new anchor.web3.Transaction().add(        
        //Creates the ATA account that is associated with our mint on our anchor wallet (key)
        createAssociatedTokenAccountInstruction(provider.wallet.publicKey, associatedTokenAccountNonCust, vinciWorldStakeEntry, mintAddress, TOKEN_PROGRAM_ID, ASSOCIATED_PROGRAM_ID),
      );
      console.log("\n\nAta address: ", associatedTokenAccountNonCust.toString());
      const ataTx = await provider.sendAndConfirm(mint_tx);
      console.log("Ata created: ", ataTx);
    }
    
    const tokenAccountInfo = await provider.connection.getParsedAccountInfo(associatedTokenAccountNonCust);
    const tokenAccountData = tokenAccountInfo.value.data;

    console.log('\n\nThe token account info is:', tokenAccountData);

    const stakeNonCust = await program.methods.stakeNonCustodial().accounts({
      stakeEntry: vinciWorldStakeEntry,
      stakePool: vinciWorldStake,
      originalMint: mintAddress,
      fromMintTokenAccount: associatedTokenAccountFrom,
      toMintTokenAccount: associatedTokenAccountNonCust, //vinciWorldNonCustodial,
      user: keypair.publicKey, //key.wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      masterEdition: masterEditionAcc,
      test: provider.wallet.publicKey,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    }).signers([keypair]).rpc(
      {
        skipPreflight: true,
      }
    );
    console.log('NFT sucessfully frozen - Transaction ID: ', stakeNonCust);
  });

  it("Update Stake Entry", async () => {
    const updateEntry = await program.methods.updateStake().accounts({
      stakePool: provider.publicKey,
    }).remainingAccounts([
      {pubkey: vinciWorldStakeEntry, isSigner: false, isWritable: true}   
    ]).rpc();
    console.log('\n\nStake Entry Successfully updated - Transaction ID: ', updateEntry);
  })

  it("Claim Staking Rewards", async () => {
    const claimRewards = await program.methods.claimRewards().accounts({
      stakeEntry: vinciWorldStakeEntry,
      vinciAccount: vinciWorldPDA,
      owner: provider.wallet.publicKey,
      accountsProgram: accountsProgram.programId,
      rewardsProgram: rewardsProgram.programId,
    }).rpc();
    console.log('\n\nRewards sucesfully claimed - Transaction ID: ', claimRewards);

    let fetchAccount = await accountsProgram.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey 
    console.log("Total Ammount Of Tokens", fetchAccount.totalAmount.toString());
  })

  it("Claim Non Custodial Stake", async () => {
    const masterEditionAcc = await getMasterEdition(mintAddress);

    const associatedTokenAccountFrom = await getAssociatedTokenAddress(mintAddress, ownerAddress);

    const associatedTokenAccountNonCust = await getAssociatedTokenAddress(mintAddress, vinciWorldStakeEntry, true, TOKEN_PROGRAM_ID, ASSOCIATED_PROGRAM_ID); //vinciWorldNonCustodial

    const claimNonCust = await program.methods.claimNonCustodial().accounts({
      stakeEntry: vinciWorldStakeEntry,
      stakePool: vinciWorldStake,
      originalMint: mintAddress,
      fromMintTokenAccount: associatedTokenAccountFrom, //associatedTokenAccountFrom
      toMintTokenAccount: associatedTokenAccountNonCust, //vinciWorldNonCustodial,
      user: provider.wallet.publicKey, //key.wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      masterEdition: masterEditionAcc,
      test: provider.wallet.publicKey,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    }).rpc(
      {
        skipPreflight: true,
      }
    );
    console.log('\n\nNFT sucessfully unfrozen - Transaction ID: ', claimNonCust);
  });

  it("Close Stake Entry", async () => {
    const tx = await program.methods.closeStakeEntry().accounts({
      stakeEntry: vinciWorldStakeEntry,
      destination: provider.publicKey,
    }).rpc();
    console.log("\n\nStake Entry Closed! TxID: ", tx);
  })

  it("Close Stake Pool", async () => {
    const tx = await program.methods.closeStakePool().accounts({
      stakePool: vinciWorldStake,
      destination: provider.publicKey,
    }).rpc();
    console.log("\n\nStake Pool Closed! TxID: ", tx);
  })
});

/* Things to consider */
/*const accounts = await connection.getProgramAccounts(program.programId);
  console.log("\n\nProgram Owned Accounts:\n", accounts);

  accounts.forEach((account, i) => {
  console.log(`-- Program owned account ${i + 1}: ${account.pubkey.toString()} --`);
});*/
