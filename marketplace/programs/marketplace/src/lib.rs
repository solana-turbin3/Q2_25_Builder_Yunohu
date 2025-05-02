pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub use error::MarketplaceError;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FayHNZL5NSZvP2rorHZvQrMBuyEKvEtT8FjSVdJdg36Y");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        require!(fee <= 10000, MarketplaceError::InvalidFee);
        ctx.accounts.init(name, fee, &ctx.bumps)
    }
}
