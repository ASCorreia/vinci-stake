import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VinciStake } from "../target/types/vinci_stake";
import { Metaplex, keypairIdentity, bundlrStorage } from "@metaplex-foundation/js";

import { Connection, clusterApiUrl } from "@solana/web3.js"; //used to test the metaplex findByMint function

describe("vinci-stake", () => {
  // Configure the client to use the local cluster.
  const key = anchor.AnchorProvider.env()
  anchor.setProvider(key);

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
    console.log("Staking Pool address: ", vinciWorldStake);
    console.log("Staking Pool Initialized");
    console.log("Your transaction signature", tx);

    const mintAddress = new anchor.web3.PublicKey("EK6fYHzcwfnvBj3Tfv54aWjLpg7LJzKzGbGkd8snMLbb"); //used for testing purposes only
    const metadataAddress = await getMetadata(mintAddress); //used for testing purposes only
    /* Metaplex findByMint and metaDataAccount Tests */
    const connection = new Connection(clusterApiUrl("devnet"));
    const metaplex = new Metaplex(connection);
    const nft = await metaplex.nfts().findByMint({ mintAddress });
    console.log("NFT found: ", nft);
    console.log("Metada Account: ", metadataAddress.toString());
    
    const [vinciWorldStakeEntry, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("VinciWorldStakeEntry"),
        key.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    const tx2 = await program.methods.initializeStakeEntry().accounts({
      user: key.wallet.publicKey,

      stakeEntry: vinciWorldStakeEntry,
      stakePoolAccount: vinciWorldStake,

      originalMint: mintAddress,
      originalMintMetadata: metadataAddress,

      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Stake Entry address: ", vinciWorldStake);
    console.log("Stake Entry created");
    console.log("Your transaction signature", tx2);
    //As metadataAddress matches the address for the metadata in the fetched NFT, this account shall be sent to the staking service
    //Refer to https://github.com/metaplex-foundation/js#findByMint

  });
});
