use crate::state::{marketplace::Marketplace, listing::Listing};
use crate::error::MarketplaceError;
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        MasterEditionAccount, Metadata,
        MetadataAccount,
    },
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct Delist<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    // marketplace account accessing 

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut ,
        associated_token::mint = maker_mint,
        associated_token::authority = maker ,
        associated_token::token_program=token_program,
    )]
    pub maker_ata : InterfaceAccount<'info , TokenAccount>,
    // maker ata where the nft will be transferred to 

    #[account(
        mut ,
        associated_token::mint=maker_mint,
        associated_token::authority=listing ,
        associated_token::token_program = token_program,
    )]
    pub vault : InterfaceAccount<'info , TokenAccount>,
    // the vault token account thats holding nft 

    #[account(
        mut,
        seeds = [marketplace.key().as_ref(),maker_mint.key().as_ref()],
        bump = listing.bump,
        constraint = listing.maker == maker.key(),
        close = maker 
    )]
    pub listing : Account<'info ,Listing>,
    // the listed account , checking whether the owner is the maker and closing the account and get back rent

    pub collection_mint : InterfaceAccount<'info,Mint>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), maker_mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), maker_mint.key().as_ref(), b"edition"],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    // master edition of the nft for reference and all 

    pub metadata_program : Program<'info , Metadata>,

    pub associated_token_program:Program<'info, AssociatedToken>,

    pub system_program : Program<'info , System >,

    pub token_program : Interface<'info , TokenInterface>,
}

impl<'info> Delist<'info>{
    pub fn withdraw_nft(&mut self )-> Result<()>{
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            mint : self.maker_mint.to_account_info(),
            to : self.maker_ata.to_account_info(),
            authority:self.listing.to_account_info(),
        };

        //defining cpi program and accounts


        let marketplace_key = self.marketplace.key();
        let maker_mint_key = self.maker_mint.key();

        //storing pdas in vars

        let seeds = &[
            marketplace_key.as_ref(),
            maker_mint_key.as_ref(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        // signing the cpi

        let cpi_ctx = CpiContext::new_with_signer(cpi_program ,cpi_accounts , signer_seeds);
        transfer_checked(cpi_ctx ,1, self.maker_mint.decimals)?;
        // transfer and check 

        Ok(())
    }

    pub fn execute_delist(&mut self)->Result<()>{
        self.withdraw_nft()?;
        Ok(())
        // executeeeeeeee
    }
}