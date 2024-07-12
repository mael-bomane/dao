import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import {
  Account,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { DaoVoting } from "../target/types/dao_voting";
import { BN } from "bn.js";

const commitment: Commitment = "confirmed";

describe("dao-voting", () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.DaoVoting as Program<DaoVoting>;
  const connection: Connection = anchor.getProvider().connection;

  const user1 = new Keypair();
  const user2 = new Keypair();
  const user3 = new Keypair();

  const mint = Keypair.generate();

  let mintAddress: string;

  const analytics = PublicKey.findProgramAddressSync(
    [Buffer.from("analytics")],
    program.programId
  )[0];
  const auth = PublicKey.findProgramAddressSync(
    [Buffer.from("auth")],
    program.programId
  )[0];
  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault")],
    program.programId
  )[0];
  const dao = PublicKey.findProgramAddressSync(
    // seeds = [b"dao", creator.key().as_ref(), mint.key().as_ref()]
    [Buffer.from("dao"), user1.publicKey.toBytes(), mint.publicKey.toBytes()],
    program.programId
  )[0];
  const vpVault = PublicKey.findProgramAddressSync(
    // seeds = [b"vp_vault", creator.key().as_ref(), mint.key().as_ref()]
    [Buffer.from("vp_vault"), user1.publicKey.toBytes(), mint.publicKey.toBytes()],
    program.programId
  )[0];
  let user1Ata: Account;
  let user2Ata: Account;
  let user3Ata: Account;

  const amount = new BN(100 * 1 * 10 ** 6);

  before(async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        user1.publicKey,
        100 * LAMPORTS_PER_SOL
      )
      .then(confirmTx);

    await anchor
      .getProvider()
      .connection.requestAirdrop(
        user2.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        user3.publicKey,
        100 * LAMPORTS_PER_SOL
      )
      .then(confirmTx);

    const decimals = 6;

    let token = await createMint(
      connection,
      user1,
      user1.publicKey,
      user1.publicKey,
      decimals,
      mint
    );
    console.log("Token : ", token.toBase58());
    mintAddress = token.toBase58();
    user1Ata = await getOrCreateAssociatedTokenAccount(
      connection,
      user1,
      token,
      user1.publicKey
    );
    console.log("User 1 Associated Token Aaccount : ", user1Ata.address.toBase58());
    let sendToken = await mintTo(
      connection,
      user1,
      token,
      user1Ata.address,
      user1.publicKey,
      100 * 1 * 10 ** decimals
    );
    console.log(`https://explorer.solana.com/tx/${sendToken}?cluster=devnet`);
    let tokenAmount = await connection.getTokenAccountBalance(user1Ata.address);
    console.log(
      `minted ${tokenAmount.value.uiAmountString} ${token.toBase58()} tokens`
    );
  })

  it("initialize analytics", async () => {
    await program.methods.init()
      .accounts({
        signer: user1.publicKey,
        auth,
        vault,
        analytics,
      })
      .signers([user1])
      .rpc()
      .then(confirmTx);;
  });

  it("create dao", async () => {
    await program.methods.daoCreate({ twentyFourHours: {} })
      .accounts({
        creator: user1.publicKey,
        auth,
        dao,
        signerAta: user1Ata.address,
        vpVault,
        mint: mint.publicKey,
        analytics,
      })
      .signers([user1])
      .rpc()
      .then(confirmTx);
  });
});

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    commitment
  );
};
