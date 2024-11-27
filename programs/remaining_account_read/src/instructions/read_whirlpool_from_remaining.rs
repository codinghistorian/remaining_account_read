use anchor_lang::prelude::*;
use borsh::BorshDeserialize;
use crate::state::whirlpool::*;
use crate::state::mint::*;
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
    let mint_a_data = ctx.remaining_accounts[1].data.borrow();
    let mint_b_data = ctx.remaining_accounts[2].data.borrow();

    let mint_a = Mint::try_from_slice(&mint_a_data[..])?;
    let mint_b = Mint::try_from_slice(&mint_b_data[..])?;
    msg!("Mint A decimals: {}", mint_a.decimals);
    msg!("Mint B decimals: {}", mint_b.decimals);

    msg!("Whirlpool config: {:?}", whirlpool.whirlpools_config);
    msg!("Whirlpool bump: {:?}", whirlpool.whirlpool_bump);
    msg!("Tick spacing: {}", whirlpool.tick_spacing);
    msg!("Fee rate: {}", whirlpool.fee_rate);
    msg!("Protocol fee rate: {}", whirlpool.protocol_fee_rate);
    msg!("Liquidity: {}", whirlpool.liquidity);
    msg!("Sqrt price: {}", whirlpool.sqrt_price);
    // msg!("Price: {}", get_price_from_sqrt_price(whirlpool.sqrt_price));
    // msg!("Price test: {}", test_get_price_from_sqrt_price(whirlpool.sqrt_price));
    // msg!("Squared price: {}", squared_sqrt_price(whirlpool.sqrt_price));
    // msg!("Squared price U256: {:?}", squared_sqrt_price_return_U256(whirlpool.sqrt_price));
    // msg!("Squared price scaled U256: {:?}", squared_sqrt_price_scaled_return_U256(whirlpool.sqrt_price));
    // msg!("Scaled sqrt price divide by 2^128: {:?}", scaled_sqrt_price_divide_by_2_128(whirlpool.sqrt_price));
    let sqrt_price = 11221602387266021796 as u128;
    // msg!("Inverted price in USDC decimals when USDC is token a: {}", get_price_in_USDC_decimals_when_USDC_is_token_a(sqrt_price, 8, 6));
    // msg!("Price in USDC decimals when reversed when USDC is token a: {:?}", get_price_and_reverse_decimals_when_USDC_is_token_a(sqrt_price, 8, 6));
    msg!("USDC decimals when USDC is token b: {}", get_price_in_USDC_decimals(sqrt_price, 6, 9));

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