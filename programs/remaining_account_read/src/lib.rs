use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod constants;
pub mod error;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use error::*;
declare_id!("516r1c3Sn6dk9kNMy7tSb82JNzSAzU25vBTZ2UN6QKts");

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
}