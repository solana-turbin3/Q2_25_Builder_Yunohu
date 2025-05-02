use crate::state::{marketplace::Marketplace, listing::Listing};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct List<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,
    // this is for the seller & signature 
    #[account(
        seeds = [b"marketplace",marketplace.name.as_str().as_bytes()],
        bump ,
    )]
    pub marketplace:Account<'info,Marketplace>,
    // if it contains init then we are initialising , here we are just accessing the created pda from initializ.rs

    pub maker_mint:InterfaceAccount<'info,Mint>,
     //  mint address of the nft which will be listed 

    #[account(
        mut ,
        associated_token::mint = maker_mint,
        associated_token::authority = maker ,
        associated_token::token_program = token_program,
    )]
    pub maker_ata:InterfaceAccount<'info , TokenAccount>,
    // maker token account where the nft will be transferred from 

    #[account(
        init , 
        payer = maker ,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program=token_program,
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,
    // the vault aka ata where the nft will be stored after listing 

    #[account(
        init , 
        payer = maker ,
        seeds = [marketplace.key().as_ref(),maker_mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE,
    )]
    pub listing:Account<'info,Listing>,
    // pda from marketplace , account is created for listing to store the information of the listing 

    pub collection_mint : InterfaceAccount<'info , Mint>,
    // this is used to check the listed nft mint is authentic or not 
        
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key == collection_mint.key(),
        constraint = metadata.collection.as_ref().unwrap().verified == true 
    )]
    pub metadata :Account<'info,MetadataAccount>,
    // meta data

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition:Account<'info, MasterEditionAccount>,
    // this checks the master edition with the listed edition to check if its original or not 

    pub metadata_program: Program<'info, Metadata>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub system_program: Program<'info, System>,
    
    pub token_program: Interface<'info, TokenInterface>,

    // we have to write this everywhere because each accounts have to communicate with th ext. programs 
}

impl<'info> List <'info>{
    pub fn create_listing(&mut self , price :u64 , bumps : &ListBumps)-> Result<()>{
        self.listing.set_inner(Listing{
            maker :self.maker.key(),
            maker_mint:self.maker_mint.key(), 
            price , 
            bump:bumps.listing
        });
       
        Ok(())
    }
    // creates new listing  by using listing account , listing first will be created by maker itself 

    pub fn deposit_nft(&mut self)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from : self.maker_ata.to_account_info(),
            mint:self.maker_mint.to_account_info(),
            to : self.vault.to_account_info(),
            authority :self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);

        transfer_checked(cpi_ctx , 1, self.maker_mint.decimals)
        // deposit_nft
    }
}