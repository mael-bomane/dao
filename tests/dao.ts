import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
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
    // seeds = [b"vault", creator.key().as_ref(), mint.key().as_ref()]
    [Buffer.from("vault"), user1.publicKey.toBytes(), mint.publicKey.toBytes()],
    program.programId
  )[0];
  let user1Ata: Account;
  let user2Ata: Account;
  let user3Ata: Account;

  const decimals = 6;
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
    console.log(`https://explorer.solana.com/tx/${user3MintTo}?cluster=devnet`);
    let user3TokenAmount = await connection.getTokenAccountBalance(user3Ata.address);
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

  it("user1 create a dao with 51% threshold, min. 100 tokens required to start poll", async () => {
    // await program.methods.daoCreate({ twentyFourHours: {} }, 51, new BN(100), "Monolith DAO")
    await program.methods.daoCreate({ fiveSeconds: {} }, 51, new BN(100), "Monolith DAO")
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
    const token = new PublicKey(mintAddress);
    await program.methods.stakeNew(new BN(100 * 1 * 10 ** 6))
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
        console.log(daoDebug);
        let user1TokenAmount = await connection.getTokenAccountBalance(user1Ata.address);
        console.log(
          `User1 now have ${user1TokenAmount.value.uiAmountString} ${token.toBase58()} tokens`
        );
      });
  });

  it("user2 stake 50 tokens", async () => {
    await program.methods.stakeNew(new BN(50 * 1 * 10 ** 6))
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
    const title = "should i get hired ?";
    const content = "i've been coding for 3 years (blockchain full-stack rust + typescript), self-taught + WBA/Turbin3 2023 Q3 cohort."

    console.log("title length : ", title.length)
    console.log("content length : ", content.length)

    await program.methods.pollNew(
      title,
      content
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

  it("user1 vote 'approve' on poll 0 /w 100 voting power", async () => {
    await program.methods.voteNew(new BN(0), { approve: {} })
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

  xit("user1 tries voting twice", async () => {
    await program.methods.voteNew(new BN(0), { approve: {} })
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

  it("user2 vote 'reject' on poll 0 /w 50 voting power", async () => {
    await program.methods.voteNew(new BN(0), { reject: {} })
      .accounts({
        signer: user2.publicKey,
        dao,
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

  xit("user1 tries to execute poll 0 before end of voting period", async () => {
    await program.methods.pollExecute(new BN(0))
      .accounts({
        signer: user1.publicKey,
        dao,
        analytics,
      })
      .signers([user1])
      .rpc()
      .then(confirmTx);
  });

  it("user1 execute poll 0 after end of voting period", async () => {
    setTimeout(async () =>
      await program.methods.pollExecute(new BN(0))
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
        }), 5000);
  }).timeout(6000);

  it("user1 deactivate his staked deposits", async () => {
    await program.methods.stakeDeactivate()
      .accounts({
        user: user1.publicKey,
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

  it("user1 claim his deactivated staked deposits", async () => {
    setTimeout(async () =>
      await program.methods.stakeClaim()
        .accounts({
          user: user1.publicKey,
          auth,
          dao,
          signerAta: user1Ata.address,
          mint: mint.publicKey,
          vault,
          analytics,
        })
        .signers([user1])
        .rpc()
        .then(confirmTx)
        .then(async () => {
          const daoDebug = await program.account.dao.fetch(dao);
          console.log(daoDebug);
        }), 5000);
  }).timeout(6000);

  it("shows poll results", async () => {
    const token = new PublicKey(mintAddress);
    setTimeout(async () => {
      const daoDebug = await program.account.dao.fetch(dao);
      daoDebug.users.forEach((user) => {
        console.log(user)
      });
      daoDebug.polls.forEach((poll) => {
        console.log(poll)
        poll.votes.forEach((vote) => {
          console.log(vote)
        })
      });
      let user1TokenAmount = await connection.getTokenAccountBalance(user1Ata.address);
      console.log(
        `User1 now have ${user1TokenAmount.value.uiAmountString} ${token.toBase58()} tokens`
      );

    }, 9000)
  }).timeout(10000);


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
