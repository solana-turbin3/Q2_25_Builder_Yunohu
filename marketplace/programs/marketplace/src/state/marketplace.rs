use anchor_lang::prelude::*;
/// # Marketplace
/// The central state account that stores configuration for the marketplace
/// This account is a Program Derived Address (PDA) created at initialization
/// and holds information about fees, admin access, and naming

#[account]

pub struct Marketplace {
    pub admin : Pubkey,
    pub fee : u16,
    pub bump : u8 ,
    pub treasury_bump : u8 ,
    pub rewards_bumps : u8,
    pub name : String ,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 +32+2+3*1+(4+32);
}

