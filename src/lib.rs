//! Multimedia (Audio, Raster) Channel Newtypes and Conversions
//!
//! Each module is enabled with a feature by the same name.

#![no_std]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences,
    unsafe_code
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ardaku/whoami/v1/res/icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/ardaku/whoami/v1/res/icon.svg"
)]

#[cfg(any(feature = "unsigned", feature = "signed"))]
#[macro_use]
mod macros;

#[cfg(feature = "signed")]
pub mod signed {
    //! Signed channel newtypes

    // Stolen from: https://doc.rust-lang.org/src/core/num/mod.rs.html#148-155
    macro_rules! midpoint {
        () => {
            /// Calculates the middle point of `self` and `rhs`.
            ///
            /// `midpoint(a, b)` is `(a + b) / 2` calculated without overflow,
            /// rounded to zero.
            pub const fn midpoint(self, rhs: Self) -> Self {
                // Use the well known branchless algorithm from Hacker's Delight
                // to compute `(a + b) / 2` without overflowing:
                // `((a ^ b) >> 1) + (a & b)`.
                let t = ((self.0 ^ rhs.0) >> 1) + (self.0 & rhs.0);

                // Except that it fails for integers whose sum is an odd
                // negative number as their floor is one less than their
                // average. So we adjust the result.
                Self(t + (if t < 0 { 1 } else { 0 } & (self.0 ^ rhs.0)))
            }
        };
    }

    ch_int!(
        (Ch8, i8, i16, core::convert::identity, midpoint! {}),
        doc = "8-bit signed integer channel value",
    );

    ch_int!(
        (Ch12, i16, i32, core::convert::identity, midpoint! {}),
        doc = "12-bit signed integer channel value",
    );

    ch_int!(
        (Ch16, i16, i32, core::convert::identity, midpoint! {}),
        doc = "16-bit signed integer channel value",
    );

    ch_int!(
        (Ch24, i32, i64, core::convert::identity, midpoint! {}),
        doc = "24-bit signed integer channel value",
    );

    ch_float!(
        (Ch32, f32, core::convert::identity, -1.0, 0.0),
        doc = "32-bit float (-1 to 1) channel value",
    );

    ch_float!(
        (Ch64, f64, core::convert::identity, -1.0, 0.0),
        doc = "64-bit float (-1 to 1) channel value",
    );
}

#[cfg(feature = "unsigned")]
pub mod unsigned {
    //! Unsigned channel newtypes

    // Stolen from: https://doc.rust-lang.org/src/core/num/mod.rs.html#121-125
    macro_rules! midpoint {
        () => {
            /// Calculates the middle point of `self` and `rhs`.
            ///
            /// `midpoint(a, b)` is `(a + b) / 2` calculated without overflow,
            /// rounded to zero.
            pub const fn midpoint(self, rhs: Self) -> Self {
                // Use the well known branchless algorithm from Hacker's Delight
                // to compute `(a + b) / 2` without overflowing:
                // `((a ^ b) >> 1) + (a & b)`.
                Self(((self.0 ^ rhs.0) >> 1) + (self.0 & rhs.0))
            }
        };
    }

    ch_int!(
        (Ch8, u8, u16, core::convert::identity, midpoint! {}),
        doc = "8-bit unsigned integer channel value",
    );

    ch_int!(
        (Ch12, u16, u32, core::convert::identity, midpoint! {}),
        doc = "12-bit unsigned integer channel value",
    );

    ch_int!(
        (Ch16, u16, u32, core::convert::identity, midpoint! {}),
        doc = "16-bit unsigned integer channel value",
    );

    ch_int!(
        (Ch24, u32, u64, core::convert::identity, midpoint! {}),
        doc = "24-bit unsigned integer channel value",
    );

    ch_float!(
        (Ch32, f32, core::convert::identity, 0.0, 0.5),
        doc = "32-bit float (0 to 1) channel value",
    );

    ch_float!(
        (Ch64, f64, core::convert::identity, 0.0, 0.5),
        doc = "64-bit float (0 to 1) channel value",
    );
}
