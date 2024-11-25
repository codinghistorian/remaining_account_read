use uint::construct_uint;

// Define a 256-bit unsigned integer type
construct_uint! {
    pub struct U256(4);
}

/// Calculates the price B/A from the sqrt_price_x64 using integer arithmetic.
///
/// # Arguments
///
/// * `sqrt_price_x64` - The sqrt price in Q64.64 fixed-point format.
///
/// # Returns
///
/// * `u128` representing the price B/A scaled by 1e18.
pub fn get_price_from_sqrt_price(sqrt_price_x64: u128) -> u128 {
    // Convert sqrt_price_x64 to U256 for 256-bit arithmetic
    let sqrt_price = U256::from(sqrt_price_x64);

    // Compute sqrt_price^2
    let price_x128 = sqrt_price * sqrt_price;

    // Multiply by scaling factor to preserve precision
    let scaling_factor = U256::from(1_000_000_000_000_000_000u128); // 1e18
    let price_x128_scaled = price_x128 * scaling_factor;

    // Shift right by 128 bits to divide by 2^128
    let price = price_x128_scaled >> 128;

    // Return the lower 128 bits as u128
    price.as_u128()
}

pub fn test_get_price_from_sqrt_price(sqrt_price_x64: u128) -> u128 {
    let price = U256::from(sqrt_price_x64);
    price.as_u128()
}

pub fn squared_sqrt_price(sqrt_price_x64: u128) -> u128 {
    let price = U256::from(sqrt_price_x64);
    let price_x128 = price * price;
    price_x128.as_u128()
}

/// Adjusts the raw price based on the token decimals.
///
/// # Arguments
///
/// * `raw_price` - The raw B/A price as u128.
/// * `decimals_a` - Decimal places of token A.
/// * `decimals_b` - Decimal places of token B.
///
/// # Returns
///
/// * `u128` representing the adjusted price B/A.
pub fn get_adjusted_price(
    raw_price: u128,
    decimals_a: u8,
    decimals_b: u8,
) -> u128 {
    if decimals_a >= decimals_b {
        // Divide by 10^(decimals_a - decimals_b)
        raw_price / 10u128.pow((decimals_a - decimals_b) as u32)
    } else {
        // Multiply by 10^(decimals_b - decimals_a)
        raw_price * 10u128.pow((decimals_b - decimals_a) as u32)
    }
}