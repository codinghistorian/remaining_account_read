use anchor_lang::prelude::*;
use borsh::BorshDeserialize;
use crate::state::whirlpool::*;
use crate::utils::get_price_from_sqrt_price::*;
#[derive(Accounts)]
#[instruction()]
pub struct ReadWhirlpoolPrice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handle_read_whirlpool_price(ctx: Context<ReadWhirlpoolPrice>) -> Result<()> {
    msg!("Reading whirlpool price");
    
    // Get whirlpool from remaining account
    let whirlpool_account = &ctx.remaining_accounts[0];
    let account_data = whirlpool_account.data.borrow();
    let whirlpool = Whirlpool::try_from_slice(&account_data[8..])?;

    msg!("Whirlpool config: {:?}", whirlpool.whirlpools_config);
    msg!("Whirlpool bump: {:?}", whirlpool.whirlpool_bump);
    msg!("Tick spacing: {}", whirlpool.tick_spacing);
    msg!("Fee rate: {}", whirlpool.fee_rate);
    msg!("Protocol fee rate: {}", whirlpool.protocol_fee_rate);
    msg!("Liquidity: {}", whirlpool.liquidity);
    msg!("Sqrt price: {}", whirlpool.sqrt_price);
    msg!("Price: {}", get_price_from_sqrt_price(whirlpool.sqrt_price));
    msg!("Price test: {}", test_get_price_from_sqrt_price(whirlpool.sqrt_price));
    msg!("Squared price: {}", squared_sqrt_price(whirlpool.sqrt_price));
    msg!("Current tick index: {}", whirlpool.tick_current_index);
    msg!("Protocol fee owed A: {}", whirlpool.protocol_fee_owed_a);
    msg!("Protocol fee owed B: {}", whirlpool.protocol_fee_owed_b);
    msg!("Token mint A: {:?}", whirlpool.token_mint_a);
    msg!("Token vault A: {:?}", whirlpool.token_vault_a);
    msg!("Fee growth global A: {}", whirlpool.fee_growth_global_a);
    msg!("Token mint B: {:?}", whirlpool.token_mint_b);
    msg!("Token vault B: {:?}", whirlpool.token_vault_b);
    msg!("Fee growth global B: {}", whirlpool.fee_growth_global_b);
    msg!("Reward last updated timestamp: {}", whirlpool.reward_last_updated_timestamp);
    msg!("Reward infos: {:?}", whirlpool.reward_infos);

    Ok(())
}