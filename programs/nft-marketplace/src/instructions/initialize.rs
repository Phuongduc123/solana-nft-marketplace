
use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, TokenAccount};
use crate::{MARKETPLACE_PDA_SEED};


use crate::state::*;

pub fn initialize(
    ctx: Context<Initialize>,
    signer: Pubkey,
) -> Result<()> {

    let authorizer = &ctx.accounts.authorizer;
    let marketplace = &mut ctx.accounts.marketplace;
        
    marketplace.authorizer=authorizer.key();
    marketplace.mint_of_nft=ctx.accounts.mint_of_nft.key();
    marketplace.total_minted=0;
    marketplace.receiver=ctx.accounts.receiver.key();
    marketplace.signer=signer.key();

    Ok(())
}

#[derive(Accounts)]
#[instruction(signer: Pubkey)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authorizer: Signer<'info>,
    pub mint_of_nft: Account<'info, Mint>,
    pub receiver: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer=authorizer, 
        space = Marketplace::LEN,
        seeds = [MARKETPLACE_PDA_SEED.as_ref()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}
