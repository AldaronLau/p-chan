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

    /// 8-bit signed channel sample
    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
    )]
    #[repr(transparent)]
    pub struct Ch8(i8);

    /// 12-bit signed channel sample
    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
    )]
    #[repr(transparent)]
    pub struct Ch12(i16);

    /// 16-bit signed channel sample
    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
    )]
    #[repr(transparent)]
    pub struct Ch16(i16);

    /// 24-bit signed channel sample
    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
    )]
    #[repr(transparent)]
    pub struct Ch24(i32);

    ch_float!(
        (Ch32, f32, core::convert::identity, -1.0, 0.0),
        doc = "32-bit float (-1 to 1) channel value",
    );

    ch_float!(
        (Ch64, f64, core::convert::identity, -1.0, 0.0),
        doc = "64-bit float (-1 to 1) channel value",
    );

    ops_int!(Ch8, i8, i16, core::convert::identity);
    ops_int!(Ch12, i16, i32, core::convert::identity);
    ops_int!(Ch16, i16, i32, core::convert::identity);
    ops_int!(Ch24, i32, i64, core::convert::identity);
}

#[cfg(feature = "unsigned")]
pub mod unsigned {
    //! Unsigned channel newtypes

    /// 8-bit unsigned channel sample
    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
    )]
    #[repr(transparent)]
    pub struct Ch8(u8);

    /// 12-bit unsigned channel sample
    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
    )]
    #[repr(transparent)]
    pub struct Ch12(u16);

    /// 16-bit unsigned channel sample
    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
    )]
    #[repr(transparent)]
    pub struct Ch16(u16);

    /// 24-bit unsigned channel sample
    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
    )]
    #[repr(transparent)]
    pub struct Ch24(u32);

    ch_float!(
        (Ch32, f32, core::convert::identity, 0.0, 0.5),
        doc = "32-bit float (0 to 1) channel value",
    );

    ch_float!(
        (Ch64, f64, core::convert::identity, 0.0, 0.5),
        doc = "64-bit float (0 to 1) channel value",
    );

    ops_int!(Ch8, u8, u16, core::convert::identity);
    ops_int!(Ch12, u16, u32, core::convert::identity);
    ops_int!(Ch16, u16, u32, core::convert::identity);
    ops_int!(Ch24, u32, u64, core::convert::identity);
}
