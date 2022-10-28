use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub authorizer: Pubkey,
    pub mint_of_nft: Pubkey,
    pub total_minted: u64,
    pub receiver: Pubkey,
    pub signer: Pubkey
}

const STRING_LENGTH_PREFIX: usize = 4;
const DISCRIMINATOR_LENGTH: usize = 8;

const AUTHORIZER_KEY_LENGTH: usize = 32;
const MINT_OF_NFT_LENGTH: usize = 32;
const TOTAL_MINTED_LENGTH: usize = 8;
const RECEIVER_LENGTH: usize = 32;
const SIGNER_LENGTH: usize = 32;


impl Marketplace {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + AUTHORIZER_KEY_LENGTH
        + STRING_LENGTH_PREFIX 
        + MINT_OF_NFT_LENGTH
        + TOTAL_MINTED_LENGTH
        + RECEIVER_LENGTH
        + SIGNER_LENGTH;
}