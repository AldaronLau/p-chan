//! Low-level integer channel upscaling conversions

use crate::math;

/// Downscale [`f64`] fraction to [`f32`] fraction.
#[inline(always)]
pub const fn f64_to_f32(float: f64) -> f32 {
    let float = math::normalize_f64(float).to_bits();
    let sign = ((float >> 63) as u32) << 31;
    let exponent = ((float >> 52) as u32) << 23;
    let fraction = ((float << 12) >> 41) as u32;

    f32::from_bits(sign | exponent | fraction)
}
