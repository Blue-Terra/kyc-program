import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { BtKyc } from '../target/types/bt_kyc';

describe('bt-kyc', () => {

  const provider = anchor.Provider.Mainnet();
  anchor.setProvider(provider);

  const counter = anchor.web3.Keypair.generate();

  // Program for the tests.
  const program = anchor.workspace.Basic2;


  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.BtKyc as Program<BtKyc>;

  it('Can Claim Land', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
