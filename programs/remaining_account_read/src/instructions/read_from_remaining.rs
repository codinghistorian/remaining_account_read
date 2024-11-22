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
    let data = invest_tracker_account.try_borrow_data()?;
    msg!("invest_tracker_account: {:?}", invest_tracker_account.key());
    msg!("invest_tracker_account data length: {}", data.len());
    msg!("data: {:?}", data);
    
    // Skip the 8-byte discriminator and deserialize
    // let invest_tracker_account_data = InvestTracker::try_deserialize(&mut &data[8..])?;
    let invest_tracker_account_data = InvestTracker::try_from_slice(&data[8..])?;

    msg!("invest_tracker_account_data: {:?}", invest_tracker_account_data);
    msg!("amount_invested: {}", invest_tracker_account_data.amount_invested);
    Ok(())
}