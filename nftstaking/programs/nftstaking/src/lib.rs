pub mod constant;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use states::*;

declare_id!("Bm12Zfyy1mKXDmJPHm9ArrrkjjeVSXhRokrJgjAca8Vh");

#[program]
pub mod nftstaking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx);
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
