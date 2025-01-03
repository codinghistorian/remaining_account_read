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

pub fn squared_sqrt_price_return_U256(sqrt_price_x64: u128) -> U256 {
    let price = U256::from(sqrt_price_x64);
    let price_x128 = price * price;
    price_x128
}

pub fn squared_sqrt_price_scaled_return_U256(sqrt_price_x64: u128) -> U256 {
    let price = U256::from(sqrt_price_x64);
    let price_x128 = price * price;
    let scaling_factor = U256::from(1_000_000_000_000_000_000u128); // 1e18
    let price_x128_scaled = price_x128 * scaling_factor;
    price_x128_scaled
}

pub fn scaled_sqrt_price_divide_by_2_128(sqrt_price_x64: u128) -> U256 {
    let price = U256::from(sqrt_price_x64);
    let price_x128 = price * price;
    let scaling_factor = U256::from(1_000_000_000_000_000_000_000u128); // 1e21
    let mut price_x128_scaled = price_x128 * scaling_factor;
    price_x128_scaled = price_x128_scaled >> 64;
    price_x128_scaled = price_x128_scaled * scaling_factor;
    price_x128_scaled >> 64
    // Right bit shifting by n is equivalent to dividing by 2^n
    // So price_x128_scaled >> 128 is the same as price_x128_scaled / 2^128
}
//initially made it return U256, but it's totally fine to return u128
//works when the USDC is token B, basically when we want to get the price of a in the units of b
pub fn get_price_in_USDC_decimals(sqrt_price_x64: u128, a_decimals: u8, b_decimals: u8) -> 
// u128
U256 
{
    let price = U256::from(sqrt_price_x64);
    let price_x128 = price * price;
    let scaling_factor = U256::from(1_000_000_000_000_000_000_000u128); // 1e21
    let mut price_x128_scaled = price_x128 * scaling_factor;
    price_x128_scaled = price_x128_scaled >> 64;
    price_x128_scaled = price_x128_scaled * scaling_factor;
    price_x128_scaled = price_x128_scaled >> 64;

    price_x128_scaled = if a_decimals > b_decimals {
        // Left shift is equivalent to multiplying by 2^(a_decimals - b_decimals)
        price_x128_scaled * U256::from(10).pow(U256::from(a_decimals - b_decimals))
    } else if a_decimals < b_decimals {
        // Right shift is equivalent to dividing by 2^(b_decimals - a_decimals)
        price_x128_scaled / U256::from(10).pow(U256::from(b_decimals - a_decimals))
    } else {
        price_x128_scaled
    };

    //then let's scale it by b's decimal, which in this scenario is of USDC
    price_x128_scaled = price_x128_scaled * U256::from(10).pow(U256::from(b_decimals));
    price_x128_scaled = price_x128_scaled / U256::from(10).pow(U256::from(42));

    // price_x128_scaled.as_u128()
    price_x128_scaled
}

// pub fn get_inverted_price_in_USDC_decimals(
//     sqrt_price_x64: u128,
//     a_decimals: u8,
//     b_decimals: u8,
// ) -> u128 {
//     let sqrt_price = U256::from(sqrt_price_x64);

//     // Compute sqrt_price^2 (P * 2^128)
//     let price_x128 = sqrt_price * sqrt_price;

//     // Scaling factor (1e21)
//     let scaling_factor = U256::from(1_000_000_000_000_000_000_000u128);

//     // Numerator: scaling_factor * 2^64
//     let numerator = scaling_factor << 64;

//     // Inverted price calculation
//     let mut inverted_price_x128 = numerator / price_x128;

//     inverted_price_x128 = inverted_price_x128 * scaling_factor;

//     inverted_price_x128 = inverted_price_x128 << 64;

//     // Adjust for decimals (reverse multiplication and division)
//     inverted_price_x128 = if a_decimals > b_decimals {
//         inverted_price_x128 / U256::from(10).pow(U256::from(a_decimals - b_decimals))
//     } else if a_decimals < b_decimals {
//         inverted_price_x128 * U256::from(10).pow(U256::from(b_decimals - a_decimals))
//     } else {
//         inverted_price_x128
//     };

//     // Scale by token A's decimals
//     inverted_price_x128 *= U256::from(10).pow(U256::from(a_decimals));

//     // Adjust scaling factor
//     inverted_price_x128 /= U256::from(10).pow(U256::from(42));

//     inverted_price_x128.as_u128()
// }

// pub fn get_price_and_reverse_decimals(sqrt_price_x64: u128, a_decimals: u8, b_decimals: u8) -> U256 {
//     let mut price = get_price_in_USDC_decimals(sqrt_price_x64, a_decimals, b_decimals);
//     let scaling_factor = U256::from(1_000_000_000_000_000_000_000u128); // 1e21
//     price = scaling_factor / price;
//     price

//     //well I do need to adjust decimals but let's first check the absolute number
// }

//This one works for the case when USDC is token a
pub fn get_price_in_USDC_decimals_when_USDC_is_token_a(sqrt_price_x64: u128, a_decimals: u8, b_decimals: u8) -> 
// u128
U256 
{
    let price = U256::from(sqrt_price_x64);
    let price_x128 = price * price;
    let scaling_factor = U256::from(1_000_000_000_000_000_000_000u128); // 1e21
    let mut price_x128_scaled = price_x128 * scaling_factor;
    price_x128_scaled = price_x128_scaled >> 64;
    price_x128_scaled = price_x128_scaled * scaling_factor;
    price_x128_scaled = price_x128_scaled >> 64;

    price_x128_scaled = if a_decimals > b_decimals {
        // Left shift is equivalent to multiplying by 2^(a_decimals - b_decimals)
        price_x128_scaled * U256::from(10).pow(U256::from(a_decimals - b_decimals))
    } else if a_decimals < b_decimals {
        // Right shift is equivalent to dividing by 2^(b_decimals - a_decimals)
        price_x128_scaled / U256::from(10).pow(U256::from(b_decimals - a_decimals))
    } else {
        price_x128_scaled
    };

    //then let's scale it by b's decimal, which in this scenario is of USDC
    price_x128_scaled = price_x128_scaled * U256::from(10).pow(U256::from(a_decimals));
    price_x128_scaled = price_x128_scaled / U256::from(10).pow(U256::from(42));

    // price_x128_scaled.as_u128()
    price_x128_scaled
}
//This one works for the case when USDC is token a
pub fn get_price_and_reverse_decimals_when_USDC_is_token_a(sqrt_price_x64: u128, a_decimals: u8, b_decimals: u8) -> U256 {
    let mut price = get_price_in_USDC_decimals_when_USDC_is_token_a(sqrt_price_x64, a_decimals, b_decimals);
    let scaling_factor = U256::from(1_000_000_000_000_000_000_000u128); // 1e21
    // price = scaling_factor / price / U256::from(10).pow(U256::from(9));
    price = scaling_factor / price / U256::from(10).pow(U256::from(21 - 2 * a_decimals));
    //basically, when USDC is token a, divide he result by USDc's decimal,
    //if not USDC, divide by 10^(21 - 2 * a_decimals), then the price would make sense

    price

    //well I do need to adjust decimals but let's first check the absolute number
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
        // Multiply by 10^(decimals_a - decimals_b) for sqrt_price_x64
        raw_price * 10u128.pow((decimals_a - decimals_b) as u32)
    } else {
        // Divide by 10^(decimals_b - decimals_a) for sqrt_price_x64
        raw_price / 10u128.pow((decimals_b - decimals_a) as u32)
    }
}