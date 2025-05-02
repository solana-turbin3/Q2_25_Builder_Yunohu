use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct InitialiseConfig<'info>{
    #[account(mut)]
    pub admin: Signer <'info>,
    // creating a new config account
    #[account(
        init , 
        payer = admin ,
        seeds = [b"config"],
        bump,
        space = 8 + StakeConfig::INIT_SPACE , 
    )]
    pub config : Account<'info , StakeConfig>,

    // creating a new mint account for rewards

    #[account(
        init , 
        payer = admin , 
        seeds = [b"rewards"],
        bump , 
        mint::decimals = 6,
        mint::authority = config , 
    )]
    pub reward_mint  : Account<'info , Mint>,
    pub token_program : Program<'info , Token>,

    pub system_program : Program<'info , System >,

}

impl<'info>InitialiseConfig<'info>{
    pub fn initialise_config(&mut self ,points_per_stake:u8 , max_stake:u8 , freeze_period: u32 , bumps : &InitialiseConfigBumps )
    -> Result<()>{
        self.config.set_inner(StakeConfig{
            points_per_stake,
            max_stake,
            freeze_period,
            bumps, 

        });
        ok(())
    }

}