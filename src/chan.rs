//! Low-level float / integer channel conversions

use crate::conversions::{self, Signed, Unsigned};

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
    let nonzero = Unsigned(-conversions::word(!overflow)).reinterpret();

    // Make zero if zero, otherwise no-op
    fraction & nonzero
}

/// Convert normal [`f32`] (ranged -1 to 1) to [`i32`] fraction.
#[inline(always)]
const fn normal_f32_to_i32(float: f32) -> i32 {
    // Convert to unsigned integer and reduce precision
    let magnitude = Signed(normal_f32_to_u32(float.abs()) >> 1).reinterpret();
    // Get offset
    let offset = -conversions::word(float.is_sign_negative());
    // Get sign
    let sign = (offset * 2) + 1;

    // Construct fraction with sign, magnitude, and offset
    offset + (magnitude * sign)
}

/// Convert [`u32`] fraction to [`f32`] (ranged 0 to 1).
pub const fn u32_to_f32(fraction: u32) -> f32 {
    // Check if fraction is 0 or not
    let nonzero = Unsigned(-conversions::word(fraction != 0)).reinterpret();

    // Make zero if zero, otherwise no-op
    f32::from_bits(nonzero_u32_to_f32(fraction).to_bits() & nonzero)
}

/// Convert [`i32`] fraction to [`f32`] (ranged -1 to 1).
pub const fn i32_to_f32(int: i32) -> f32 {
    // Split sign and magnitude from signed integer
    let sign = -conversions::word(int < 0);
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
    let float = conversions::normalize_f32(float).clamp(0.0, 1.0);

    // Convert to unsigned integer
    normal_f32_to_u32(float)
}

/// Convert [`f32`] (ranged -1 to 1) to [`i32`] fraction.
#[inline(always)]
pub const fn f32_to_i32(float: f32) -> i32 {
    // Normalize and clamp from -1 to 1
    let float = conversions::normalize_f32(float).clamp(-1.0, 1.0);

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

/// Upscale `u24` fraction to [`u32`] fraction.
#[inline(always)]
pub const fn u24_to_u32(fraction: u32) -> u32 {
    let upper = fraction << 8;
    let lower = upper >> 24;

    upper | lower
}

/// Upscale [`u16`] fraction to [`u32`] fraction.
#[inline(always)]
pub const fn u16_to_u32(fraction: u16) -> u32 {
    let fraction = fraction as u32;

    fraction | (fraction << 16)
}

/// Upscale `u12` fraction to [`u32`] fraction.
#[inline(always)]
pub const fn u12_to_u32(fraction: u16) -> u32 {
    let fraction = fraction as u32;
    let upper = fraction << 4;
    let middle = upper >> 12;
    let lower = upper >> 24;

    upper | middle | lower
}

/// Upscale [`u8`] fraction to [`u32`] fraction.
#[inline(always)]
pub const fn u8_to_u32(fraction: u8) -> u32 {
    u32::from_ne_bytes([fraction, fraction, fraction, fraction])
}

/// Upscale `i24` fraction to [`i32`] fraction.
#[inline(always)]
pub const fn i24_to_i32(mut fraction: i32) -> i32 {
    if fraction > 2_i32.pow(23) - 1 {
        fraction = 2_i32.pow(23) - 1;
    }

    if fraction < -2_i32.pow(23) {
        fraction = -2_i32.pow(23);
    }

    let fraction = Unsigned(fraction).reinterpret() ^ (1 << (i32::BITS - 9));

    Signed(u24_to_u32(fraction)).reinterpret_with_offset()
}

/// Upscale [`i16`] fraction to [`i32`] fraction.
#[inline(always)]
pub const fn i16_to_i32(fraction: i16) -> i32 {
    Signed(u16_to_u32(Unsigned(fraction).reinterpret_with_offset()))
        .reinterpret_with_offset()
}

/// Upscale `i12` fraction to [`i32`] fraction.
#[inline(always)]
pub const fn i12_to_i32(mut fraction: i16) -> i32 {
    if fraction > 2_i16.pow(11) - 1 {
        fraction = 2_i16.pow(11) - 1;
    }

    if fraction < -2_i16.pow(11) {
        fraction = -2_i16.pow(11);
    }

    let fraction = Unsigned(fraction).reinterpret() ^ (1 << (i16::BITS - 5));

    Signed(u12_to_u32(fraction)).reinterpret_with_offset()
}

/// Upscale [`i8`] fraction to [`i32`] fraction.
#[inline(always)]
pub const fn i8_to_i32(fraction: i8) -> i32 {
    Signed(u8_to_u32(Unsigned(fraction).reinterpret_with_offset()))
        .reinterpret_with_offset()
}
