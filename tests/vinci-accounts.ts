import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { VinciAccounts } from "../target/types/vinci_accounts";
import assert from "assert";
import { isConstructorDeclaration } from "typescript";
import { VinciQuiz } from "../target/types/vinci_quiz";
import { MINT_SIZE, TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, createInitializeMintInstruction, getAssociatedTokenAddress } from "@solana/spl-token";

describe("vinci-accounts", () => {
  // Configure the client to use the local cluster.
  const key = anchor.AnchorProvider.env();
  anchor.setProvider(key);

  const program = anchor.workspace.VinciAccounts as Program<VinciAccounts>;
  const programQuiz = anchor.workspace.VinciQuiz as Program<VinciQuiz>;

  const keypair = anchor.web3.Keypair.generate();

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

  /* Derive a PDA for the vinci accounts program */
  const [vinciWorldPDA, bump] = findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("VinciWorldAccount1"),
      key.wallet.publicKey.toBuffer(),
    ],
    program.programId
  );
  console.log("\n\nVinci World account: ", vinciWorldPDA.toBase58());
  console.log("Vinci World account bump: ", bump);

  /* Derive a PDA for the vinci quiz program */
  let vinciQuizPDA = PublicKey.findProgramAddressSync([anchor.utils.bytes.utf8.encode("VinciQuiz")], programQuiz.programId);
  console.log("\n\nVinci Quiz account: ", vinciQuizPDA[0].toBase58());
  console.log("Vinci Quiz account bump: ", vinciQuizPDA[1]);

  /*it("Account Initialization", async() => {
    const tx = await program.methods.startStuffOff().accounts({
          baseAccount: vinciWorldPDA,
          user: key.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        }).rpc({skipPreflight: true}); //.signers[account] before rpc()

    console.log("\n\nVinci World PDA account created with Transaction", tx);
  });*/

  /*it ("Quest simulation", async() => {
    let fetchAccount = await program.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey
    
    console.log("\n\nTotal Ammount Of Tokens", fetchAccount.totalAmount.toString());
    console.log("Owner of the account: ", fetchAccount.owner.toString());
    console.log("Address of the provider: ", key.wallet.publicKey.toString());
    //assert.equal(fetchAccount.totalAmount.toString(), "0");

    const addValue = await program.methods.addAmmount(new anchor.BN(15)).accounts({
        baseAccount: vinciWorldPDA,
        owner: key.publicKey,
    }).rpc();
    //can we pass more than one ammount and accounts?

    let fetchAccount2 = await program.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey
    console.log("Match won - 15 Tokens awarded");
    console.log("Total Ammount Of Tokens", fetchAccount2.totalAmount.toString());
    //assert.equal(fetchAccount2.totalAmount.toString(), "15");

    const tournamentPay = await program.methods.payTournament(new anchor.BN(30)).accounts({
        user: key.wallet.publicKey,
    }).remainingAccounts([{pubkey: vinciWorldPDA, isSigner: false, isWritable: true}]).rpc();
    console.log("Tournament transaction details: ", tournamentPay);
    console.log("Signer wallet address (Provider): ", key.wallet.publicKey.toString());

    let fetchAccount3 = await program.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey
    console.log("Tournament won - 30 Tokens awarded");
    console.log("Total Ammount Of Tokens", fetchAccount3.totalAmount.toString());
    //assert.equal(fetchAccount3.totalAmount.toString(), "45");
  });*/

  /*it("Distribute Season Rewards", async() => {
    const tx = await program.methods.seasonRewards().accounts({
      vinciQuiz: vinciQuizPDA[0],
      quizProgram: programQuiz.programId,
    }).remainingAccounts([{
      pubkey: vinciWorldPDA,
      isSigner: false,
      isWritable: true,
    }]).rpc();

    console.log("\n\nSeason Rewards distributed - TxID: ", tx);
  })*/

  it("Mint one NFT", async () => {
    //Starts the Mint Operation
    console.log("Starting the NFT Mint Operation");

    const NFTmintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);

    //Get the ATA for a token on a public key (but might not exist yet)
    let receiver = key.publicKey;
    let associatedTokenAccount = await getAssociatedTokenAddress(NFTmintKey.publicKey, receiver);

    //Create a PDA to be mint authority
    let pdaAuthority = findProgramAddressSync([anchor.utils.bytes.utf8.encode("authority")], program.programId);

    //Creates a transaction with a list of instructions
    const mint_nft_tx = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.createAccount({
            fromPubkey: receiver,
            newAccountPubkey: NFTmintKey.publicKey,
            space: MINT_SIZE,
            programId: TOKEN_PROGRAM_ID,
            lamports,
        }),
        
        createInitializeMintInstruction(NFTmintKey.publicKey, 0, pdaAuthority[0], pdaAuthority[0]),
        
        createAssociatedTokenAccountInstruction(receiver, associatedTokenAccount, receiver, NFTmintKey.publicKey)
    );

    //Sends and confirms the transaction
    console.log("Sending transaction");
    const res = await key.sendAndConfirm(mint_nft_tx, [NFTmintKey]);

    const metadataAddress = await getMetadata(NFTmintKey.publicKey);
    const masterEdition = await getMasterEdition(NFTmintKey.publicKey);

    //Executes our smart contract to mint our token into our specified ATA
    //Don't forget to add your creator, uri and NFT title
    console.log("Executing Smart Contract");
    const tx = await program.methods.mintNft(
      "https://arweave.net/Pe4erqz3MZoywHqntUGZoKIoH0k9QUykVDFVMjpJ08s",
      "Vinci World EA").accounts(
        {
          mintAuthority: pdaAuthority[0],
          mint: NFTmintKey.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          metadata: metadataAddress,
          tokenAccount: associatedTokenAccount,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          payer: receiver,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          masterEdition: masterEdition,
        },
    ).rpc({skipPreflight: true});
    console.log("Your transaction signature", tx);       
    console.log("NFT Mint Operation Finished!");
    
  })

  /*it("Close Vinci Account and refund rent lamports", async() => {
    const tx = await program.methods.closeAccount().accounts({
        vinciAccount: vinciWorldPDA,
        destination: key.wallet.publicKey,
    }).rpc();
    console.log("\n\nVinci Account succesfully closed - TxID: ", tx);
  })*/
});
