use crate::error::MarketplaceError;
use crate::state::marketplace::Marketplace;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

#[derive(Accounts)]
#[instruction(name:String)]

pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    //this is admin
    #[account(
        init,
        payer = admin ,
        seeds = [b"marketplace",name.as_str().as_bytes()],
        bump,
        space = Marketplace::INIT_SPACE,
    )]
    pub marketplace: Account<'info, Marketplace>,
    // here we are creating a account for marketplace
    #[account(
        seeds = [b"treasury",marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    // here we are creating a treasury account the fees will be stored here
    #[account(
        init,
        payer= admin ,
        seeds= [b"rewards" , marketplace.key().as_ref() ],
        bump ,
        mint::decimals =6,
        mint::authority=marketplace,
        mint::token_program = token_program
       )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    // this is created to give rewards to users -  can be excluded if not needed
    pub system_program: Program<'info, System>,

    // mandatory system program and token program initialisation
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(fee <= 10000, MarketplaceError::InvalidFee);
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_bumps: bumps.reward_mint,
            name,
        });

        Ok(())
    }
}
