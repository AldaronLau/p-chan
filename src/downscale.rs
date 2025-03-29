//! Low-level integer channel upscaling conversions

use crate::math;

/// Downscale [`f64`] fraction to [`f32`] fraction.
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
