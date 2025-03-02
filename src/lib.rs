//! Multimedia (Audio, Raster) Channel Newtypes and Conversions
//!
//! Each module is enabled with a feature by the same name.
//!
//! The types provided by each module are `Ch8`, `Ch12`, `Ch16`, `Ch24` for
//! integers, and `Ch32` and `Ch64` for floating-point.  Integer channels can
//! not exceed the range of their minimum and maximum values, while
//! floating-point channels can.  Floating-point channels can only ever be
//! normal numbers.

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
    html_logo_url = "https://raw.githubusercontent.com/AldaronLau/p-chan/v1/res/icon.png",
    html_favicon_url = "https://raw.githubusercontent.com/AldaronLau/p-chan/v1/res/icon.png"
)]

mod conversions;
#[cfg(any(feature = "unsigned", feature = "signed"))]
#[macro_use]
mod macros;
pub mod ops;

#[cfg(feature = "signed")]
pub mod signed {
    //! Signed channel newtypes

    macro_rules! midpoint {
        () => {
            /// Calculates the middle point of `self` and `rhs`.
            ///
            /// `midpoint(a, b)` is `(a + b) / 2` calculated without overflow,
            /// rounded down.
            pub const fn midpoint(self, rhs: Self) -> Self {
                use crate::conversions::{Signed, Unsigned};

                let this = Unsigned(self.0).reinterpret_with_offset();
                let rhs = Unsigned(rhs.0).reinterpret_with_offset();

                Self(Signed(this.midpoint(rhs)).reinterpret_with_offset())
            }
        };
    }

    ch_int!(
        (Ch8, i8, i16, core::convert::identity, midpoint! {}),
        doc = "8-bit signed integer (-128 to 127) channel value",
    );

    ch_int!(
        (Ch12, i16, i32, normalize_ch12, midpoint! {}),
        doc = "12-bit signed integer (-2\\_048 to 2\\_047) channel value",
    );

    ch_int!(
        (Ch16, i16, i32, core::convert::identity, midpoint! {}),
        doc = "16-bit signed integer (-32\\_768 to 32\\_767) channel value",
    );

    ch_int!(
        (Ch24, i32, i64, normalize_ch24, midpoint! {}),
        doc = "24-bit signed integer (-8\\_388\\_608 to 8\\_388\\_607) channel value",
    );

    ch_float!(
        (Ch32, f32, core::convert::identity, -1.0, 0.0),
        doc = "32-bit float (-1 to 1) channel value",
    );

    ch_float!(
        (Ch64, f64, core::convert::identity, -1.0, 0.0),
        doc = "64-bit float (-1 to 1) channel value",
    );

    const fn normalize_ch12(mut chan: i16) -> i16 {
        if chan > 2_i16.pow(11) - 1 {
            chan = 2_i16.pow(11) - 1;
        }

        if chan < -2_i16.pow(11) {
            chan = -2_i16.pow(11);
        }

        chan
    }

    const fn normalize_ch24(mut chan: i32) -> i32 {
        if chan > 2_i32.pow(23) - 1 {
            chan = 2_i32.pow(23) - 1;
        }

        if chan < -2_i32.pow(23) {
            chan = -2_i32.pow(23);
        }

        chan
    }
}

#[cfg(feature = "unsigned")]
pub mod unsigned {
    //! Unsigned channel newtypes

    macro_rules! midpoint {
        () => {
            /// Calculates the middle point of `self` and `rhs`.
            ///
            /// `midpoint(a, b)` is `(a + b) / 2` calculated without overflow,
            /// rounded to zero.
            pub const fn midpoint(self, rhs: Self) -> Self {
                Self(self.0.midpoint(rhs.0))
            }
        };
    }

    ch_int!(
        (Ch8, u8, u16, core::convert::identity, midpoint! {}),
        doc = "8-bit (0 to 255) unsigned integer channel value",
    );

    ch_int!(
        (Ch12, u16, u32, normalize_ch12, midpoint! {}),
        doc = "12-bit unsigned integer (0 to 4\\_095) channel value",
    );

    ch_int!(
        (Ch16, u16, u32, core::convert::identity, midpoint! {}),
        doc = "16-bit unsigned integer (0 to 65\\_535) channel value",
    );

    ch_int!(
        (Ch24, u32, u64, normalize_ch24, midpoint! {}),
        doc = "24-bit unsigned integer (0 to 16\\_777\\_215) channel value",
    );

    ch_float!(
        (Ch32, f32, core::convert::identity, 0.0, 0.5),
        doc = "32-bit float (0 to 1) channel value",
    );

    ch_float!(
        (Ch64, f64, core::convert::identity, 0.0, 0.5),
        doc = "64-bit float (0 to 1) channel value",
    );

    const fn normalize_ch12(chan: u16) -> u16 {
        (chan << 4) >> 4
    }

    const fn normalize_ch24(chan: u32) -> u32 {
        (chan << 8) >> 8
    }
}
