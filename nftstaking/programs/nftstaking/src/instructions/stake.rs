
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_delegate::instructions::{
            FreezeDelegatedAccountCpi  , FreezeDelegatedAccountCpiAccounts,

        },
        MasterEditionAccount,Metadata , MetadataAccount,
    },
    token::{approve ,Approve, Mint , Token , TokenAccount},         
};
use Error::StakeError;

#[derive(Accounts)]
pub struct Stake<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub collection_mint : Account<'info, Mint>,
    // the user is the owner of the nft

    // we are creating a ata account for the user
    #[account(
         mut , 
         associated_token::mint = mint ,
         associated_token::authority = user,

    )]
    pub mint_ata : Account<'info, TokenAccount>,

    // we are creating metadata account 
    #[account(
        seeds =[b"metadata" , 
        metadata_program.key().as_ref() ,
        mint.key().as_ref()],
        bump , 
        seeds::program = metadata_program.key(),
        //checking if the nft belongs to the collection it belongs to 
        constraint = metadata.collection.as_ref().unwrap().key == collection_mint.key() ,
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata : Account<'info , MetadataAccount>,


    // checking th nft with the ,aster edition pda 
    #[account(
        seeds =[b"metadata" , 
        metadata_program.key().as_ref() ,
        mint.key().as_ref(),
        b"edition"],
        bump,
        seeds::program = metadata_program.key(),//ensures that the PDA for these accounts is derived using the Token Metadata Program’s program ID
    )]
    pub master_edition : Account<'info , MasterEditionAccount>,


    // creating new account to track the nft staked !!
    #[account(
        
        init,
        payer = user ,
        seeds = [b"stake" , config.key().as_ref(), mint.key().as_ref()],
        bump , 
        space = 8+ StakeAccount::INIT_SPACE,
    )]
    pub stake_account : Account<'info , StakeAccount>,

    #[account(
        mut , 
        seeds = [b"user",user.key().as_ref()],
        bump = user_account.bump ,
    )]
    pub user_account : Account<'info , UserAccount >,

    #[account(
        seeds = [b"config"],
        bump = config.bump ,
    )]
    pub config : Account<'info , StakeConfig>,

    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,


}

impl<'info>Stake<'info>{
    pub fn stake(&mut self , bumps :&StakeBumps) -> Result<()>{
        require!(
            self.user_account.amount_staked <= self.config.max_stake,
            StakeError::MaxStakeReached,
        );
        // checking whether the amount is less than max amount

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts =Approve {
            to : self.mint_ata.to_account_info(),
            delegate : self.stake_account.to_account_info(),
            // The stake_account is set as the delegate so it can freeze the NFT
            authority : self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program , cpi_accounts);

        approve(cpi_ctx , 1)?;

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        // WE ARE ACCESING THE DATA TO DO CPI

            let seeds = [
            b"stake",
            self.config.key().as_ref(),
            self.mint.key().as_ref(),
            &[bumps.stake_account],
            
        ];
        let signer_seeds : &[&[&[u8]]] = &[&seeds[..]];

        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts{
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            //here we are calling the program to freeze the nft 
            },
        )
        .invoke_signed(signer_seeds)?;

        self.stake_account.set_inner(StakeAccount{
            owner : self.user.key(),
            mint :self.mint.key(),
            staked_at:Clock::get()?.unix_timestamp,
            bump : bumps.stake_account,
        });
        ///here we are actually implementing the staking mechanism




        Ok(())
    }
}