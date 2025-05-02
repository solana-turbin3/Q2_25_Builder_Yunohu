use anchor_lang::prelude::*;

declare_id!("EH95Bz326btkhuzw6JWAPWhLTmxDF2jArTbRLiPDKCgS");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
