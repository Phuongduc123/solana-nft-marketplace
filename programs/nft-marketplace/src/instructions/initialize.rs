

use crate::errors::VestingError;

use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, TokenAccount};
use crate::{VESTING_PDA_SEED, LOCKER_PDA_SEED};


use crate::state::*;

pub fn initialize(
    ctx: Context<Initialize>,
    round: String,
    schedule: Vec<i64>,
    percents: Vec<u64>,
    denominator: u64
) -> Result<()> {
    require_neq!(schedule.len(), 0, VestingError::InvalidInput);
    require_eq!(schedule.len(), percents.len(), VestingError::InvalidInput);
    require_gte!(128, round.chars().count(), VestingError::InvalidInput);

    let initializer = &ctx.accounts.initializer;
    let vesting = &mut ctx.accounts.vesting_account;
        
    vesting.schedule = schedule;
    vesting.percents = percents;
    vesting.denominator = denominator;
    vesting.authorizer = initializer.key();
    vesting.round = round;
    vesting.mint_of_vesting_token = ctx.accounts.mint_of_vesting_token.key();

    Ok(())
}

#[derive(Accounts)]
#[instruction(round: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_of_vesting_token: Account<'info, Mint>,

    #[account(
        init, 
        payer=initializer,
        seeds=[LOCKER_PDA_SEED.as_ref(), mint_of_vesting_token.key().as_ref(), round.as_ref()],
        bump,
        token::mint=mint_of_vesting_token,
        token::authority=vesting_account,
    )]
    pub locker: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer=initializer, 
        space = Vesting::LEN,
        seeds = [VESTING_PDA_SEED.as_ref(), mint_of_vesting_token.key().as_ref(), round.as_ref()],
        bump
    )]
    pub vesting_account: Account<'info, Vesting>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}
