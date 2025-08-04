#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"config", admin.key().as_ref() ],
        bump,
        space = 2 + StakeConfig::INIT_SPACE
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::authority = config,
        mint::decimals = 6
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init_config(
        &mut self,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
        bump: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bump.rewards_mint,
            bump: bump.config,
        });
        Ok(())
    }
}
