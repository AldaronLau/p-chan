//! Low-level integer channel upscaling conversions

use crate::math::{self, Signed, Unsigned};

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

/// Upscale [`f32`] to [`f64`] and normalize.
#[inline(always)]
pub const fn f32_to_f64(float: f32) -> f64 {
    math::normalize_f32(float) as f64
}

/// Upscale [`u32`] fraction to [`u64`] fraction.
#[inline(always)]
pub const fn u32_to_u64(fraction: u32) -> u64 {
    let fraction = fraction as u64;

    fraction | (fraction << 32)
}

/// Upscale [`i32`] fraction to [`i64`] fraction.
#[inline(always)]
pub const fn i32_to_i64(fraction: i32) -> i64 {
    Signed(u32_to_u64(Unsigned(fraction).reinterpret_with_offset()))
        .reinterpret_with_offset()
}
