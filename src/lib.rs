//! Multimedia (Audio, Raster) Channel Newtypes and Conversions
//!
//! Each module is enabled with a feature by the same name.

#[cfg(any(feature = "unsigned", feature = "signed"))]
macro_rules! ops_int {
    ($ty: ty, $p: ty, $b: ty, $normalize: path) => {
        impl $ty {
            /// Minimum value
            pub const MIN: Self = Self::new(<$p>::MIN);
            /// Middle value
            pub const MID: Self = Self::new(<$p>::MAX >> 1);
            /// Maximum value
            pub const MAX: Self = Self::new(<$p>::MAX);

            /// Create a new channel value.
            pub const fn new(value: $p) -> Self {
                Self(value)
            }
        }

        impl core::ops::Add for $ty {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self(self.0.saturating_add(rhs.0))
            }
        }

        impl core::ops::Sub for $ty {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }
        }
    };
}

#[cfg(any(feature = "unsigned", feature = "signed"))]
macro_rules! ops_float {
    ($ty: ty, $p: ty, $normalize: path, $min: literal, $mid: literal) => {
        impl $ty {
            /// Minimum value
            pub const MIN: Self = Self::new($min);
            /// Middle value
            pub const MID: Self = Self::new($mid);
            /// Maximum value
            pub const MAX: Self = Self::new(1.0);

            /// Create a new channel value.
            pub const fn new(value: $p) -> Self {
                Self(value)
            }
        }

        impl core::ops::Add for $ty {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl core::ops::Sub for $ty {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }
    };
}

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
   
    /// 32-bit float (-1 to 1) channel sample
    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
    #[repr(transparent)]
    pub struct Ch32(f32);
   
    /// 64-bit float (-1 to 1) channel sample
    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
    #[repr(transparent)]
    pub struct Ch64(f64);

    ops_int!(Ch8, i8, i16, core::convert::identity);
    ops_int!(Ch12, i16, i32, core::convert::identity);
    ops_int!(Ch16, i16, i32, core::convert::identity);
    ops_int!(Ch24, i32, i64, core::convert::identity);
    ops_float!(Ch32, f32, core::convert::identity, -1.0, 0.0);
    ops_float!(Ch64, f64, core::convert::identity, -1.0, 0.0);
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
   
    /// 32-bit float (0 to 1) channel sample
    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
    #[repr(transparent)]
    pub struct Ch32(f32);
    
    /// 64-bit float (0 to 1) channel sample
    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
    #[repr(transparent)]
    pub struct Ch64(f64);

    ops_int!(Ch8, u8, u16, core::convert::identity);
    ops_int!(Ch12, u16, u32, core::convert::identity);
    ops_int!(Ch16, u16, u32, core::convert::identity);
    ops_int!(Ch24, u32, u64, core::convert::identity);
    ops_float!(Ch32, f32, core::convert::identity, 0.0, 0.5);
    ops_float!(Ch64, f64, core::convert::identity, 0.0, 0.5);
}
