use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod constants;
pub mod error;
pub mod utils;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use error::*;
pub use utils::*;
declare_id!("AGJTaqNYeuFnT5P5VBej8cFtwzkX9HD1W2SiYRD2sRY1");

#[program]
pub mod remaining_account_read {
    use super::*;

    pub fn init_invest_tracker(ctx: Context<InitInvestTracker>) -> Result<()> {
        msg!("Initializing invest tracker");
        handle_init_invest_tracker(ctx)?;
        Ok(())
    }

    pub fn read_from_remaining(ctx: Context<ReadFromRemaining>) -> Result<()> {
        msg!("Reading from remaining account");
        handle_read_from_remaining(ctx)?;
        Ok(())
    }

    pub fn read_whirlpool_price(ctx: Context<ReadWhirlpoolPrice>) -> Result<()> {
        msg!("Reading whirlpool price");
        handle_read_whirlpool_price(ctx)?;
        Ok(())
    }
}
