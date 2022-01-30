use anchor_lang::prelude::*;
use solana_gateway::Gateway;

declare_id!("<PROGRAM_ID>");

#[program]
pub mod bt_kyc {
    use super::*;

    pub fn claim_land(
        ctx: Context<ClaimLand>,
        gatekeeper_network: Pubkey) -> ProgramResult {
        let user_wallet = &ctx.accounts.user_wallet;
        let gateway_token = &ctx.accounts.gateway_token;
        Gateway::verify_gateway_token_account_info(
            &gateway_token, &user_wallet.key, &gatekeeper_network
        )?;
        msg!("bt-kyc:{}:{}", user_wallet.key, gateway_token.key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimLand<'info> {
    gateway_token: AccountInfo<'info>,
    #[account(mut)]
    user_wallet: Signer<'info>
}
