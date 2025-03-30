//! Low-level integer channel upscaling conversions

use crate::math;

/// Downscale [`u32`] fraction to `u24` fraction.
#[inline(always)]
pub const fn u32_to_u24(fraction: u32) -> u32 {
    fraction >> 8
}

/// Downscale [`u32`] fraction to [`u16`] fraction.
#[inline(always)]
pub const fn u32_to_u16(fraction: u32) -> u16 {
    (fraction >> 16) as u16
}

/// Downscale [`u32`] fraction to `u12` fraction.
#[inline(always)]
pub const fn u32_to_u12(fraction: u32) -> u16 {
    (fraction >> 20) as u16
}

/// Downscale [`u32`] fraction to [`u8`] fraction.
#[inline(always)]
pub const fn u32_to_u8(fraction: u32) -> u8 {
    (fraction >> 24) as u8
}

/// Downscale [`i32`] fraction to `i24` fraction.
#[inline(always)]
pub const fn i32_to_i24(fraction: i32) -> i32 {
    fraction >> 8
}

/// Downscale [`i32`] fraction to [`i16`] fraction.
#[inline(always)]
pub const fn i32_to_i16(fraction: i32) -> i16 {
    (fraction >> 16) as i16
}

/// Downscale [`i32`] fraction to `i12` fraction.
#[inline(always)]
pub const fn i32_to_i12(fraction: i32) -> i16 {
    (fraction >> 20) as i16
}

/// Downscale [`i32`] fraction to [`i8`] fraction.
#[inline(always)]
pub const fn i32_to_i8(fraction: i32) -> i8 {
    (fraction >> 24) as i8
}

/// Downscale [`f64`] to [`f32`] and normalize.
#[inline(always)]
pub const fn f64_to_f32(float: f64) -> f32 {
    let float = math::normalize_f64(float).to_bits();
    let sign = ((float >> 63) as u32) << 31;
    let exponent = (((float << 1) >> 53) as u32).saturating_sub(1023 - 127);
    let exponent = (exponent << 24) >> 1;
    let fraction = (float >> 28) as u32;
    let fraction = fraction + (fraction & 1);
    let fraction = (fraction << 8) >> 9;

    f32::from_bits(sign | exponent | fraction)
}

/// Downscale [`u64`] fraction to [`u32`] fraction.
#[inline(always)]
pub const fn u64_to_u32(fraction: u64) -> u32 {
    (fraction >> 32) as u32
}

/// Downscale [`i64`] fraction to [`i32`] fraction.
#[inline(always)]
pub const fn i64_to_i32(fraction: i64) -> i32 {
    (fraction >> 32) as i32
}

/// Downscale [`u64`] fraction to `u24` fraction.
#[inline(always)]
pub const fn u64_to_u24(fraction: u64) -> u32 {
    (fraction >> 40) as u32
}

/// Downscale [`u64`] fraction to [`u16`] fraction.
#[inline(always)]
pub const fn u64_to_u16(fraction: u64) -> u16 {
    (fraction >> 48) as u16
}

/// Downscale [`u64`] fraction to `u12` fraction.
#[inline(always)]
pub const fn u64_to_u12(fraction: u64) -> u16 {
    (fraction >> 52) as u16
}

/// Downscale [`u64`] fraction to [`u8`] fraction.
#[inline(always)]
pub const fn u64_to_u8(fraction: u64) -> u8 {
    (fraction >> 56) as u8
}

/// Downscale [`i64`] fraction to `i24` fraction.
#[inline(always)]
pub const fn i64_to_i24(fraction: i64) -> i32 {
    (fraction >> 40) as i32
}

/// Downscale [`i64`] fraction to [`i16`] fraction.
#[inline(always)]
pub const fn i64_to_i16(fraction: i64) -> i16 {
    (fraction >> 48) as i16
}

/// Downscale [`i64`] fraction to `i12` fraction.
#[inline(always)]
pub const fn i64_to_i12(fraction: i64) -> i16 {
    (fraction >> 52) as i16
}

/// Downscale [`i64`] fraction to [`i8`] fraction.
#[inline(always)]
pub const fn i64_to_i8(fraction: i64) -> i8 {
    (fraction >> 56) as i8
}
