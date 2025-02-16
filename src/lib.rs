//! Multimedia (Audio, Raster) Channel Newtypes and Conversions
//!
//! Each module is enabled with a feature by the same name.

macro_rules! ops_int {
    ($ty: ty) => {
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

macro_rules! ops_float {
    ($ty: ty) => {
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
    pub struct Ch8(i8);
    /// 12-bit signed channel sample
    pub struct Ch12(i16);
    /// 16-bit signed channel sample
    pub struct Ch16(i16);
    /// 24-bit signed channel sample
    pub struct Ch24(i32);
    /// 32-bit float (-1 to 1) channel sample
    pub struct Ch32(f32);
    /// 64-bit float (-1 to 1) channel sample
    pub struct Ch64(f64);

    ops_int!(Ch8);
    ops_int!(Ch12);
    ops_int!(Ch16);
    ops_int!(Ch24);
    ops_float!(Ch32);
    ops_float!(Ch64);
}

#[cfg(feature = "unsigned")]
pub mod unsigned {
    //! Unsigned channel newtypes

    /// 8-bit unsigned channel sample
    pub struct Ch8(u8);
    /// 12-bit unsigned channel sample
    pub struct Ch12(u16);
    /// 16-bit unsigned channel sample
    pub struct Ch16(u16);
    /// 24-bit unsigned channel sample
    pub struct Ch24(u32);
    /// 32-bit float (0 to 1) channel sample
    pub struct Ch32(f32);
    /// 64-bit float (0 to 1) channel sample
    pub struct Ch64(f64);

    ops_int!(Ch8);
    ops_int!(Ch12);
    ops_int!(Ch16);
    ops_int!(Ch24);
    ops_float!(Ch32);
    ops_float!(Ch64);
}
