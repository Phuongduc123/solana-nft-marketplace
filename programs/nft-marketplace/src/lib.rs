use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
