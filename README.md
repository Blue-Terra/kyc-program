</br>
<p align="center">
    <img
        style="width:20%;height:auto;"
        src="./docs/img/logo.png">
    </img>
</p>

<div style="font-size:2.0em;text-align:center;">
    Blue Terra Engineering
</div>
<div style="font-size:1.7em;text-align:center;">
    KYC Program
</div>

# I. Overview 

At Blue Terra we believe in the vision of building a protocol for the democratizing property ownership rights. Doing this means sharing some of the secret sauce that makes Blue Terra possible so that others may expand the model. The following is the Anchor Program used in our claim land and kyc process.

# I. Program

This repo contains the source code for the Blue Terra [Civic](https://docs.civic.com/) KYC nonce Solana program.

The program is minimalist nonce program, written in Anchor Lang and provides Blue Terra a simple way to keep track of KYC verified wallets. If a Blue Terra NFT holder does not have valid KYC, they will be able to interact with a KYC Program to claim their land. 

The program contains one instruction `claimLand`. This instruction leverages the Civic Gateway decorator to detect the KYC status of a given `walletAddress` and `gatewayToken`. 

On detection of valid KYC status, the program will log the `walletAddress` and `gatewayToken` of the user.

# III. Requirements

1) [Rustup](https://rustup.rs/)
2) [Solana](https://docs.solana.com/cli/install-solana-cli-tools)
3) [Anchor]()

## III. Building

To build this program run: 

    anchor build 

## IV. Deployment 

To deploy the program follow the steps below: 

Find the program's address by running:

    solana address -k ./target/deploy/bt_kyc-keypair.json

Copy and paste this address into `./Anchor.toml` and `./programs/bt-kyc`

Finally run the following:

    anchor deploy 








