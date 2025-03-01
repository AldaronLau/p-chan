//! Multimedia (Audio, Raster) Channel Newtypes and Conversions
//!
//! Each module is enabled with a feature by the same name.
//!
//! The types provided by each module are `Ch8`, `Ch12`, `Ch16`, `Ch24` for
//! integers, and `Ch32` and `Ch64` for floating-point.  Integer channels can
//! not exceed their maximum value, while floating-point channels can.
//! Floating-point channels can only ever be normal numbers.

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
    html_logo_url = "https://raw.githubusercontent.com/ardaku/whoami/v1/res/icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/ardaku/whoami/v1/res/icon.svg"
)]

#[cfg(any(feature = "unsigned", feature = "signed"))]
#[macro_use]
mod macros;

#[cfg(feature = "signed")]
pub mod signed {
    //! Signed channel newtypes

    macro_rules! midpoint {
        () => {
            /// Calculates the middle point of `self` and `rhs`.
            ///
            /// `midpoint(a, b)` is `(a + b) / 2` calculated without overflow,
            /// rounded to zero.
            pub const fn midpoint(self, rhs: Self) -> Self {
                use crate::conversions::{Signed, Unsigned};

                let this =
                    Unsigned(self.0.wrapping_sub(Self::MIN.0)).reinterpret();
                let rhs =
                    Unsigned(rhs.0.wrapping_sub(Self::MIN.0)).reinterpret();

                Self(
                    Signed(this.midpoint(rhs))
                        .reinterpret()
                        .wrapping_add(Self::MIN.0),
                )
            }
        };
    }

    ch_int!(
        (Ch8, i8, i16, core::convert::identity, midpoint! {}),
        doc = "8-bit signed integer (-128 to 127) channel value",
    );

    ch_int!(
        (Ch12, i16, i32, core::convert::identity, midpoint! {}),
        doc = "12-bit signed integer (-2\\_048 to 2\\_047) channel value",
    );

    ch_int!(
        (Ch16, i16, i32, core::convert::identity, midpoint! {}),
        doc = "16-bit signed integer (-32\\_768 to 32\\_767) channel value",
    );

    ch_int!(
        (Ch24, i32, i64, core::convert::identity, midpoint! {}),
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
        (Ch12, u16, u32, core::convert::identity, midpoint! {}),
        doc = "12-bit unsigned integer (0 to 4\\_095) channel value",
    );

    ch_int!(
        (Ch16, u16, u32, core::convert::identity, midpoint! {}),
        doc = "16-bit unsigned integer (0 to 65\\_535) channel value",
    );

    ch_int!(
        (Ch24, u32, u64, core::convert::identity, midpoint! {}),
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
}

mod conversions {
    #![allow(dead_code)]

    macro_rules! reinterpret {
        ($wrapper: ident, $from: ty, $to: ty) => {
            impl $wrapper<$from> {
                #[inline(always)]
                pub(super) const fn reinterpret(self) -> $to {
                    <$to>::from_ne_bytes(self.0.to_ne_bytes())
                }
            }
        };
    }

    pub(super) struct Unsigned<T>(pub(super) T);

    reinterpret!(Unsigned, i8, u8);
    reinterpret!(Unsigned, i16, u16);
    reinterpret!(Unsigned, i32, u32);
    reinterpret!(Unsigned, i64, u64);
    reinterpret!(Unsigned, i128, u128);

    pub(super) struct Signed<T>(pub(super) T);

    reinterpret!(Signed, u8, i8);
    reinterpret!(Signed, u16, i16);
    reinterpret!(Signed, u32, i32);
    reinterpret!(Signed, u64, i64);
    reinterpret!(Signed, u128, i128);
}
