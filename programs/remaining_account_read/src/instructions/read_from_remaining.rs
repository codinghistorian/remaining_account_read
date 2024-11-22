use anchor_lang::prelude::*;
//borsh?
use crate::state::invest_tracker::*;
use crate::INVEST_TRACKER_SEED;

//This instruction initializes an invest tracker for the strategy

#[derive(Accounts)]
#[instruction()]
pub struct ReadFromRemaining<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handle_read_from_remaining(ctx: Context<ReadFromRemaining>) -> Result<()> {
    msg!("Reading from remaining account");
    let invest_tracker_account = &ctx.remaining_accounts[0];
    
    // Verify the account is writable
    if !invest_tracker_account.is_writable {
        return Err(ProgramError::InvalidAccountData.into());
    }

    // Read and update in a single mutable borrow
    let mut account_data = invest_tracker_account.try_borrow_mut_data()?;
    
    // First read the current data
    let current_data = InvestTracker::try_from_slice(&account_data[8..])?;
    msg!("Current amount invested: {}", current_data.amount_invested);

    // Create and serialize updated data
    let mut updated_data = current_data;
    updated_data.amount_invested = 11;
    let serialized = updated_data.try_to_vec()?;

    // Write the updated data
    account_data[8..].copy_from_slice(&serialized);
    
    // We can verify by deserializing the same account_data
    let final_data = InvestTracker::try_from_slice(&account_data[8..])?;
    msg!("Updated amount invested: {}", final_data.amount_invested);

    Ok(())
}