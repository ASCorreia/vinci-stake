import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionMessage, TransactionSignature, VersionedTransaction } from '@solana/web3.js';
import * as spl from "@solana/spl-token";
import bs58 from 'bs58';
import { FC, useCallback, useEffect, useState } from 'react';
import { notify } from "../utils/notifications";

import { Program, AnchorProvider, web3, utils, BN } from '@project-serum/anchor';
import idl from "./solanapdas.json";
import idlSwap from "./solanapdas2.json";
import idlQuiz from "./solanapdas3.json";
import idlAccounts from "./solanapdas4.json";
import { program } from '@project-serum/anchor/dist/cjs/native/system';
import { ASSOCIATED_PROGRAM_ID } from '@project-serum/anchor/dist/cjs/utils/token';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import { generateKeyPair } from 'crypto';

const idl_string = JSON.stringify(idl);
const idl_object = JSON.parse(idl_string);
const programID = new PublicKey(idl.metadata.address);

const idl_string_swap = JSON.stringify(idlSwap);
const idl_object_swap = JSON.parse(idl_string_swap);
const swapProgramID = new PublicKey(idlSwap.metadata.address);

const idl_string_quiz = JSON.stringify(idlQuiz);
const idl_object_quiz = JSON.parse(idl_string_quiz);
const quizProgramID = new PublicKey(idlQuiz.metadata.address);

const idl_string_accounts = JSON.stringify(idlAccounts);
const idl_object_accounts = JSON.parse(idl_string_accounts);
const accountsProgramID = new PublicKey(idlAccounts.metadata.address);

const player2 = new Keypair;
const vinciAccountPDA2 = findProgramAddressSync([utils.bytes.utf8.encode("VinciWorldAccount1"), player2.publicKey.toBuffer()], accountsProgramID);

export const Quiz: FC = () => {
    const { connection } = useConnection();
    const ourWallet = useWallet();

    const [scores, setScores] = useState([]);

    const [players, setPlayers] = useState([]);
    
    const [poolPDA2, setPoolPDA2] = useState([]);

    const [poolBalance1, setPoolBalance1] = useState(0);
    const [poolBalance2, setPoolBalance2] = useState(0);

    const playerList: web3.AccountMeta[] = [];

    const [inputValueUpdate, setInputValueUpdate] = useState("");
    const onInputChangeUpdate = event => {
        const { value } = event.target;
        setInputValueUpdate(value);
      }

    const getProvider = () => {
        const provider = new AnchorProvider(connection, ourWallet, AnchorProvider.defaultOptions())
        return provider;
    }

    const anchProvider = getProvider();
    const program = new Program(idl_object, programID, anchProvider);
    const programSwap = new Program(idl_object_swap, swapProgramID, anchProvider);
    const programQuiz = new Program(idl_object_quiz, quizProgramID, anchProvider);
    const programAccounts = new Program(idl_object_accounts, accountsProgramID, anchProvider);

    //Derive a Vinci Quiz PDA
    let vinciQuizPDA = PublicKey.findProgramAddressSync([utils.bytes.utf8.encode("VinciQuiz")], programQuiz.programId);

    //Derive a Vinci Swap PDA
    const [vinciSwap, _] = PublicKey.findProgramAddressSync([
        utils.bytes.utf8.encode("VinciSwap"),
    ], programSwap.programId);

    const mint1 = new PublicKey("8LbiacZvDREPUa5a7Ljth16G9p1BoKXccqs5cMcjuhfu");
    const mint2 = new PublicKey("E7sRawws3T77FLf5P7u5W1gBA9ex2H6TfFCxKQJA2TYA");

    const vinciAccountPDA = findProgramAddressSync([utils.bytes.utf8.encode("VinciWorldAccount1"), anchProvider.publicKey.toBuffer()], accountsProgramID);

    const createQuiz = async() => {
        try {
            const tx = await programQuiz.methods.initialize().accounts({
                vinciQuiz: vinciQuizPDA[0],
                user: anchProvider.publicKey,
                systemProgram: SystemProgram.programId,
            }).rpc();

            console.log("Vinci Quiz account successfully created - TxID: ", tx);
        }
        catch (error) {
            console.log("Something went wrong while trying to create Vinci Quiz Account - ", error);
        }
    }

    const closeQuiz = async() => {
        try {
            const tx = await programQuiz.methods.closeSeason().accounts({
                vinciQuiz: vinciQuizPDA[0],
                destination: anchProvider.wallet.publicKey,
            }).rpc();

            console.log("Vinci Quiz account successfully closed - TxID: ", tx);
        }
        catch(error) {
            console.log("Error while closing the liquidity pool: ", error);
        }
    }

    const addPlayer = async() => {
        try {
            const tx = await programQuiz.methods.addPlayer().accounts({
                vinciQuiz: vinciQuizPDA[0],
                user: anchProvider.publicKey,
                systemProgram: SystemProgram.programId,
              }).rpc();

              console.log("Player Sucessfully added to Vinci Quiz - TxID: ", tx);
        }
        catch(error) {
            console.log("Error while addin gplayer to Vinci Quiz: ", error)
        }
    }

    const addPlayer2 = async() => {
        try{
            const txhash = await anchProvider.connection.requestAirdrop(player2.publicKey, 1 * LAMPORTS_PER_SOL);

            let latestBlockHash = await anchProvider.connection.getLatestBlockhash()
            await anchProvider.connection.confirmTransaction({
                blockhash: latestBlockHash.blockhash,
                lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                signature: txhash,
            });

            console.log(`Airdrop to Player 2 Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
        }
        catch(error) {
            console.log("Error while trying to request airdrop: ", error);
            console.log("Trying to transfer from anchor provider");
            try {
                const transaction = new Transaction().add(
                    SystemProgram.transfer({
                        fromPubkey: anchProvider.wallet.publicKey,
                        toPubkey:  player2.publicKey,
                        lamports: (LAMPORTS_PER_SOL/100) * 3,
                    })
                );
                transaction.recentBlockhash = (await connection.getLatestBlockhash('finalized')).blockhash;
                transaction.feePayer = anchProvider.wallet.publicKey;
            
                // Sign transaction, broadcast, and confirm
                const signature = await anchProvider.sendAndConfirm(transaction);
                console.log(`Transfer Success! Check out your TX here: 
                https://explorer.solana.com/tx/${signature}?cluster=devnet`);
            } catch(e) {
                console.error(`Oops, something went wrong: ${e}`)
            }
        }

        try {
            const tx = await programQuiz.methods.addPlayer().accounts({
                vinciQuiz: vinciQuizPDA[0],
                user: player2.publicKey,
                systemProgram: SystemProgram.programId,
              }).signers([player2]).rpc();

              console.log("Player 2 Sucessfully added to Vinci Quiz - TxID: ", tx);
        }
        catch(error) {
            console.log("Error while adding player to Vinci Quiz: ", error)
        }

        try {
            const tx = await programAccounts.methods.startStuffOff().accounts({
                user: player2.publicKey,
                baseAccount: vinciAccountPDA2[0],
                systemProgram: SystemProgram.programId,
            }).signers([player2]).rpc();

            console.log("Vinci Account for Player 2 created successfuly - TxID: ", tx);
        }
        catch(error) {
            console.log("Error while creating account: ", error);
        }
    }

    const updateScore = async(pubkey: PublicKey, win: boolean) => {
        try {
            const tx = await programQuiz.methods.updateScore(win).accounts({
                vinciQuiz: vinciQuizPDA[0],
                user: pubkey,
            }).rpc();
        }
        catch(error) {
            console.log("Error while updating player score: ", error);
        }
    }

    const getScores = async () => {
        try {
            await Promise.all((await connection.getProgramAccounts(quizProgramID)).map(async score => ({
                ...(await programQuiz.account.quizSeason.fetch(score.pubkey)),
                //pubKey: score.pubkey
            }))).then(scores => {
                console.log(scores);
                setScores(scores);
            })
        }
        catch (error) {
            console.log("Error while getting the Player Scores ", error);
        }
    }

    const distributeRewards = async () => {
        const vinciAccountPDA = findProgramAddressSync([utils.bytes.utf8.encode("VinciWorldAccount1"), anchProvider.publicKey.toBuffer()], accountsProgramID);

        try {
            const tx = await programAccounts.methods.seasonRewards().accounts({
                vinciQuiz: vinciQuizPDA[0],
                owner: anchProvider.publicKey,
                quizProgram: quizProgramID,
            }).remainingAccounts(playerList).rpc({skipPreflight: true});

            console.log("Quiz rewards have been distributed - TxID: ", tx);
        }
        catch (error) {
            console.log("Error while distibuting rewards: ", error)
        }
    }

    const upgrade = async (pubkey: PublicKey) => {
        try {
            const tx = programQuiz.methods.upgrade().accounts({
                vinciQuiz: vinciQuizPDA[0],
                user: pubkey,
                authority: anchProvider.publicKey,
            }).rpc();
        }
        catch (error) {
            console.log("Error while upgrading account: ", error);
        }
    }

    const createAccount = async () => {
        const vinciAccountPDA = findProgramAddressSync([utils.bytes.utf8.encode("VinciWorldAccount1"), anchProvider.publicKey.toBuffer()], accountsProgramID);
        try {
            const tx = await programAccounts.methods.startStuffOff().accounts({
                user: anchProvider.publicKey,
                baseAccount: vinciAccountPDA[0],
                systemProgram: SystemProgram.programId,
            }).rpc();

            console.log("Vinci Account created successfuly - TxID: ", tx);
        }
        catch(error) {
            console.log("Error while creating account: ", error);
        }
    }

    const closeAccount = async (account: PublicKey) => {
        try {
            const tx = await programAccounts.methods.closeAccount().accounts({
                vinciAccount: account,
                destination: anchProvider.publicKey,
            }).rpc();

            console.log("Vinci Account closed successfuly - TxID: ", tx)
        }
        catch(error) {
            console.log("Error while closing the account: ", error);
        }
    }
    return (
        <>
        {scores.map((score) => {
            for (let i = 0; i < score.tournament.length; i++) {
                console.log("Player ", i + 1, " address is ", score.tournament[i].user.toString());
                players[i] = score.tournament[i].user;
                let playerPDA = findProgramAddressSync([utils.bytes.utf8.encode("VinciWorldAccount1"), score.tournament[i].user.toBuffer()], accountsProgramID)
                playerList[i] = {pubkey: playerPDA[0], isSigner: false, isWritable: true};
                console.log(playerList[i]);
                console.log("Player ", i + 1, " score is ", score.tournament[i].score.toString());
                <div className="md:hero-content flex flex-col">
                    <h1>{score.tournament[i].user.toString()}</h1>
                    <h1>{score.tournament[i].score.toString()}</h1>
                </div>
            }
            return(
                <div className="md:hero-content flex flex-col">
                    <h1>Player 1: {score.tournament[0].user.toString()}</h1>
                    <span>Score: {score.tournament[0].score.toString()}</span>
                    <span>Level: {score.tournament[0].level.toString()}</span>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => updateScore(score.tournament[0].user, true)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Update Player Score (+30)
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => updateScore(score.tournament[0].user, false)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Update Player Score (-Rand)
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => upgrade(score.tournament[0].user)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            !!!UPGRADE!!!
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => updateScore(score.tournament[0].user, true)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            !!!MEGA UPGRADE!!!
                        </span>
                    </button>

                    <h1>Player 2: {score.tournament[1].user.toString()}</h1>
                    <span>Score: {score.tournament[1].score.toString()}</span>
                    <span>Level: {score.tournament[1].level.toString()}</span>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => updateScore(score.tournament[1].user, true)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Update Player Score (+30)
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => updateScore(score.tournament[1].user, false)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Update Player Score (-Rand)
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => upgrade(score.tournament[1].user)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            !!!UPGRADE!!!
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => updateScore(score.tournament[1].user, true)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            !!!MEGA UPGRADE!!!
                        </span>
                    </button>
                </div>                            
            )
        })
        }
        <div className="flex flex-row justify-center">
            <div className="relative group items-center">
                <div className="m-1 absolute -inset-0.5 bg-gradient-to-r from-indigo-500 to-fuchsia-500 
                rounded-lg blur opacity-20 group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-tilt"></div>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={createQuiz} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Create Vinci Quiz Account
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={closeQuiz} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Close Vinci Quiz Account
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={getScores} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            List Player / Scores
                        </span>
                    </button>
             </div>
        </div>
        <div className="flex flex-row justify-center">
            <div className="relative group items-center">
                <div className="m-1 absolute -inset-0.5 bg-gradient-to-r from-indigo-500 to-fuchsia-500 
                rounded-lg blur opacity-20 group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-tilt"></div>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={addPlayer} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Add Player 1 to Quiz
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={addPlayer2} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Add Player 2 to Quiz
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={distributeRewards} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Distribute Rewards
                        </span>
                    </button>
             </div>
        </div>
        <div className="flex flex-row justify-center">
            <div className="relative group items-center">
                <div className="m-1 absolute -inset-0.5 bg-gradient-to-r from-indigo-500 to-fuchsia-500 
                rounded-lg blur opacity-20 group-hover:opacity-100 transition duration-1000 group-hover:duration-200 animate-tilt"></div>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={createAccount} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Create Vinci Account
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => closeAccount(vinciAccountPDA[0])} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Close Vinci Account for Player 1
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => closeAccount(vinciAccountPDA2[0])} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Close Vinci Account for Player 2
                        </span>
                    </button>
             </div>
        </div>
        </>
    );
};
