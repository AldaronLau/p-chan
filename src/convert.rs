//! Low-level float / integer channel conversions

use crate::math::{self, Signed, Unsigned};

#[inline(always)]
const fn add_sign_word(float: f32, sign: i32) -> f32 {
    f32::from_bits(float.to_bits() | (Unsigned(sign).reinterpret() << 31))
}

/// Convert non-zero [`u32`] fraction to [`f32`] (ranged 0 to 1).
#[inline(always)]
const fn nonzero_u32_to_f32(fraction: u32) -> f32 {
    // Calculate leading zeros (with inferred 1)
    let leading_zeros = fraction.leading_zeros() + 1;
    // Remove leading zeros and inferred 1 to subtract from exponent
    let fraction = fraction.wrapping_shl(leading_zeros);
    // Shift right to truncate to 23-bit fraction
    let fraction = fraction >> 9;
    // Calculate -127 bias exponent
    let exponent = (127 - leading_zeros) << 23;

    // Scale up (u32 max is 2³² - 1, and we want 2³²)
    f32::from_bits(exponent | fraction)
        * f32::from_bits(0b111111100000000000000000000001)
}

/// Convert normal [`f32`] (ranged 0 to 1) to [`u32`] fraction.
#[inline(always)]
const fn normal_f32_to_u32(float: f32) -> u32 {
    // Scale down (f32 max fraction is 2³², and we want 2³² - 1)
    let float =
        (float * f32::from_bits(0b111111011111111111111111111111)).to_bits();
    // Convert fraction to 23 bits
    let fraction = (float << 9) >> 1;
    // Artificially extend fraction precision, and add inferred 1
    let fraction = (1 << 31) | fraction | (fraction >> 23);
    // Extract -127 bias 8-bit negative exponent
    let exponent =
        Unsigned(127 - Signed(float >> 23).reinterpret()).reinterpret();
    // Scale by exponent
    let (fraction, overflow) = fraction.overflowing_shr(exponent - 1);
    // Check if fraction should be 0 or not
    let nonzero = Unsigned(-math::word(!overflow)).reinterpret();

    // Make zero if zero, otherwise no-op
    fraction & nonzero
}

/// Convert normal [`f32`] (ranged -1 to 1) to [`i32`] fraction.
#[inline(always)]
const fn normal_f32_to_i32(float: f32) -> i32 {
    // Convert to unsigned integer and reduce precision
    let magnitude = Signed(normal_f32_to_u32(float.abs()) >> 1).reinterpret();
    // Get offset
    let offset = -math::word(float.is_sign_negative());
    // Get sign
    let sign = (offset * 2) + 1;

    // Construct fraction with sign, magnitude, and offset
    offset + (magnitude * sign)
}

/// Convert [`u32`] fraction to [`f32`] (ranged 0 to 1).
pub const fn u32_to_f32(fraction: u32) -> f32 {
    // Check if fraction is 0 or not
    let nonzero = Unsigned(-math::word(fraction != 0)).reinterpret();

    // Make zero if zero, otherwise no-op
    f32::from_bits(nonzero_u32_to_f32(fraction).to_bits() & nonzero)
}

/// Convert [`i32`] fraction to [`f32`] (ranged -1 to 1).
pub const fn i32_to_f32(int: i32) -> f32 {
    // Split sign and magnitude from signed integer
    let sign = -math::word(int < 0);
    let uint = int.abs_diff(sign);
    // Scale up unsigned integer to full range (without true zero)
    let uint = (uint * 2) + 1;

    // Copy sign back into converted float
    add_sign_word(nonzero_u32_to_f32(uint), sign)
}

/// Convert [`f32`] (ranged 0 to 1) to [`u32`] fraction.
#[inline(always)]
pub const fn f32_to_u32(float: f32) -> u32 {
    // Normalize and clamp from 0 to 1
    let float = math::normalize_f32(float).clamp(0.0, 1.0);

    // Convert to unsigned integer
    normal_f32_to_u32(float)
}

/// Convert [`f32`] (ranged -1 to 1) to [`i32`] fraction.
#[inline(always)]
pub const fn f32_to_i32(float: f32) -> i32 {
    // Normalize and clamp from -1 to 1
    let float = math::normalize_f32(float).clamp(-1.0, 1.0);

    // Convert to signed integer
    normal_f32_to_i32(float)
}

/// Convert [`u32`] fraction to [`i32`] fraction.
#[inline(always)]
pub const fn u32_to_i32(fraction: u32) -> i32 {
    Signed(fraction).reinterpret_with_offset()
}

/// Convert [`i32`] fraction to [`u32`] fraction.
#[inline(always)]
pub const fn i32_to_u32(fraction: i32) -> u32 {
    Unsigned(fraction).reinterpret_with_offset()
}

/// Convert [`u64`] fraction to [`i64`] fraction.
#[inline(always)]
pub const fn u64_to_i64(fraction: u64) -> i64 {
    Signed(fraction).reinterpret_with_offset()
}

/// Convert [`i64`] fraction to [`u64`] fraction.
#[inline(always)]
pub const fn i64_to_u64(fraction: i64) -> u64 {
    Unsigned(fraction).reinterpret_with_offset()
}

#[inline(always)]
const fn add_sign_long(float: f64, sign: i64) -> f64 {
    f64::from_bits(float.to_bits() | (Unsigned(sign).reinterpret() << 63))
}

/// Convert non-zero [`u64`] fraction to [`f64`] (ranged 0 to 1).
#[inline(always)]
const fn nonzero_u64_to_f64(fraction: u64) -> f64 {
    // Calculate leading zeros (with inferred 1)
    let leading_zeros = fraction.leading_zeros() + 1;
    // Remove leading zeros and inferred 1 to subtract from exponent
    let fraction = fraction.wrapping_shl(leading_zeros);
    // Shift right to truncate to 52-bit fraction
    let fraction = fraction >> 12;
    // Calculate -1023 bias exponent
    let exponent = (1023 - (leading_zeros as u64)) << 52;

    // Scale up (u64 max is 2⁶⁴ - 1, and we want 2⁶⁴)
    f64::from_bits(exponent | fraction)
        * f64::from_bits(
            0b11111111110000000000000000000000000000000000000000000000000001,
        )
}

/// Convert normal [`f64`] (ranged 0 to 1) to [`u64`] fraction.
#[inline(always)]
const fn normal_f64_to_u64(float: f64) -> u64 {
    // Scale down (f64 max fraction is 2⁶⁴, and we want 2⁶⁴ - 1)
    let float = (float
        * f64::from_bits(
            0b11111111101111111111111111111111111111111111111111111111111110,
        ))
    .to_bits();
    // Convert fraction to 52 bits
    let fraction = (float << 12) >> 1;
    // Artificially extend fraction precision, and add inferred 1
    let fraction = (1 << 63) | fraction | (fraction >> 52);
    // Extract -1023 bias 11-bit negative exponent
    let exponent = Unsigned(1023 - Signed((float >> 52) as u32).reinterpret())
        .reinterpret();
    // Scale by exponent
    let (fraction, overflow) = fraction.overflowing_shr(exponent - 1);
    // Check if fraction should be 0 or not
    let nonzero = Unsigned(-math::long(!overflow)).reinterpret();

    // Make zero if zero, otherwise no-op
    fraction & nonzero
}

/// Convert normal [`f64`] (ranged -1 to 1) to [`i64`] fraction.
#[inline(always)]
const fn normal_f64_to_i64(float: f64) -> i64 {
    // Convert to unsigned integer and reduce precision
    let magnitude = Signed(normal_f64_to_u64(float.abs()) >> 1).reinterpret();
    // Get offset
    let offset = -math::long(float.is_sign_negative());
    // Get sign
    let sign = (offset * 2) + 1;

    // Construct fraction with sign, magnitude, and offset
    offset + (magnitude * sign)
}

/// Convert [`u64`] fraction to [`f64`] (ranged 0 to 1).
pub const fn u64_to_f64(fraction: u64) -> f64 {
    // Check if fraction is 0 or not
    let nonzero = Unsigned(-math::long(fraction != 0)).reinterpret();

    // Make zero if zero, otherwise no-op
    f64::from_bits(nonzero_u64_to_f64(fraction).to_bits() & nonzero)
}

/// Convert [`i64`] fraction to [`f64`] (ranged -1 to 1).
pub const fn i64_to_f64(int: i64) -> f64 {
    // Split sign and magnitude from signed integer
    let sign = -math::long(int < 0);
    let uint = int.abs_diff(sign);
    // Scale up unsigned integer to full range (without true zero)
    let uint = (uint * 2) + 1;

    // Copy sign back into converted float
    add_sign_long(nonzero_u64_to_f64(uint), sign)
}

/// Convert [`f64`] (ranged 0 to 1) to [`u64`] fraction.
#[inline(always)]
pub const fn f64_to_u64(float: f64) -> u64 {
    // Normalize and clamp from 0 to 1
    let float = math::normalize_f64(float).clamp(0.0, 1.0);

    // Convert to unsigned integer
    normal_f64_to_u64(float)
}

/// Convert [`f64`] (ranged -1 to 1) to [`i64`] fraction.
#[inline(always)]
pub const fn f64_to_i64(float: f64) -> i64 {
    // Normalize and clamp from -1 to 1
    let float = math::normalize_f64(float).clamp(-1.0, 1.0);

    // Convert to signed integer
    normal_f64_to_i64(float)
}
