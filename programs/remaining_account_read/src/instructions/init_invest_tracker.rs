use anchor_lang::prelude::*;
use crate::state::invest_tracker::*;
use crate::INVEST_TRACKER_SEED;

//This instruction initializes an invest tracker for the strategy

#[derive(Accounts)]
#[instruction()]
pub struct InitInvestTracker<'info> {
    #[account(
        init, 
        space = 8 + InvestTracker::INIT_SPACE,
        seeds = [
            INVEST_TRACKER_SEED.as_bytes(),
        ], 
        bump, 
        payer = signer, 
    )]
    pub invest_tracker: Account<'info, InvestTracker>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_init_invest_tracker(ctx: Context<InitInvestTracker>) -> Result<()> {
    msg!("Invest tracker initialized");
    msg!("Invest tracker account: {:?}", ctx.accounts.invest_tracker.key());
    let invest_tracker = &mut ctx.accounts.invest_tracker;
    invest_tracker.amount_invested = 0;
    Ok(())
}