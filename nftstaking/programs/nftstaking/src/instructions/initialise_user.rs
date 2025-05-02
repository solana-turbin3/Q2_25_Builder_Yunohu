use anchor_lang::prelude::*;
use crate::state::UserAccount;

#[derive(Accounts)]
pub struct InitialiseUser<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    // we are creating a new user account
    #[account(
        init , 
        payer = user , 
        seeds = [b"user" , user.key().as_ref()],
        bump ,
        space = UserAccount::INIT_SPACE,

    )]
    pub user_account : Account<'info , UserAccount>,
     
    pub system_program : Program<'info , System >,
}

impl<'info> InitialiseUser<'info>{
    pub fn initialise_user(&mut self , user_bump : u8) -> Result<()>{
        self.user_account.set_inner(UserAccount{
            points_per_stake: 0,
            amount_staked: 0,
            bump: bumps.user_account,
        });
        // setting all the values to 0 
        Ok(())

        
    }
}
