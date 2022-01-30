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
            KYC Program
            <br/>
        </h3>
        <br/>
    </div>
</p>


# I. Overview 

At Blue Terra we believe in the vision of building a protocol for the democratizing property ownership rights. 

Doing this means sharing some of the secret sauce that makes Blue Terra possible so that others may build on the model.

The following is the Anchor Program used in our claim land and KYC process. This repo contains the source code for the Blue Terra [Civic](https://www.civic.com/) KYC nonce Solana program.

# I. Program

The program is a minimalist nonce written in Anchor Lang and provides Blue Terra a simple way to keep track of KYC verified wallets and civic tokens on-chain.

In particular, if a Blue Terra NFT holder has a valid KYC, through the posession of a avalid civic `gatewayToken` spl token, then they will be able to interact with a the Blue Terra KYC Program to claim their land and activate the leases embedded within the NFTs.

The program contains one instruction `claimLand`. This instruction leverages the Civic Gateway decorator to detect the KYC status of a given `walletAddress` and `gatewayToken`. 

On detection of valid Civic KYC status, the program will log the `walletAddress` and `gatewayToken` of the user.

# III. Requirements

1) [Rustup](https://rustup.rs/)
2) [Solana](https://docs.solana.com/cli/install-solana-cli-tools)
3) [Anchor](https://github.com/project-serum/anchor)

## III. Building

To build this program run: 

    anchor build 

On successful build you will have a `target` directory in your top level directory.

## IV. Deployment 

To deploy the program follow the steps below: 

Find the program's address by running:

    solana address -k ./target/deploy/bt_kyc-keypair.json

Copy and paste this address into `./Anchor.toml` and `./programs/bt-kyc`

Finally run the following:

    anchor deploy 

## V. Client Side Instrumentation

Great, now you have this Solana Nonce Program onchain, now you want your client to interact with your program. 
We recommend using the [@project-serum/anchor](https://www.npmjs.com/package/@project-serum/anchor) to instrument 
program rpc requests from the client. 





