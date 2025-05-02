use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed : u64,
    pub maker : pubkey,
    pub mint_a : pubkey,
    pub mint_b : pubkey ,
    pub receive : u64 ,
    pub bump : u8,
}