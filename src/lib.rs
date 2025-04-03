//! Multimedia (Audio, Raster) Channel Newtypes and Conversions
//!
//! The `unsigned` and `signed` modules are enabled by features with the same
//! name.
//!
//! The types provided by the `unsigned` and `signed` module are `Ch8`, `Ch12`,
//! `Ch16`, `Ch24` for integers, and `Ch32` and `Ch64` for floating-point.
//!
//! Math operations on integer channels won't exceed the range of their minimum
//! and maximum values, while math on floating-point channels can.
//! Math operations on floating-point channels will only result in normal
//! numbers or ±infinity.  Floating-point channels implement [`Eq`] and [`Ord`]
//! since NaN is always flushed to zero.
//!
//! Channels support casting with the [`bytemuck`] crate, after which integer
//! channels may contain out of range values and floating-point channels could
//! contain NaN or denormals.  To flush denormals and NaN to zero and clamp
//! integer ranges you can use [`ops::Difference::sub()`] on each channel value.

#![no_std]
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unsafe_code
)]
#![warn(
    anonymous_parameters,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/AldaronLau/p-chan/v0/res/icon.png",
    html_favicon_url = "https://raw.githubusercontent.com/AldaronLau/p-chan/v0/res/icon.png"
)]

pub use bytemuck;

mod math;
#[cfg(any(feature = "unsigned", feature = "signed"))]
#[macro_use]
mod macros;
pub mod convert;
pub mod downscale;
pub mod ops;
#[cfg(feature = "signed")]
pub mod signed;
#[cfg(feature = "unsigned")]
pub mod unsigned;
pub mod upscale;
