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
import { Dao } from "../target/types/dao";
import { BN } from "bn.js";

const commitment: Commitment = "confirmed";

describe("dao", () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Dao as Program<Dao>;
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
  const dao = PublicKey.findProgramAddressSync(
    // seeds = [b"dao", creator.key().as_ref(), mint.key().as_ref()]
    [Buffer.from("dao"), user1.publicKey.toBytes(), mint.publicKey.toBytes()],
    program.programId
  )[0];
  const vault = PublicKey.findProgramAddressSync(
    // seeds = [b"vp_vault", creator.key().as_ref(), mint.key().as_ref()]
    [Buffer.from("vault"), user1.publicKey.toBytes(), mint.publicKey.toBytes()],
    program.programId
  )[0];
  let user1Ata: Account;
  let user2Ata: Account;
  let user3Ata: Account;

  const decimals = 6;
  const amount = new BN(200 * 1 * 10 ** decimals);

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

    let user1MintTo = await mintTo(
      connection,
      user1,
      token,
      user1Ata.address,
      user1.publicKey,
      100 * 1 * 10 ** decimals
    );
    console.log(`https://explorer.solana.com/tx/${user1MintTo}?cluster=devnet`);
    let user1TokenAmount = await connection.getTokenAccountBalance(user1Ata.address);
    console.log(
      `minted ${user1TokenAmount.value.uiAmountString} ${token.toBase58()} tokens for user1`
    );

    user2Ata = await getOrCreateAssociatedTokenAccount(
      connection,
      user1,
      token,
      user2.publicKey
    );

    let user2MintTo = await mintTo(
      connection,
      user1,
      token,
      user2Ata.address,
      user1.publicKey,
      100 * 1 * 10 ** decimals
    );
    console.log(`https://explorer.solana.com/tx/${user2MintTo}?cluster=devnet`);
    let user2TokenAmount = await connection.getTokenAccountBalance(user2Ata.address);
    console.log(
      `minted ${user2TokenAmount.value.uiAmountString} ${token.toBase58()} tokens for user2`
    );

    user3Ata = await getOrCreateAssociatedTokenAccount(
      connection,
      user1,
      token,
      user3.publicKey
    );

    let user3MintTo = await mintTo(
      connection,
      user1,
      token,
      user3Ata.address,
      user1.publicKey,
      100 * 1 * 10 ** decimals
    );
    console.log(`https://explorer.solana.com/tx/${user2MintTo}?cluster=devnet`);
    let user3TokenAmount = await connection.getTokenAccountBalance(user2Ata.address);
    console.log(
      `minted ${user3TokenAmount.value.uiAmountString} ${token.toBase58()} tokens for user3`
    );
  });

  it("initialize analytics", async () => {
    await program.methods.init()
      .accounts({
        signer: user1.publicKey,
        auth,
        analytics,
      })
      .signers([user1])
      .rpc()
      .then(confirmTx);
  });

  it("create dao", async () => {
    await program.methods.daoCreate({ twentyFourHours: {} }, 51, new BN(100), "Monolith DAO")
      .accounts({
        creator: user1.publicKey,
        auth,
        dao,
        signerAta: user1Ata.address,
        vault,
        mint: mint.publicKey,
        analytics,
      })
      .signers([user1])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        const daoDebug = await program.account.dao.fetch(dao);
        console.log(daoDebug)
      });
  });

  it("user1 stake 100 tokens", async () => {
    await program.methods.stake(new BN(100))
      .accounts({
        user: user1.publicKey,
        auth,
        dao,
        signerAta: user1Ata.address,
        vault,
        mint: mint.publicKey,
        analytics,
      })
      .signers([user1])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        const daoDebug = await program.account.dao.fetch(dao);
        console.log(daoDebug)
      });
  });

  it("user2 stake 50 tokens", async () => {
    await program.methods.stake(new BN(50))
      .accounts({
        user: user2.publicKey,
        auth,
        dao,
        signerAta: user2Ata.address,
        vault,
        mint: mint.publicKey,
        analytics,
      })
      .signers([user2])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        const daoDebug = await program.account.dao.fetch(dao);
        console.log(daoDebug)
      });
  });

  it("user1 start poll", async () => {
    await program.methods.pollCreate(
      "should i get hired ?",
      "i have only 3 years of experience in both crypto and coding, all self-taught + WBA/Turbin3 2023 Q3 cohort."
    )
      .accounts({
        signer: user1.publicKey,
        dao,
        analytics,
      })
      .signers([user1])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        const daoDebug = await program.account.dao.fetch(dao);
        console.log(daoDebug)
      });
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
