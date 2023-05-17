import * as anchor from "@project-serum/anchor";
import { Program, AnchorProvider } from "@project-serum/anchor";
import { VinciStake } from "../target/types/vinci_stake";
import { VinciRewards } from "../target/types/vinci_rewards";
import { VinciAccounts } from "../target/types/vinci_accounts";
import { Metaplex, keypairIdentity, bundlrStorage, findNftsByOwnerOperation } from "@metaplex-foundation/js";
import {TOKEN_PROGRAM_ID, MINT_SIZE, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, getAccount, createInitializeMintInstruction} from "@solana/spl-token";

import { Connection, clusterApiUrl, ConfirmOptions, PublicKey, SystemProgram} from "@solana/web3.js"; //used to test the metaplex findByMint function
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import { keypair } from "../wallet";
import assert from 'assert';

describe("vinci-stake", () => {
  // Configure the client to use the local cluster (environment variable).
  const key = anchor.AnchorProvider.env()
  anchor.setProvider(key);

  /* Programs to be used (Vinci Stake Program, Vinci Rewards Program, Vinci Accounts Program) */
  const program = anchor.workspace.VinciStake as Program<VinciStake>;
  const rewardsProgram = anchor.workspace.VinciRewards as Program<VinciRewards>;
  const accountsProgram = anchor.workspace.VinciAccounts as Program<VinciAccounts>;

  /* --------------------------------- Derive the necessary PDAs ---------------------------------- */
  /* Derive a PDA for the vinci accounts program */
  const [vinciWorldPDA, _bump] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("VinciWorldAccount1"),
      key.wallet.publicKey.toBuffer(),
    ],
    accountsProgram.programId
  );
  console.log("Vinci World account: ", vinciWorldPDA.toBase58());

  /* Derive a PDA for a Vinci Stake Pool */
  const [vinciWorldStake, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("VinciWorldStakePool_28"),
      key.wallet.publicKey.toBuffer(),
    ],
    program.programId
  );
  console.log("Vinci World Staking Pool account: ", vinciWorldStake.toBase58());

  /* Derive a PDA for a Vinci Stake Entry */
  const [vinciWorldStakeEntry, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("VinciWorldStakeEntry_28"),
      key.wallet.publicKey.toBuffer(),
    ],
    program.programId
  )
  console.log("Vinci Worls Stake Entry account: ", vinciWorldStakeEntry.toBase58());
  /* --------------------------------- Derive the necessary PDAs ---------------------------------- */

  /* Provider public key logged for some reason that I cannot remember :) */
  console.log("\nProvider public key", key.wallet.publicKey.toString());

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

  /* Let's test this :) */
  it("Is initialized!", async () => {
    /*const stakePoolTx = await program.methods.initializeStakePool().accounts({
      stakePool: vinciWorldStake,
      user: key.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Staking Pool address: ", vinciWorldStake);
    console.log("Staking Pool Initialized");
    console.log("Your transaction signature", stakePoolTx);*/

    /* -------------------------------------------------------------------------------- */

    const mintAddress = new anchor.web3.PublicKey("EK6fYHzcwfnvBj3Tfv54aWjLpg7LJzKzGbGkd8snMLbb"); //used for testing purposes only
    const metadataAddress = await getMetadata(mintAddress); //used for testing purposes only
    const masterEditionAcc = await getMasterEdition(mintAddress);

    const ownerAddress = new anchor.web3.PublicKey("25wServiqrh2T7tXK9HrWb6KkhBegLXmPRtyQtWENnrR");  //AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C

    /* Metaplex findByMint and metaDataAccount Tests */
    const connection = new Connection(clusterApiUrl("devnet"));
    const metaplex = new Metaplex(connection);
    const nft = await metaplex.nfts().findByMint({ mintAddress });
    const allNFTs = await metaplex.nfts().findAllByOwner({ owner: ownerAddress});
    console.log("NFT found: ", nft);
    console.log("NFT json found: ", nft.json);
    console.log("Metada Account: ", metadataAddress.toString());
    console.log("Nfts owned by the user: ", allNFTs);

    /* --------------------------------------------------------------------------------- */
    const associatedTokenAccountFrom = await getAssociatedTokenAddress(mintAddress, ownerAddress);
    const associatedTokenAccountTo = await getAssociatedTokenAddress(mintAddress, key.wallet.publicKey);

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
        createAssociatedTokenAccountInstruction(key.wallet.publicKey, associatedTokenAccountFrom, ownerAddress, mintAddress, TOKEN_PROGRAM_ID, ASSOCIATED_PROGRAM_ID),
      );
      console.log("Ata address: ", associatedTokenAccountTo.toString());
      console.log("Ata address: ", associatedTokenAccountFrom.toString());
      const ataTx = await key.sendAndConfirm(mint_tx);
      console.log("Ata created: ", ataTx);
    } 

    /*const stakeEntryTx = await program.methods.initializeStakeEntry().accounts({
      user: key.wallet.publicKey,

      stakeEntry: vinciWorldStakeEntry,
      stakePoolAccount: vinciWorldStake,

      originalMint: mintAddress,
      originalMintMetadata: metadataAddress,

      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Stake Entry address: ", vinciWorldStake);
    console.log("Stake Entry created");
    console.log("Your transaction signature", stakeEntryTx);*/
    //As metadataAddress matches the address for the metadata in the fetched NFT, this account shall be sent to the staking service
    //Refer to https://github.com/metaplex-foundation/js#findByMint

    /*const stakeNFTtx = await program.methods.stake().accounts({
      stakeEntry: vinciWorldStakeEntry,
      stakePool: vinciWorldStake,

      originalMint: mintAddress,
      fromMintTokenAccount: associatedTokenAccountFrom,
      toMintTokenAccount: associatedTokenAccountTo,

      user: key.wallet.publicKey,

      tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();
    console.log("NFT Stacked - Transaction ID", stakeNFTtx);*/

    /*const claimStakeTx = await program.methods.claimStake().accounts({
      user: key.wallet.publicKey,

      stakeEntry: vinciWorldStakeEntry,
      stakePool: vinciWorldStake,

      fromMintTokenAccount: associatedTokenAccountTo,
      toMintTokenAccount: associatedTokenAccountFrom,

      originalMint: mintAddress,

      masterEdition: masterEditionAcc,

      tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();
    console.log("Mint Claimed - Transaction ID: ", claimStakeTx);*/

    const associatedTokenAccountNonCust = await getAssociatedTokenAddress(mintAddress, vinciWorldStakeEntry, true, TOKEN_PROGRAM_ID, ASSOCIATED_PROGRAM_ID); //vinciWorldNonCustodial

    let receiverTokenAccount2: any
    try {
      receiverTokenAccount2 = await getAccount(
        connection,
        associatedTokenAccountNonCust,
        "confirmed",
        TOKEN_PROGRAM_ID
      )
    } catch (e) {
      // If the account does not exist, add the create account instruction to the transaction
      //Fires a list of instructions
      const mint_tx = new anchor.web3.Transaction().add(        
        //Creates the ATA account that is associated with our mint on our anchor wallet (key)
        createAssociatedTokenAccountInstruction(key.wallet.publicKey, associatedTokenAccountNonCust, vinciWorldStakeEntry, mintAddress, TOKEN_PROGRAM_ID, ASSOCIATED_PROGRAM_ID),
      );
      console.log("Ata address: ", associatedTokenAccountNonCust.toString());
      const ataTx = await key.sendAndConfirm(mint_tx);
      console.log("Ata created: ", ataTx);
    }
    
    const tokenAccountInfo = await connection.getParsedAccountInfo(associatedTokenAccountNonCust);
    const tokenAccountData = tokenAccountInfo.value.data;

    console.log('The token account info is:', tokenAccountData);

    /*const stakeNonCust = await program.methods.stakeNonCustodial().accounts({
      stakeEntry: vinciWorldStakeEntry,
      stakePool: vinciWorldStake,
      originalMint: mintAddress,
      fromMintTokenAccount: associatedTokenAccountFrom,
      toMintTokenAccount: associatedTokenAccountNonCust, //vinciWorldNonCustodial,
      user: keypair.publicKey, //key.wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      masterEdition: masterEditionAcc,
      test: key.wallet.publicKey,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    }).signers([keypair]).rpc(
      {
        skipPreflight: true,
      }
    );
    console.log('NFT sucessfully frozen - Transaction ID: ', stakeNonCust);*/

    const updateEntry = await program.methods.updateStake().accounts({
      stakePool: key.publicKey,
    }).remainingAccounts([
      {pubkey: vinciWorldStakeEntry, isSigner: false, isWritable: true}   
    ]).rpc();
    console.log('Stake Entry Successfully updated - Transaction ID: ', updateEntry);

    const claimRewards = await program.methods.claimRewards().accounts({
      stakeEntry: vinciWorldStakeEntry,
      vinciAccount: vinciWorldPDA,
      owner: key.wallet.publicKey,
      accountsProgram: accountsProgram.programId,
      rewardsProgram: rewardsProgram.programId,
    }).rpc();
    console.log('Rewards sucesfully claimed - Transaction ID: ', claimRewards);

    let fetchAccount = await accountsProgram.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey 
    console.log("Total Ammount Of Tokens", fetchAccount.totalAmount.toString());

    /*const claimNonCust = await program.methods.claimNonCustodial().accounts({
      stakeEntry: vinciWorldStakeEntry,
      stakePool: vinciWorldStake,
      originalMint: mintAddress,
      fromMintTokenAccount: associatedTokenAccountFrom, //associatedTokenAccountFrom
      toMintTokenAccount: associatedTokenAccountNonCust, //vinciWorldNonCustodial,
      user: key.wallet.publicKey, //key.wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      masterEdition: masterEditionAcc,
      test: key.wallet.publicKey,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    }).rpc(
      {
        skipPreflight: true,
      }
    );
    console.log('NFT sucessfully unfrozen - Transaction ID: ', claimNonCust);*/
  });
  /*it ("Quest simulation", async() => {
    const tx = await accountsProgram.methods.startStuffOff().accounts({
          baseAccount: vinciWorldPDA,
          user: key.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        }).rpc(); //.signers[account] before rpc()

    console.log("Vinci World PDA account created with Transaction", tx);

    let fetchAccount = await accountsProgram.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey
    
    console.log("Total Ammount Of Tokens", fetchAccount.totalAmount.toString());
    console.log("Owner of the account: ", fetchAccount.owner.toString());
    console.log("Address of the provider: ", key.wallet.publicKey.toString());
    assert.equal(fetchAccount.totalAmount.toString(), "0");

    const addValue = await accountsProgram.methods.addAmmount(new anchor.BN(15)).accounts({
        baseAccount: vinciWorldPDA,
    }).rpc();
    //can we pass more than one ammount and accounts?

    let fetchAccount2 = await accountsProgram.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey
    console.log("Match won - 15 Tokens awarded");
    console.log("Total Ammount Of Tokens", fetchAccount2.totalAmount.toString());
    assert.equal(fetchAccount2.totalAmount.toString(), "15");

    const tournamentPay = await accountsProgram.methods.payTournament(new anchor.BN(30)).accounts({
        user: key.wallet.publicKey,
    }).remainingAccounts([{pubkey: vinciWorldPDA, isSigner: false, isWritable: true}]).rpc();
    console.log("Tournament transaction details: ", tournamentPay);
    console.log("Signer wallet address (Provider): ", key.wallet.publicKey);

    let fetchAccount3 = await accountsProgram.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey
    console.log("Tournament won - 30 Tokens awarded");
    console.log("Total Ammount Of Tokens", fetchAccount3.totalAmount.toString());
    assert.equal(fetchAccount3.totalAmount.toString(), "45");
  });*/
});

/* Things to consider */
/*const accounts = await connection.getProgramAccounts(program.programId);
  console.log("\n\nProgram Owned Accounts:\n", accounts);

  accounts.forEach((account, i) => {
  console.log(`-- Program owned account ${i + 1}: ${account.pubkey.toString()} --`);
});*/
