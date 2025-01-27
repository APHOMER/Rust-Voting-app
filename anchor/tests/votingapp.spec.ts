import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair, PublicKey} from '@solana/web3.js'
import { Votingapp } from '../target/types/votingapp'

import { BankrunProvider, startAnchor } from "anchor-bankrun";
// import { startAnchor } from 'solana-bankrun';
// import { BankrunProvider } from 'anchor-bankrun';
//................
// import { program } from '@coral-xyz/anchor/dist/cjs/native/system';


const IDL = require('../target/idl/votingapp.json');
// const votingAddress = new PublicKey("AyD5wnZEk5po81hTJBG8LxVRHb7GdewWUVhVHv24dm3");
const votingAddress = new PublicKey("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

describe('votingapp', () => {
  
  it('Initialize Poll', async () => {
    try {
      const context = await startAnchor("", [{ name: "votingapp", programId: votingAddress }], []);
      const provider = new BankrunProvider(context);
  
      const votingProgram = new Program<Votingapp>(
        IDL as any,
        // IDL,
        votingAddress,
        provider,
      );
  
      await votingProgram.methods.initializePoll(
        new anchor.BN(1),
        "What is your favorite type of peanut butter?",
        new anchor.BN(0),
        new anchor.BN(Math.floor(Date.now() / 1000) + 60 * 60 * 24), // Poll ends in 24 hours
      ).rpc();
    } catch (error) {
      console.error("Error during Initialize Poll:", error);
      throw error; // Re-throw the error to fail the test
    }
  });
  
  
  
  
  
  
  
  
  
  
  
  // it('Initialize Poll', async () => {
  //   const context = await startAnchor("", [{ name: "votingapp", programId: votingAddress }], []);
	//   const provider = new BankrunProvider(context);

  //   // const votingProgram = new Program(
  //   const votingProgram = new Program<Votingapp>(
  //     IDL, // The IDL file
  //     votingAddress, // Program ID (Public Key of the deployed program)
  //     provider, // The provider instance
  //   );
      
  //   const endTimestamp = Math.floor(Date.now() / 1000) + 60 * 60 * 24; // 24 hours from now

  //   await votingProgram.methods.initializePoll(
  //     new anchor.BN(1),
  //     "What is your favorite type of peanut butter ?",
  //     new anchor.BN(0),
  //     new anchor.BN(endTimestamp),
  //   ).rpc();

  // });


});




    
  //   await program.methods
  //     .initialize()
  //     .accounts({
  //       votingapp: votingappKeypair.publicKey,
  //       payer: payer.publicKey,
  //     })
  //     .signers([votingappKeypair])
  //     .rpc()

  //   const currentCount = await program.account.votingapp.fetch(votingappKeypair.publicKey)

  //   expect(currentCount.count).toEqual(0)
  // })

  // it('Increment Votingapp', async () => {
  //   await program.methods.increment().accounts({ votingapp: votingappKeypair.publicKey }).rpc()

  //   const currentCount = await program.account.votingapp.fetch(votingappKeypair.publicKey)

  //   expect(currentCount.count).toEqual(1)
  // })

  // it('Increment Votingapp Again', async () => {
  //   await program.methods.increment().accounts({ votingapp: votingappKeypair.publicKey }).rpc()

  //   const currentCount = await program.account.votingapp.fetch(votingappKeypair.publicKey)

  //   expect(currentCount.count).toEqual(2)
  // })

  // it('Decrement Votingapp', async () => {
  //   await program.methods.decrement().accounts({ votingapp: votingappKeypair.publicKey }).rpc()

  //   const currentCount = await program.account.votingapp.fetch(votingappKeypair.publicKey)

  //   expect(currentCount.count).toEqual(1)
  // })

  // it('Set votingapp value', async () => {
  //   await program.methods.set(42).accounts({ votingapp: votingappKeypair.publicKey }).rpc()

  //   const currentCount = await program.account.votingapp.fetch(votingappKeypair.publicKey)

  //   expect(currentCount.count).toEqual(42)
  // })

  // it('Set close the votingapp account', async () => {
  //   await program.methods
  //     .close()
  //     .accounts({
  //       payer: payer.publicKey,
  //       votingapp: votingappKeypair.publicKey,
  //     })
  //     .rpc()

  //   // The account should no longer exist, returning null.
  //   const userAccount = await program.account.votingapp.fetchNullable(votingappKeypair.publicKey)
  //   expect(userAccount).toBeNull()