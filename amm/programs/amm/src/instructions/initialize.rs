use anchor_lang :: prelude::*;

use anchor_spl::{
    associated_token ::AssociatedToken,
    token::{Mint , Token ,TokenAccount},
};

#[derive(Accounts)]
#[instruction(seed : u64)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub admin :Signer<'info>,
    pub mint_x:Account<'info , Mint>,
    pub mint_y:Account<'info , Mint>,
    #[account(
        init,
        payer=admin,
        seeds = [b"lp",config.key.as_ref()],
        bump ,
        mint::decimals=6,
        mint::authority = config
    )]
    pub mint_lp : account<'info , Mint>,
    // admin_lp to mint LP tokens when users add liquidity and burn them when users withdraw
    #[account(
        init,
        payer = admin , 
        associated_token::mint = mint_x,
        associated_token::authority = config ,
    )]
    pub vault_x:Account<'info , TokenAccount>,
    #[account(
        init,
        payer = admin , 
        associated_token::mint = mint_y,
        associated_token::authority= config ,
    )]
    pub vault_y : Account<'info ,TokenAccount>,
    // here we are creating two token accounts to store two diff tokens 

    #[account(
        init, 
        payer = admin ,
        seeds = [b"config" , seed.to_le_bytes().as_ref()],
        bump,
        space = Config::INIT_SPACE,
    )]
    pub config :Account<'info ,Config>,
    // this account is to store metadata of the pool 
        //that stores the configuration and state of the liquidity pool


    pub token_program:Program<'info ,Config >,
    pub associated_token_program :Program<'info , AssociatedToken>,
    pub system_program : Program<'info , System>,

}
impl<'info> Initialize<'info>{
    pub fn initialize(&mut self , seed:u64,fee:u16,authority:option<Pubkey>,bumps:&InitializeBumps)->Result<()>{
        self.config.set_inner(Config{
            seed,
            authority,
            mint_x : self.mint_x.key(),
            mint_y : self.mint_y.key(),
            fee,
            locked:false,
            config_bump:bumps.config,
            lp_bump:bumps.mint_lp,
        });
        Ok(())
    }
}