import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LilPyth } from "../target/types/lil_pyth";

describe("lilPyth", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.LilPyth as Program<LilPyth>;

  // checkaaaaut -> https://pyth.network/developers/accounts/?cluster=devnet#
  let SolPriceAccount = new anchor.web3.PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix")
  let SolProductAccount = new anchor.web3.PublicKey("3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E");

  it("Gib SolPrice ser", async () => {
    const tx = await program.methods.gibPrice().accounts({
      pythPrice: SolPriceAccount,
      pythPoduct: SolProductAccount
    }).rpc();
    console.log("txHash", tx);
  });
});
