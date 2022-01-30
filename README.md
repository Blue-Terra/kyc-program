</br>
<p align="center">
    <img
        style="width:20%;height:auto;"
        src="./docs/img/logo.png">
    </img>
    <div align="center">
        <h3 style="font-size:26px;line-height:40px">
            Blue Terra Engineering
            <br/>
            <br/>
            🏝️ KYC Program
            <br/>
        </h3>
        <br/>
    </div>
</p>


# I. Overview 

Blue Terra is a decentralized protocol for the global democratization of stable and accessible property rights.  

Toward this end, we believe in the power of sharing some of the secret sauce that makes Blue Terra possible so that others may build and expand on the core model.

This repo contains the source code for the Blue Terra [Civic](https://www.civic.com/) KYC nonce Solana program.

# II. Requirements

1) [Rustup](https://rustup.rs/)
2) [Solana](https://docs.solana.com/cli/install-solana-cli-tools)
3) [Anchor](https://github.com/project-serum/anchor)

# III. Program

The following is a high level description of the Anchor Program used in our claim land and KYC process. 

The program is a minimalist nonce written in Anchor Lang and provides Blue Terra a simple way to keep track of KYC verified wallets and civic tokens on-chain.

In particular, if a Blue Terra NFT holder has a valid KYC, through the posession of a valid civic `gatewayToken` SPL token, then they will be able to interact with the `kyc-program` to claim their land and activate the leases embedded within their NFTs.

The program contains one instruction, `claimLand`. This instruction leverages the Civic Gateway decorator to detect the KYC status of a given `walletAddress` and `gatewayToken`. 

On detection of a valid Civic KYC status, the program will log the `walletAddress` and `gatewayToken` of the user to the program standard output.

## III. Building

To build this program run: 

    anchor build 

On successful build you will have a `target` directory in your top level.

## IV. Deployment 

To deploy the program follow the steps below: 

Find the program's address by running:

    solana address -k ./target/deploy/bt_kyc-keypair.json

Copy and paste this address into `./Anchor.toml` and `./programs/bt-kyc`

Finally run the following:

    anchor deploy 

## V. Client Side Instrumentation

To interact with the program from a NodeJS client we recommend using:

1) [@project-serum/anchor](https://www.npmjs.com/package/@project-serum/anchor) to instrument program rpc requests from the client. 
2) [@solana/web3.js](https://www.npmjs.com/package/@solana/web3.js) for NodeJS Solana libraries. 
3) [@civic/solana-gateway-react](https://www.npmjs.com/package/@civic/solana-gateway-react) for Civic's KYC Libraries and React Providers. 
4) [@solana/wallet-adapter](https://www.npmjs.com/package/@solana/wallet-adapter) for Solana React wallet adapter components. 
<br/>
<br/>
A high level sketch of this interaction can be found below. 
<br/>
<br/>
<p align="center">
    <img
        style="width:90%;height:auto;"
        src="./docs/img/client.png">
    </img>
</p>
