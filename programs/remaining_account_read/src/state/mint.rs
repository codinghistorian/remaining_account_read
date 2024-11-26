use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::whirlpool_reward_info::*;
use crate::constants::*;
#[account]
#[derive(Copy, Debug, PartialEq)]
pub struct Mint {
    /// Optional authority used to mint new tokens.
    /// First 4 bytes are discriminator (0 for None, 1 for Some)
    /// Next 32 bytes are the pubkey if Some
    pub mint_authority: [u8; 36],  // 4 byte tag + 32 byte pubkey
    /// Total supply of tokens.
    pub supply: u64,               // 8 bytes
    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,              // 1 byte
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,      // 1 byte
    /// Optional authority to freeze token accounts.
    /// First 4 bytes are discriminator (0 for None, 1 for Some)
    /// Next 32 bytes are the pubkey if Some
    pub freeze_authority: [u8; 36], // 4 byte tag + 32 byte pubkey
}

