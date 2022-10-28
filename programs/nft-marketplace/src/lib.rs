use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;


pub const MARKETPLACE_PDA_SEED: &[u8] = b"marketplace";

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, signer: Pubkey) -> Result<()> {
        instructions::initialize::initialize(ctx, signer)
    }
}
