use crate::state::{marketplace::Marketplace, listing::Listing};
use crate::error::MarketplaceError;
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct purchase<'info>{
    #[account(mut)]
    pub taker : Signer<'info>,
    // the taker signs the tx buys the nft 
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    //system account accessing

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    // marketplace account accessing 

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,
    // treasury account accessing 

    pub maker_mint: InterfaceAccount<'info, Mint>,
    // getting the mint address of the buying nft ig 
    
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = reward_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_rewards_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program,
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut , 
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    
    )]
    pub listing: Account<'info,Listing>,

    // the mint address of the collection this nft belongs to 
    pub collection_mint : InterfaceAccount<'info , Mint>,

    pub reward_mint : InterfaceAccount<'info , Mint>,

    // to create associated token accounts 
    pub associated_token_program: Program<'info , AssociatedToken>,

    // required for the sol transfer 
    pub system_program : Program<'info , System>,

    // required for operations between the tokens 
    pub token_program : Interface<'info , TokenInterface>,
}

impl<'info>purchase<'info>{

    pub fn send_sol(&mut self)->Result<()>{
         //checking underflow and overflow
        let marketplace_fee : u64 = (self.marketplace.fee as u64)
            .checked_mul(self.listing.price)
            .ok_or(MarketplaceError::Overflow)?
            .checked_div(10000_u64)
            .ok_or(MarketplaceError::Underflow)?;

        // we are setting up cpi for the price and fee and transfer here 
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from : self.taker.to_account_info(),
            to : self.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program , cpi_accounts);

        // here we are subtracting the price minus fee 
        let amount : u64 = self.listing.price 
            .checked_sub(marketplace_fee)
            .ok_or(MarketplaceError::Underflow)?;
            

        // transfering sol to seller 
        transfer(cpi_ctx , amount)?;

        // setting up transfer of fee to treasury 
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from : self.taker.to_account_info(),
            to : self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx , marketplace_fee)?;


        Ok(())
    }


    pub fn transfer_nft(&mut self)->Result<()>{
        // setting up the cpi , like transfer and all
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from : self.vault.to_account_info(),
            mint : self.maker_mint.to_account_info(),
            to : self.taker_ata.to_account_info(),
            authority : self.listing.to_account_info(),
        };
        // to make transfer we need to sign , so we are using seeds here to sign 
        let marketplace_key = self.marketplace.key();
        let maker_mint_key = self.maker_mint.key();
        let seeds = &[
            marketplace_key.as_ref(),
            maker_mint_key.as_ref(),
            &[self.listing.bump],
        ];
        // checking and transfering 
        // why seeds ? The listing PDA is the authority for the vault, but PDAs can’t sign transactions directly.
        //The seeds allow the program to “sign” on behalf of the PDA, proving it’s the correct authority.
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts , signer_seeds);
        transfer_checked(cpi_ctx,1,self.maker_mint.decimals)?;


        Ok(())
    }


    pub fn execute_purchase(&mut self )->Result<()>{

        // the code we wrote above just the function , here we are executing 
        self.send_sol()?;

        self.transfer_nft()?;

        Ok(())
    }

}