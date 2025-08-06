#![allow(deprecated)]
#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BQZDn8p91Ci45wFVT3gSLqqdvp3VMeL3GmYjLd1JBiTy");

#[program]
pub mod staking {
    use super::*;

    pub fn init_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32, bump: u8) -> Result<()> {
        Ok(())
    }
}
