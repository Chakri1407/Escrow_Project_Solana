import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicEscrow } from "../target/types/basic_escrow";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddress,
  getAccount,
} from "@solana/spl-token";
import { expect } from "chai";

describe("basic-escrow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BasicEscrow as Program<BasicEscrow>;
  const wallet = provider.wallet as anchor.Wallet;

  let mint: anchor.web3.PublicKey;
  let userTokenAccount: anchor.web3.PublicKey; 
  let escrowTokenAccount: anchor.web3.PublicKey; 
  let escrow: anchor.web3.PublicKey; 
  let escrowTokenAccountBump: number; 

  const initialBalance = 1000;
  const depositAmount = 500;
  const withdrawAmount = 300;

  before(async () => {
    mint = await createMint(provider.connection, wallet.payer, wallet.publicKey, null, 6);
    userTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      mint,
      wallet.publicKey
    );
    console.log("Mint PublicKey:", mint.toString());
    console.log("User Token Account PublicKey:", userTokenAccount.toString());

    try {
      await mintTo(provider.connection, wallet.payer, mint, userTokenAccount, wallet.publicKey, initialBalance);
      console.log("Minted initial balance to userTokenAccount");
    } catch (error) {
      console.error("Error minting tokens:", error);
    }

    escrow = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), wallet.publicKey.toBuffer()],
      program.programId
    )[0];

    try {
      [escrowTokenAccount, escrowTokenAccountBump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("escrow"), wallet.publicKey.toBuffer(), mint.toBuffer()],
        program.programId
      );
      console.log("Escrow Token Account PublicKey:", escrowTokenAccount.toString());
    } catch (error) {
      console.error("Error finding program address for escrowTokenAccount:", error);
    }
  });

  it("Initializes the escrow account", async () => {
    await program.methods
      .deposit(new anchor.BN(0))
      .accounts({
        authority: wallet.publicKey,
        escrow: escrow,
        escrowTokenAccount: escrowTokenAccount, 
        userTokenAccount: userTokenAccount, 
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId, 
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    const escrowAccountData = await program.account.escrow.fetch(escrow);
    expect(escrowAccountData.authority.toString()).to.equal(wallet.publicKey.toString());
    expect(escrowAccountData.amount.toNumber()).to.equal(0);

    const escrowTokenAccountInfo = await getAccount(provider.connection, escrowTokenAccount);
    expect(escrowTokenAccountInfo.mint.toString()).to.equal(mint.toString());
    expect(escrowTokenAccountInfo.amount.toString()).to.equal("0");
  });

  it("Deposits tokens into the escrow", async () => {
    const userInitialBalance = (await getAccount(provider.connection, userTokenAccount)).amount;
    await program.methods
      .deposit(new anchor.BN(depositAmount))
      .accounts({
        authority: wallet.publicKey,
        escrow: escrow,
        escrowTokenAccount: escrowTokenAccount, 
        userTokenAccount: userTokenAccount, 
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID, 
        systemProgram: anchor.web3.SystemProgram.programId, 
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    const userFinalBalance = (await getAccount(provider.connection, userTokenAccount)).amount;
    const escrowFinalBalance = (await getAccount(provider.connection, escrowTokenAccount)).amount;
    expect((userInitialBalance - userFinalBalance).toString()).to.equal(depositAmount.toString());
    expect(escrowFinalBalance.toString()).to.equal(depositAmount.toString());

    const escrowAccountData = await program.account.escrow.fetch(escrow);
    expect(escrowAccountData.amount.toNumber()).to.equal(depositAmount);
  });

  it("Withdraws tokens from the escrow", async () => {
    const userInitialBalance = (await getAccount(provider.connection, userTokenAccount)).amount;
    const escrowInitialBalance = (await getAccount(provider.connection, escrowTokenAccount)).amount;
    await program.methods
      .withdraw(new anchor.BN(withdrawAmount))
      .accounts({
        authority: wallet.publicKey,
        escrow: escrow,
        escrowTokenAccount: escrowTokenAccount, 
        userTokenAccount: userTokenAccount, 
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID, 
        systemProgram: anchor.web3.SystemProgram.programId, 
      })
      .rpc();

    const userFinalBalance = (await getAccount(provider.connection, userTokenAccount)).amount;
    const escrowFinalBalance = (await getAccount(provider.connection, escrowTokenAccount)).amount;
    expect((userFinalBalance - userInitialBalance).toString()).to.equal(withdrawAmount.toString());
    expect((escrowInitialBalance - escrowFinalBalance).toString()).to.equal(withdrawAmount.toString());

    const escrowAccountData = await program.account.escrow.fetch(escrow);
    expect(escrowAccountData.amount.toNumber()).to.equal(depositAmount - withdrawAmount);
  });

  it("Fails to withdraw more than available", async () => {
    try {
      await program.methods
        .withdraw(new anchor.BN(depositAmount))
        .accounts({
          authority: wallet.publicKey,
          escrow: escrow,
          escrowTokenAccount: escrowTokenAccount, 
          userTokenAccount: userTokenAccount, 
          mint: mint,
          tokenProgram: TOKEN_PROGRAM_ID, 
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      expect.fail("Expected to throw an error");
    } catch (error) {
      expect(error.toString()).to.include("InsufficientFunds");
    }
  });

  it("Withdraws the remaining balance", async () => {
    const escrowAccountData = await program.account.escrow.fetch(escrow);
    const remainingAmount = escrowAccountData.amount.toNumber();
    await program.methods
      .withdraw(new anchor.BN(remainingAmount))
      .accounts({
        authority: wallet.publicKey,
        escrow: escrow,
        escrowTokenAccount: escrowTokenAccount, 
        userTokenAccount: userTokenAccount, 
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID, 
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const escrowFinalBalance = (await getAccount(provider.connection, escrowTokenAccount)).amount;
    expect(escrowFinalBalance.toString()).to.equal("0");

    const finalEscrowData = await program.account.escrow.fetch(escrow);
    expect(finalEscrowData.amount.toNumber()).to.equal(0);
  });
});