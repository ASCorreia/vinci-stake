import * as anchor from "@project-serum/anchor";
import * as bs58 from "bs58";
import { Program, AnchorProvider } from "@project-serum/anchor";
import { VinciStake } from "../target/types/vinci_stake";
import { Metaplex, keypairIdentity, bundlrStorage, findNftsByOwnerOperation } from "@metaplex-foundation/js";
import {TOKEN_PROGRAM_ID, MINT_SIZE, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, getAccount, createInitializeMintInstruction} from "@solana/spl-token";

import { Connection, clusterApiUrl, ConfirmOptions} from "@solana/web3.js"; //used to test the metaplex findByMint function
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import base58 from "bs58";
import { Wallet } from "@project-serum/anchor";

describe("vinci-stake", () => {
  // Configure the client to use the local cluster.
  const key = anchor.AnchorProvider.env()
  anchor.setProvider(key);

  // Convert private key string to a Uint8Array
  const b = bs58.decode('privatekey');
  const j = new Uint8Array(b.buffer, b.byteOffset, b.byteLength / Uint8Array.BYTES_PER_ELEMENT);
  const keypair = anchor.web3.Keypair.fromSecretKey(j);
  console.log("Public Key: ", keypair.publicKey.toString(), "\n Private Key: ", keypair.secretKey.toString());

  /*const network = clusterApiUrl("devnet");
  const opts: ConfirmOptions = {
    preflightCommitment: 'processed' //processed
  }

  const connection2 = new Connection(network, 'processed');

  const wallet = new Wallet(keypair);    
  const provider = new AnchorProvider(connection2, wallet, opts);
  anchor.setProvider(provider);*/

  const program = anchor.workspace.VinciStake as Program<VinciStake>;

  console.log("\nProvider public key", key.wallet.publicKey.toString());

  /* Metaplex function to retrieve metadata accounts PDA */
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const getMetadata = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    return (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };
  const getMasterEdition = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    return (
      await anchor.web3.PublicKey.findProgramAddress(
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

  it("Is initialized!", async () => {
    const [vinciWorldStake, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("VinciWorldStakePool_28"),
        key.wallet.publicKey.toBuffer(),
      ],
      program.programId
      );
    // Add your test here.
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
        associatedTokenAccountFrom,
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


    const [vinciWorldStakeEntry, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("VinciWorldStakeEntry_28"),
        key.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
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

    const stakeNonCust = await program.methods.stakeNonCustodial().accounts({
      stakeEntry: vinciWorldStakeEntry,
      stakePool: vinciWorldStake,
      originalMint: mintAddress,
      fromMintTokenAccount: associatedTokenAccountFrom, //associatedTokenAccountFrom
      toMintTokenAccount: program.programId,
      user: keypair.publicKey, //key.wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      masterEdition: masterEditionAcc, 
    }).signers([keypair]).rpc();
    console.log('NFT sucessfully frozen - Transaction ID: ', stakeNonCust);

   /*const accounts = await connection.getProgramAccounts(program.programId);
    console.log("\n\nProgram Owned Accounts:\n", accounts);

    accounts.forEach((account, i) => {
      console.log(`-- Program owned account ${i + 1}: ${account.pubkey.toString()} --`);
    });*/
  });
});
