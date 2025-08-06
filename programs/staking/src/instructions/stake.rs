#![allow(unexpected_cfgs)]
use anchor_lang::{prelude::*};
use anchor_spl::{metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, Metadata, MetadataAccount}, token::{Mint, Token, TokenAccount}, token_2022::{approve, Approve}};

use crate::state::{StakeAccount, UserAccount, StakeConfig};

use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata_account.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata_account.collection.as_ref().unwrap().verified == true
    )]
    pub metadata_account: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init,
        payer = user,
        seeds = [
            b"stake",
            mint.key().as_ref(),
            config.key().as_ref()
        ],
        bump,
        space = 8 + StakeAccount::INIT_SPACE
    )]
    pub stake_account : Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>
}

impl <'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        require!(self.user_account.amount_staked < self.config.max_stake,
        ErrorCode::CustomError);

        self.stake_account.set_inner(StakeAccount
            {
                owner: self.user.key(),
                mint: self.mint.key(),
                stake_at: Clock::get()?.unix_timestamp, 
                bump: bumps.stake_account
            });
        
        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = Approve {
            to: self.user_mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info()
        };
        
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        approve(cpi_ctx, 1)?;
        
        let mint_key = self.mint.to_account_info().key();
        let config_key = self.config.to_account_info().key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"stake",
            mint_key.as_ref(),
            config_key.as_ref(),
            &[self.stake_account.bump]
        ]];
        
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.user_mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program
            },
        )
        .invoke_signed(signer_seeds)?;
        
        self.user_account.amount_staked += 1 ;

        Ok(())
    }
}
