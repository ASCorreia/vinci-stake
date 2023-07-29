import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionMessage, TransactionSignature, VersionedTransaction } from '@solana/web3.js';
import * as spl from "@solana/spl-token";
import bs58 from 'bs58';
import { FC, useCallback, useEffect, useState } from 'react';
import { notify } from "../utils/notifications";

import { Program, AnchorProvider, web3, utils, BN } from '@project-serum/anchor';
import idl from "./solanapdas.json";
import idlSwap from "./solanapdas2.json";
import { program } from '@project-serum/anchor/dist/cjs/native/system';
import { ASSOCIATED_PROGRAM_ID } from '@project-serum/anchor/dist/cjs/utils/token';

const idl_string = JSON.stringify(idl);
const idl_object = JSON.parse(idl_string);
const programID = new PublicKey(idl.metadata.address);

const idl_string_swap = JSON.stringify(idlSwap);
const idl_object_swap = JSON.parse(idl_string_swap);
const swapProgramID = new PublicKey(idlSwap.metadata.address);

export const Swap: FC = () => {
    const { connection } = useConnection();
    const ourWallet = useWallet();

    const [banks, setBanks] = useState([]);
    const [pools, setPools] = useState([]);

    const [poolPDA1, setPoolPDA1] = useState([]);
    const [poolPDA2, setPoolPDA2] = useState([]);

    const [poolBalance1, setPoolBalance1] = useState(0);
    const [poolBalance2, setPoolBalance2] = useState(0);

    const [receiveAmount1, setReceiveAmount1] = useState(0);
    const [receiveAmount2, setReceiveAmount2] = useState(0);

    const getProvider = () => {
        const provider = new AnchorProvider(connection, ourWallet, AnchorProvider.defaultOptions())
        return provider;
    }

    const anchProvider = getProvider();
    const program = new Program(idl_object, programID, anchProvider);
    const programSwap = new Program(idl_object_swap, swapProgramID, anchProvider);

    const [vinciSwap, _] = PublicKey.findProgramAddressSync([
        utils.bytes.utf8.encode("VinciSwap"),
    ], programSwap.programId);

    const mint1 = new PublicKey("8LbiacZvDREPUa5a7Ljth16G9p1BoKXccqs5cMcjuhfu");
    const mint2 = new PublicKey("E7sRawws3T77FLf5P7u5W1gBA9ex2H6TfFCxKQJA2TYA");

    const createPool = async() => {
        try {
            const tx = await programSwap.methods.initialize().accounts({
                vinciSwap: vinciSwap,
                user: anchProvider.wallet.publicKey,
                SystemProgram: web3.SystemProgram.programId,
            }).rpc();

            console.log("Vinci Liquidity Pool Account created - TxID: ", tx);
        }
        catch(error) {
            console.log("Error creating the Liquidity Pool ", error);
        }
    }

    const addToken1 = async() => {
        try {
            const tx = await programSwap.methods.addToken().accounts({
                vinciSwap: vinciSwap,
                mint: mint1,
                payer: anchProvider.wallet.publicKey,
                SystemProgram: web3.SystemProgram.programId,
            }).rpc();

            console.log("Token 1 added to Liquidity Pool - TxID: ", tx);

            setPoolBalance1(0 as any);
        }
        catch(error) {
            console.log("Error adding token ", mint1, " to Liquidity Pool: ", error);
        }
    }

    const addToken2 = async() => {
        try {
            const tx = await programSwap.methods.addToken().accounts({
                vinciSwap: vinciSwap,
                mint: mint2,
                payer: anchProvider.wallet.publicKey,
                SystemProgram: web3.SystemProgram.programId,
            }).rpc();

            console.log("Token 2 added to Liquidity Pool - TxID: ", tx);

            setPoolBalance2(0 as any);
        }
        catch(error) {
            console.log("Error adding token ", mint2, " to Liquidity Pool: ", error);
        }
    }

    //Get the owner ATA for our second Token
    const ownerATA2 = spl.getAssociatedTokenAddress(mint1, anchProvider.wallet.publicKey);

    //Get the vault ATA for our second Token
    const vaultATA2 = spl.getAssociatedTokenAddress(mint2, vinciSwap, true);

    const addLiquidity1 = async() => {
        try {
            //Get the owner ATA for our first Token
            const ownerATA1 = await spl.getAssociatedTokenAddress(mint1, anchProvider.wallet.publicKey);

            //Get the vault ATA for our first Token
            const vaultATA1 = await spl.getAssociatedTokenAddress(mint1, vinciSwap, true);

            const tx = await programSwap.methods.addLiquidity(new BN(1_000_000)).accounts({
                vinciSwap: vinciSwap,
                ownerAta: ownerATA1,
                vaultAta: vaultATA1,
                tokenMint: mint1,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                user: anchProvider.wallet.publicKey,
            }).rpc();

            let balance = (await connection.getTokenAccountBalance(vaultATA1)).value.amount;
            setPoolBalance1(parseInt(balance));

            console.log(1 * LAMPORTS_PER_SOL, " Tokens with mint ID ", mint1.toBase58(), " Successfuly sent to the Program Vault / Liquidity Pool");
            console.log("TxID: ", tx);
        }
        catch(error) {
            console.log("Error adding liquidity to ", mint1, " Liquidity Pool: ", error);
        }
    }

    const addLiquidity2 = async() => {
        try {
            //Get the owner ATA for our second Token
            const ownerATA2 = await spl.getAssociatedTokenAddress(mint2, anchProvider.wallet.publicKey);

            //Get the vault ATA for our second Token
            const vaultATA2 = await spl.getAssociatedTokenAddress(mint2, vinciSwap, true);

            setPoolPDA2(vaultATA2 as any);

            const tx = await programSwap.methods.addLiquidity(new BN(1_000_000)).accounts({
                vinciSwap: vinciSwap,
                ownerAta: ownerATA2,
                vaultAta: vaultATA2,
                tokenMint: mint2,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                user: anchProvider.wallet.publicKey,
            }).rpc();

            let balance = (await connection.getTokenAccountBalance(vaultATA2)).value.amount;
            setPoolBalance2(parseInt(balance));

            console.log(1 * LAMPORTS_PER_SOL, " Tokens with mint ID ", mint1.toBase58(), " Successfuly sent to the Program Vault / Liquidity Pool");
            console.log("TxID: ", tx);
        }
        catch(error) {
            console.log("Error adding liquidity to ", mint2, " Liquidity Pool: ", error);
        }
    }

    const closePool = async() => {
        try {
            const tx = await programSwap.methods.close().accounts({
                vinciSwap: vinciSwap,
                destination: anchProvider.wallet.publicKey,
            }).rpc();

            console.log("Vinci Liquidity Pool successfully closed - TxID: ", tx);
        }
        catch(error) {
            console.log("Error while closing the liquidity pool: ", error);
        }
    }

    const swapToken12 = async() => {
        try {
            //Get the owner ATA for our first Token
            const ownerATA1 = await spl.getAssociatedTokenAddress(mint1, anchProvider.wallet.publicKey);
            //Get the vault ATA for our first Token
            const vaultATA1 = await spl.getAssociatedTokenAddress(mint1, vinciSwap, true);

            //Get the owner ATA for our second Token
            const ownerATA2 = await spl.getAssociatedTokenAddress(mint2, anchProvider.wallet.publicKey);
            //Get the vault ATA for our second Token
            const vaultATA2 = await spl.getAssociatedTokenAddress(mint2, vinciSwap, true);

            const tx = await programSwap.methods.swap(new BN(0.5 * 1_000_000)).accounts({
                vinciSwap: vinciSwap,
                userReceiveMint: mint1,
                userReceiveTokenAccount: ownerATA1,
                poolReceiveTokenAccount: vaultATA1,
                userPayMint: mint2,
                userPayTokenAccount: ownerATA2,
                poolPayTokenAccount: vaultATA2,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                user: anchProvider.wallet.publicKey,
            }).rpc()
        }
        catch(error) {
            console.log("Error while swapping token 1 for token 2: ", error);
        }
    }

    const swapToken21 = async() => {
        try {
            //Get the owner ATA for our first Token
            const ownerATA1 = await spl.getAssociatedTokenAddress(mint1, anchProvider.wallet.publicKey);
            //Get the vault ATA for our first Token
            const vaultATA1 = await spl.getAssociatedTokenAddress(mint1, vinciSwap, true);

            //Get the owner ATA for our second Token
            const ownerATA2 = await spl.getAssociatedTokenAddress(mint2, anchProvider.wallet.publicKey);
            //Get the vault ATA for our second Token
            const vaultATA2 = await spl.getAssociatedTokenAddress(mint2, vinciSwap, true);

            const tx = await programSwap.methods.swap(new BN(0.5 * 1_000_000)).accounts({
                vinciSwap: vinciSwap,
                userReceiveMint: mint2,
                userReceiveTokenAccount: ownerATA2,
                poolReceiveTokenAccount: vaultATA2,
                userPayMint: mint1,
                userPayTokenAccount: ownerATA1,
                poolPayTokenAccount: vaultATA1,
                tokenProgram: spl.TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
                user: anchProvider.wallet.publicKey,
            }).rpc()
        }
        catch(error) {
            console.log("Error while swapping token 2 for token 1: ", error);
        }
    }

    const calcAmmount = () => {
        useEffect(() => {
            // Calculate the receive amount based on the constant product formula
            const r = (poolBalance1 as any * 1) / (poolBalance1 as any + 1)
            const adjustedR = r / Math.pow(10, 6)
            const roundedR = Math.round(adjustedR * 100) / 100
            setReceiveAmount1(roundedR)
        }, [poolBalance1, poolBalance2])
    }

    //TBD: Deploy Swap program to DevNet and add function to return of this function (buttons, whatever)

    const createBank = async() => {
        try {
            const [bank, _] = await PublicKey.findProgramAddressSync([
                utils.bytes.utf8.encode("bankaccount"),
                anchProvider.wallet.publicKey.toBuffer(),
            ], program.programId)

            await program.methods.create("WSoS Bank").accounts({
              bank,
              user: anchProvider.wallet.publicKey,
              systemProgram: web3.SystemProgram.programId,
            }).rpc();

            console.log("Wow, new bank was created" + bank.toString());
        }
        catch (error) {
            console.log("Error while creating bank account " + error);
        }
    }

    const getBanks = async () => {
        try {
            await Promise.all((await connection.getProgramAccounts(programID)).map(async bank => ({
                ...(await program.account.bank.fetch(bank.pubkey)),
                pubKey: bank.pubkey
            }))).then(banks => {
                console.log(banks);
                setBanks(banks);
            })
        }
        catch (error) {
            console.log("Error while getting the bank accounts " + error);
        }
    }

    const getPools = async () => {
        try {
            await Promise.all((await connection.getProgramAccounts(swapProgramID)).map(async poll => ({
                ...(await programSwap.account.vinciSwap.fetch(poll.pubkey)),
                pubKey: poll.pubkey
            }))).then(polls => {
                console.log(polls);
                setPools(polls);
            })

            const vaultATA1 = await spl.getAssociatedTokenAddress(mint1, vinciSwap, true);
            let balance1 = (await connection.getTokenAccountBalance(vaultATA1)).value.amount;
            setPoolBalance1(parseInt(balance1));

            const vaultATA2 = await spl.getAssociatedTokenAddress(mint2, vinciSwap, true);
            let balance2 = (await connection.getTokenAccountBalance(vaultATA2)).value.amount;
            setPoolBalance2(parseInt(balance2));

            const r = (poolBalance2 * 100) / (poolBalance1 + 100)
            const adjustedR = r / Math.pow(10, 6)
            const roundedR = Math.round(adjustedR * 100) / 100
            console.log("Rounded R: ", roundedR);
            setReceiveAmount1(roundedR)

            const r2 = (poolBalance1 * 100) / (poolBalance2 + 100)
            const adjustedR2 = r2 / Math.pow(10, 6)
            const roundedR2 = Math.round(adjustedR2 * 100) / 100
            console.log("Rounded R: ", roundedR2);
            setReceiveAmount2(roundedR2)
        }
        catch (error) {
            console.log("Error while getting the bank accounts " + error);
        }
    }

    const depositBank = async(publicKey) => {
        try {
          await program.methods.deposit(new BN(0.1 * LAMPORTS_PER_SOL)).accounts({
              bank: publicKey,
              user: anchProvider.wallet.publicKey,
              systemProgram: web3.SystemProgram.programId,
            }).rpc();

            console.log("Deposit done: " + publicKey);
        }
        catch (error) {
            console.log("Error while depositing into bank account" + error);
        }
    }

    const withdrawBank = async(publicKey) => {
        try {
            await program.methods.withdraw(new BN(0.1 * LAMPORTS_PER_SOL)).accounts({
                bank: publicKey,
                user: anchProvider.wallet.publicKey,
            }).rpc();

            console.log("Withdraw done: " + publicKey);
        }
        catch (error) {
            console.log("Error while withdrawing from bank account " + error);
        }
    }

    return (
        <>
        {/*banks.map((bank) => {
            return(
                <div className="md:hero-content flex flex-col">
                    <h1>{bank.name.toString()}</h1>
                    <span>{bank.balance.toString()}</span>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => depositBank(bank.pubKey)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Swap 0.5 Tokens
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => withdrawBank(bank.pubKey)} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Deposit 0.5 Tokens
                        </span>
                    </button>
                </div>
            )
        })*/
        pools.map((poll) => {
            for (let i = 0; i < poll.assets.length; i++) {
                console.log("asset ", i, " is ", poll.assets[i].toString());
                <div className="md:hero-content flex flex-col">
                    <h1>{poll.assets[i].toString()}</h1>
                </div>
            }
            return(
                <div className="md:hero-content flex flex-col">
                    <h1>Pool Token 1: {poll.assets[0].toString()}</h1>
                    <span>Pool Balance: {poolBalance1.toString()}</span>
                    <span>Wallet PubKey: {ourWallet.publicKey.toString()}</span>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => swapToken12()} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Swap 0.5 Tokens
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => addLiquidity1()} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Deposit 0.5 Tokens
                        </span>
                    </button>

                    <h1>Pool Token 2: {poll.assets[1].toString()}</h1>
                    <span>Pool Balance: {poolBalance2.toString()}</span>
                    <span>Wallet PubKey: {ourWallet.publicKey.toString()}</span>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => swapToken21()} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Swap 0.5 Tokens
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={() => addLiquidity2()} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Deposit 0.5 Tokens
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
                        onClick={createPool} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Create Liquidity Pool
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={closePool} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Close Liquidity Pool
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={getPools} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            List Liquidity Pools
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
                        onClick={addToken1} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Add Token 1 to Pool
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={addToken2} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Add Token 2 to Pool
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={addLiquidity1} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Add Liquidity for Token 1
                        </span>
                    </button>
                    <button
                        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
                        onClick={addLiquidity2} disabled={!ourWallet.publicKey}
                    >
                        <div className="hidden group-disabled:block ">
                        Wallet not connected
                        </div>
                         <span className="block group-disabled:hidden" >
                            Add Liquidity for Token 2
                        </span>
                    </button>
             </div>
        </div>
        </>
    );
};
