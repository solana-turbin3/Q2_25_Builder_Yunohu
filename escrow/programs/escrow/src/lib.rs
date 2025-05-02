use anchor_lang::prelude::*;

declare_id!("ET34A188YdZSBgmaakpgQGdimSPZHVeRUbbjmZUpZW2U");
mod instructions ;
use instructions::*;

mod state ;


#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed :u64,receive : u64,deposit :u64) -> Result<()> {
        ctx.accounts.init_escrow(seed,recieve , &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())  
    }
    pub fn take(ctx: Context<Take>, seed :u64,receive : u64,deposit :u64) -> Result<()> {
       
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close()?;
        Ok(())  
    }
}

#[derive(Accounts)]
pub struct Initialize {}
