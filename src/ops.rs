//! Constant "trait" operations
//!
//! This module provides wrapper structs for performing operations on channel
//! values (which are `const` compatible).
//!
//! ## Conversion
//!
//! This is like [`From`] / [`Into`], but `const`.
//!
//! ## Difference
//!
//! Subtract a list of channel values from a channel value.
//!
//! ## Inversion
//!
//! Flip value between the minimum and maximum.
//!
//! ## Negation
//!
//! This is the same as inversion except for unsigned floating-point channels.
//!
//! ## Product
//!
//! Multiply the channel values together (saturating for ints).
//!
//! ## Sum
//!
//! Add the channel values together (saturating for ints).

use core::marker::PhantomData;

/// Constant sum operation (`add`)
///
///  - `Sum([]).add()`
#[derive(Debug)]
pub struct Sum<T, const N: usize>(pub [T; N]);

/// Constant product operation (`mul`)
///
///  - `Product([]).mul()`
#[derive(Debug)]
pub struct Product<T, const N: usize>(pub [T; N]);

/// Constant conversion operation (`conv`)
///
///  - `Conversion::<_, U>::conv(_)`
#[derive(Debug)]
pub struct Conversion<T, U>(T, PhantomData<fn() -> U>);

/// Constant inversion operation (`inv`)
///
///  - `Inversion(_)::inv()`
#[derive(Debug)]
pub struct Inversion<T>(pub T);

/// Constant negation operation (`neg`)
///
///  - `Negation(_)::neg()`
#[derive(Debug)]
pub struct Negation<T>(pub T);

/// Constant difference operation (`sub`)
///
///  - `Difference(_, []).sub()`
#[derive(Debug)]
pub struct Difference<T, const N: usize>(pub T, pub [T; N]);

#[cfg(any(feature = "signed", feature = "unsigned"))]
macro_rules! int_channel {
    ($type:ty) => {
        impl<const N: usize> Sum<$type, N> {
            /// Add up the sum.
            pub const fn add(self) -> $type {
                let mut ret = 0;
                let mut i = 0;

                loop {
                    if i >= N {
                        break <$type>::new(ret);
                    }

                    let value = self.0[i];

                    ret = ret.saturating_add(value.into_inner());
                    i += 1;
                }
            }
        }

        impl<const N: usize> Difference<$type, N> {
            /// Subtract to get the difference.
            pub const fn sub(self) -> $type {
                let mut ret = self.0.into_inner();
                let mut i = 0;

                loop {
                    if i >= N {
                        break <$type>::new(ret);
                    }

                    let value = self.1[i];

                    ret = ret.saturating_sub(value.into_inner());
                    i += 1;
                }
            }
        }

        impl<const N: usize> Product<$type, N> {
            /// Multiply for the product.
            pub const fn mul(self) -> $type {
                let mut ret = <$type>::MAX;
                let mut i = 0;

                loop {
                    if i >= N {
                        break ret;
                    }

                    ret.int_multiply(self.0[i]);
                    i += 1;
                }
            }
        }
    };
}

#[cfg(any(feature = "signed", feature = "unsigned"))]
macro_rules! float_channel {
    ($type:ty) => {
        impl<const N: usize> Sum<$type, N> {
            /// Add up the sum.
            pub const fn add(self) -> $type {
                let mut ret = 0.0;
                let mut i = 0;

                loop {
                    if i >= N {
                        break <$type>::new(ret);
                    }

                    let value = self.0[i];

                    ret += value.into_inner();
                    i += 1;
                }
            }
        }

        impl<const N: usize> Difference<$type, N> {
            /// Subtract to get the difference.
            pub const fn sub(self) -> $type {
                let mut ret = self.0.into_inner();
                let mut i = 0;

                loop {
                    if i >= N {
                        break <$type>::new(ret);
                    }

                    let value = self.1[i];

                    ret -= value.into_inner();
                    i += 1;
                }
            }
        }

        impl<const N: usize> Product<$type, N> {
            /// Multiply for the product.
            pub const fn mul(self) -> $type {
                let mut ret = 1.0;
                let mut i = 0;

                loop {
                    if i >= N {
                        break <$type>::new(ret);
                    }

                    let value = self.0[i];

                    ret *= value.into_inner();
                    i += 1;
                }
            }
        }

        impl Negation<$type> {
            /// Negate the value.
            pub const fn neg(self) -> $type {
                <$type>::new(-self.0.into_inner())
            }
        }
    };
}

#[cfg(feature = "unsigned")]
mod unsigned {
    use super::*;
    use crate::unsigned::{Ch8, Ch12, Ch16, Ch24, Ch32, Ch64};

    int_channel!(Ch8);
    int_channel!(Ch12);
    int_channel!(Ch16);
    int_channel!(Ch24);
    float_channel!(Ch32);
    float_channel!(Ch64);

    impl Inversion<Ch32> {
        /// Invert the value.
        pub const fn inv(self) -> Ch32 {
            Ch32::new(1.0 - self.0.into_inner())
        }
    }

    impl Inversion<Ch64> {
        /// Invert the value.
        pub const fn inv(self) -> Ch64 {
            Ch64::new(1.0 - self.0.into_inner())
        }
    }

    impl Inversion<Ch8> {
        /// Invert the value.
        pub const fn inv(self) -> Ch8 {
            Ch8::new(Ch8::MAX.into_inner() - self.0.into_inner())
        }
    }

    impl Inversion<Ch12> {
        /// Invert the value.
        pub const fn inv(self) -> Ch12 {
            Ch12::new(Ch12::MAX.into_inner() - self.0.into_inner())
        }
    }

    impl Inversion<Ch16> {
        /// Invert the value.
        pub const fn inv(self) -> Ch16 {
            Ch16::new(Ch16::MAX.into_inner() - self.0.into_inner())
        }
    }

    impl Inversion<Ch24> {
        /// Invert the value.
        pub const fn inv(self) -> Ch24 {
            Ch24::new(Ch24::MAX.into_inner() - self.0.into_inner())
        }
    }

    impl Negation<Ch8> {
        /// Negate the value.
        pub const fn neg(self) -> Ch8 {
            Ch8::new(Ch8::MAX.into_inner() - self.0.into_inner())
        }
    }

    impl Negation<Ch12> {
        /// Negate the value.
        pub const fn neg(self) -> Ch12 {
            Ch12::new(Ch12::MAX.into_inner() - self.0.into_inner())
        }
    }

    impl Negation<Ch16> {
        /// Negate the value.
        pub const fn neg(self) -> Ch16 {
            Ch16::new(Ch16::MAX.into_inner() - self.0.into_inner())
        }
    }

    impl Negation<Ch24> {
        /// Negate the value.
        pub const fn neg(self) -> Ch24 {
            Ch24::new(Ch24::MAX.into_inner() - self.0.into_inner())
        }
    }

    impl Conversion<Ch8, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch12 {
            let value = crate::upscale::u8_to_u32(value.into_inner());

            Ch12::new(crate::downscale::u32_to_u12(value))
        }
    }

    impl Conversion<Ch8, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch16 {
            let value = crate::upscale::u8_to_u32(value.into_inner());

            Ch16::new(crate::downscale::u32_to_u16(value))
        }
    }

    impl Conversion<Ch8, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch24 {
            let value = crate::upscale::u8_to_u32(value.into_inner());

            Ch24::new(crate::downscale::u32_to_u24(value))
        }
    }

    impl Conversion<Ch8, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch32 {
            let value = crate::upscale::u8_to_u32(value.into_inner());

            Ch32::new(crate::convert::u32_to_f32(value))
        }
    }

    impl Conversion<Ch8, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch64 {
            let value = crate::upscale::u8_to_u64(value.into_inner());

            Ch64::new(crate::convert::u64_to_f64(value))
        }
    }

    impl Conversion<Ch12, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch8 {
            let value = crate::upscale::u12_to_u32(value.into_inner());

            Ch8::new(crate::downscale::u32_to_u8(value))
        }
    }

    impl Conversion<Ch12, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch16 {
            let value = crate::upscale::u12_to_u32(value.into_inner());

            Ch16::new(crate::downscale::u32_to_u16(value))
        }
    }

    impl Conversion<Ch12, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch24 {
            let value = crate::upscale::u12_to_u32(value.into_inner());

            Ch24::new(crate::downscale::u32_to_u24(value))
        }
    }

    impl Conversion<Ch12, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch32 {
            let value = crate::upscale::u12_to_u32(value.into_inner());

            Ch32::new(crate::convert::u32_to_f32(value))
        }
    }

    impl Conversion<Ch12, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch64 {
            let value = crate::upscale::u12_to_u64(value.into_inner());

            Ch64::new(crate::convert::u64_to_f64(value))
        }
    }

    impl Conversion<Ch16, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch8 {
            let value = crate::upscale::u16_to_u32(value.into_inner());

            Ch8::new(crate::downscale::u32_to_u8(value))
        }
    }

    impl Conversion<Ch16, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch12 {
            let value = crate::upscale::u16_to_u32(value.into_inner());

            Ch12::new(crate::downscale::u32_to_u12(value))
        }
    }

    impl Conversion<Ch16, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch24 {
            let value = crate::upscale::u16_to_u32(value.into_inner());

            Ch24::new(crate::downscale::u32_to_u24(value))
        }
    }

    impl Conversion<Ch16, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch32 {
            let value = crate::upscale::u16_to_u32(value.into_inner());

            Ch32::new(crate::convert::u32_to_f32(value))
        }
    }

    impl Conversion<Ch16, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch64 {
            let value = crate::upscale::u16_to_u64(value.into_inner());

            Ch64::new(crate::convert::u64_to_f64(value))
        }
    }

    impl Conversion<Ch24, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch8 {
            let value = crate::upscale::u24_to_u32(value.into_inner());

            Ch8::new(crate::downscale::u32_to_u8(value))
        }
    }

    impl Conversion<Ch24, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch12 {
            let value = crate::upscale::u24_to_u32(value.into_inner());

            Ch12::new(crate::downscale::u32_to_u12(value))
        }
    }

    impl Conversion<Ch24, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch16 {
            let value = crate::upscale::u24_to_u32(value.into_inner());

            Ch16::new(crate::downscale::u32_to_u16(value))
        }
    }

    impl Conversion<Ch24, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch32 {
            let value = crate::upscale::u24_to_u32(value.into_inner());

            Ch32::new(crate::convert::u32_to_f32(value))
        }
    }

    impl Conversion<Ch24, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch64 {
            let value = crate::upscale::u24_to_u64(value.into_inner());

            Ch64::new(crate::convert::u64_to_f64(value))
        }
    }

    impl Conversion<Ch32, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch8 {
            let value = crate::convert::f32_to_u32(value.into_inner());

            Ch8::new(crate::downscale::u32_to_u8(value))
        }
    }

    impl Conversion<Ch32, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch12 {
            let value = crate::convert::f32_to_u32(value.into_inner());

            Ch12::new(crate::downscale::u32_to_u12(value))
        }
    }

    impl Conversion<Ch32, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch16 {
            let value = crate::convert::f32_to_u32(value.into_inner());

            Ch16::new(crate::downscale::u32_to_u16(value))
        }
    }

    impl Conversion<Ch32, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch24 {
            let value = crate::convert::f32_to_u32(value.into_inner());

            Ch24::new(crate::downscale::u32_to_u24(value))
        }
    }

    impl Conversion<Ch32, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch64 {
            let value = crate::upscale::f32_to_f64(value.into_inner());

            Ch64::new(value)
        }
    }

    impl Conversion<Ch64, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch8 {
            let value = crate::convert::f64_to_u64(value.into_inner());

            Ch8::new(crate::downscale::u64_to_u8(value))
        }
    }

    impl Conversion<Ch64, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch12 {
            let value = crate::convert::f64_to_u64(value.into_inner());

            Ch12::new(crate::downscale::u64_to_u12(value))
        }
    }

    impl Conversion<Ch64, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch16 {
            let value = crate::convert::f64_to_u64(value.into_inner());

            Ch16::new(crate::downscale::u64_to_u16(value))
        }
    }

    impl Conversion<Ch64, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch24 {
            let value = crate::convert::f64_to_u64(value.into_inner());

            Ch24::new(crate::downscale::u64_to_u24(value))
        }
    }

    impl Conversion<Ch64, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch32 {
            let value = crate::downscale::f64_to_f32(value.into_inner());

            Ch32::new(value)
        }
    }

    #[cfg(feature = "signed")]
    mod signed {
        use super::*;
        use crate::signed;

        impl Conversion<signed::Ch8, Ch8> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch8) -> Ch8 {
                let value = crate::upscale::i8_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch8::new(crate::downscale::u32_to_u8(value))
            }
        }

        impl Conversion<signed::Ch12, Ch12> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch12) -> Ch12 {
                let value = crate::upscale::i12_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch12::new(crate::downscale::u32_to_u12(value))
            }
        }

        impl Conversion<signed::Ch16, Ch16> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch16) -> Ch16 {
                let value = crate::upscale::i16_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch16::new(crate::downscale::u32_to_u16(value))
            }
        }

        impl Conversion<signed::Ch24, Ch24> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch24) -> Ch24 {
                let value = crate::upscale::i24_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch24::new(crate::downscale::u32_to_u24(value))
            }
        }

        impl Conversion<signed::Ch32, Ch32> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch32) -> Ch32 {
                let value = crate::convert::f32_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch32::new(crate::convert::u32_to_f32(value))
            }
        }

        impl Conversion<signed::Ch64, Ch64> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch64) -> Ch64 {
                let value = crate::convert::f64_to_i64(value.into_inner());
                let value = crate::convert::i64_to_u64(value);

                Ch64::new(crate::convert::u64_to_f64(value))
            }
        }

        impl Conversion<signed::Ch8, Ch12> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch8) -> Ch12 {
                let value = crate::upscale::i8_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch12::new(crate::downscale::u32_to_u12(value))
            }
        }

        impl Conversion<signed::Ch8, Ch16> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch8) -> Ch16 {
                let value = crate::upscale::i8_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch16::new(crate::downscale::u32_to_u16(value))
            }
        }

        impl Conversion<signed::Ch8, Ch24> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch8) -> Ch24 {
                let value = crate::upscale::i8_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch24::new(crate::downscale::u32_to_u24(value))
            }
        }

        impl Conversion<signed::Ch8, Ch32> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch8) -> Ch32 {
                let value = crate::upscale::i8_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch32::new(crate::convert::u32_to_f32(value))
            }
        }

        impl Conversion<signed::Ch8, Ch64> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch8) -> Ch64 {
                let value = crate::upscale::i8_to_i64(value.into_inner());
                let value = crate::convert::i64_to_u64(value);

                Ch64::new(crate::convert::u64_to_f64(value))
            }
        }

        impl Conversion<signed::Ch12, Ch8> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch12) -> Ch8 {
                let value = crate::upscale::i12_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch8::new(crate::downscale::u32_to_u8(value))
            }
        }

        impl Conversion<signed::Ch12, Ch16> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch12) -> Ch16 {
                let value = crate::upscale::i12_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch16::new(crate::downscale::u32_to_u16(value))
            }
        }

        impl Conversion<signed::Ch12, Ch24> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch12) -> Ch24 {
                let value = crate::upscale::i12_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch24::new(crate::downscale::u32_to_u24(value))
            }
        }

        impl Conversion<signed::Ch12, Ch32> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch12) -> Ch32 {
                let value = crate::upscale::i12_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch32::new(crate::convert::u32_to_f32(value))
            }
        }

        impl Conversion<signed::Ch12, Ch64> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch12) -> Ch64 {
                let value = crate::upscale::i12_to_i64(value.into_inner());
                let value = crate::convert::i64_to_u64(value);

                Ch64::new(crate::convert::u64_to_f64(value))
            }
        }

        impl Conversion<signed::Ch16, Ch8> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch16) -> Ch8 {
                let value = crate::upscale::i16_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch8::new(crate::downscale::u32_to_u8(value))
            }
        }

        impl Conversion<signed::Ch16, Ch12> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch16) -> Ch12 {
                let value = crate::upscale::i16_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch12::new(crate::downscale::u32_to_u12(value))
            }
        }

        impl Conversion<signed::Ch16, Ch24> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch16) -> Ch24 {
                let value = crate::upscale::i16_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch24::new(crate::downscale::u32_to_u24(value))
            }
        }

        impl Conversion<signed::Ch16, Ch32> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch16) -> Ch32 {
                let value = crate::upscale::i16_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch32::new(crate::convert::u32_to_f32(value))
            }
        }

        impl Conversion<signed::Ch16, Ch64> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch16) -> Ch64 {
                let value = crate::upscale::i16_to_i64(value.into_inner());
                let value = crate::convert::i64_to_u64(value);

                Ch64::new(crate::convert::u64_to_f64(value))
            }
        }

        impl Conversion<signed::Ch24, Ch8> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch24) -> Ch8 {
                let value = crate::upscale::i24_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch8::new(crate::downscale::u32_to_u8(value))
            }
        }

        impl Conversion<signed::Ch24, Ch12> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch24) -> Ch12 {
                let value = crate::upscale::i24_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch12::new(crate::downscale::u32_to_u12(value))
            }
        }

        impl Conversion<signed::Ch24, Ch16> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch24) -> Ch16 {
                let value = crate::upscale::i24_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch16::new(crate::downscale::u32_to_u16(value))
            }
        }

        impl Conversion<signed::Ch24, Ch32> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch24) -> Ch32 {
                let value = crate::upscale::i24_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch32::new(crate::convert::u32_to_f32(value))
            }
        }

        impl Conversion<signed::Ch24, Ch64> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch24) -> Ch64 {
                let value = crate::upscale::i24_to_i64(value.into_inner());
                let value = crate::convert::i64_to_u64(value);

                Ch64::new(crate::convert::u64_to_f64(value))
            }
        }

        impl Conversion<signed::Ch32, Ch8> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch32) -> Ch8 {
                let value = crate::convert::f32_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch8::new(crate::downscale::u32_to_u8(value))
            }
        }

        impl Conversion<signed::Ch32, Ch12> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch32) -> Ch12 {
                let value = crate::convert::f32_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch12::new(crate::downscale::u32_to_u12(value))
            }
        }

        impl Conversion<signed::Ch32, Ch16> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch32) -> Ch16 {
                let value = crate::convert::f32_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch16::new(crate::downscale::u32_to_u16(value))
            }
        }

        impl Conversion<signed::Ch32, Ch24> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch32) -> Ch24 {
                let value = crate::convert::f32_to_i32(value.into_inner());
                let value = crate::convert::i32_to_u32(value);

                Ch24::new(crate::downscale::u32_to_u24(value))
            }
        }

        impl Conversion<signed::Ch32, Ch64> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch32) -> Ch64 {
                let value = crate::upscale::f32_to_f64(value.into_inner());
                let value = crate::convert::f64_to_i64(value);
                let value = crate::convert::i64_to_u64(value);

                Ch64::new(crate::convert::u64_to_f64(value))
            }
        }

        impl Conversion<signed::Ch64, Ch8> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch64) -> Ch8 {
                let value = crate::convert::f64_to_i64(value.into_inner());
                let value = crate::convert::i64_to_u64(value);

                Ch8::new(crate::downscale::u64_to_u8(value))
            }
        }

        impl Conversion<signed::Ch64, Ch12> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch64) -> Ch12 {
                let value = crate::convert::f64_to_i64(value.into_inner());
                let value = crate::convert::i64_to_u64(value);

                Ch12::new(crate::downscale::u64_to_u12(value))
            }
        }

        impl Conversion<signed::Ch64, Ch16> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch64) -> Ch16 {
                let value = crate::convert::f64_to_i64(value.into_inner());
                let value = crate::convert::i64_to_u64(value);

                Ch16::new(crate::downscale::u64_to_u16(value))
            }
        }

        impl Conversion<signed::Ch64, Ch24> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch64) -> Ch24 {
                let value = crate::convert::f64_to_i64(value.into_inner());
                let value = crate::convert::i64_to_u64(value);

                Ch24::new(crate::downscale::u64_to_u24(value))
            }
        }

        impl Conversion<signed::Ch64, Ch32> {
            /// Convert between types.
            pub const fn conv(value: signed::Ch64) -> Ch32 {
                let value = crate::downscale::f64_to_f32(value.into_inner());
                let value = crate::convert::f32_to_i32(value);
                let value = crate::convert::i32_to_u32(value);

                Ch32::new(crate::convert::u32_to_f32(value))
            }
        }
    }
}

#[cfg(feature = "signed")]
mod signed {
    use super::*;
    use crate::signed::{Ch8, Ch12, Ch16, Ch24, Ch32, Ch64};

    int_channel!(Ch8);
    int_channel!(Ch12);
    int_channel!(Ch16);
    int_channel!(Ch24);
    float_channel!(Ch32);
    float_channel!(Ch64);

    impl Inversion<Ch32> {
        /// Invert the value.
        pub const fn inv(self) -> Ch32 {
            Ch32::new(-self.0.into_inner())
        }
    }

    impl Inversion<Ch64> {
        /// Invert the value.
        pub const fn inv(self) -> Ch64 {
            Ch64::new(-self.0.into_inner())
        }
    }

    impl Inversion<Ch8> {
        /// Invert the value.
        pub const fn inv(self) -> Ch8 {
            Ch8::new(-1 - self.0.into_inner())
        }
    }

    impl Inversion<Ch12> {
        /// Invert the value.
        pub const fn inv(self) -> Ch12 {
            Ch12::new(-1 - self.0.into_inner())
        }
    }

    impl Inversion<Ch16> {
        /// Invert the value.
        pub const fn inv(self) -> Ch16 {
            Ch16::new(-1 - self.0.into_inner())
        }
    }

    impl Inversion<Ch24> {
        /// Invert the value.
        pub const fn inv(self) -> Ch24 {
            Ch24::new(-1 - self.0.into_inner())
        }
    }

    impl Negation<Ch8> {
        /// Negate the value.
        pub const fn neg(self) -> Ch8 {
            Ch8::new(-1 - self.0.into_inner())
        }
    }

    impl Negation<Ch12> {
        /// Negate the value.
        pub const fn neg(self) -> Ch12 {
            Ch12::new(-1 - self.0.into_inner())
        }
    }

    impl Negation<Ch16> {
        /// Negate the value.
        pub const fn neg(self) -> Ch16 {
            Ch16::new(-1 - self.0.into_inner())
        }
    }

    impl Negation<Ch24> {
        /// Negate the value.
        pub const fn neg(self) -> Ch24 {
            Ch24::new(-1 - self.0.into_inner())
        }
    }

    impl Conversion<Ch8, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch12 {
            let value = crate::upscale::i8_to_i32(value.into_inner());

            Ch12::new(crate::downscale::i32_to_i12(value))
        }
    }

    impl Conversion<Ch8, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch16 {
            let value = crate::upscale::i8_to_i32(value.into_inner());

            Ch16::new(crate::downscale::i32_to_i16(value))
        }
    }

    impl Conversion<Ch8, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch24 {
            let value = crate::upscale::i8_to_i32(value.into_inner());

            Ch24::new(crate::downscale::i32_to_i24(value))
        }
    }

    impl Conversion<Ch8, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch32 {
            let value = crate::upscale::i8_to_i32(value.into_inner());

            Ch32::new(crate::convert::i32_to_f32(value))
        }
    }

    impl Conversion<Ch8, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch8) -> Ch64 {
            let value = crate::upscale::i8_to_i64(value.into_inner());

            Ch64::new(crate::convert::i64_to_f64(value))
        }
    }

    impl Conversion<Ch12, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch8 {
            let value = crate::upscale::i12_to_i32(value.into_inner());

            Ch8::new(crate::downscale::i32_to_i8(value))
        }
    }

    impl Conversion<Ch12, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch16 {
            let value = crate::upscale::i12_to_i32(value.into_inner());

            Ch16::new(crate::downscale::i32_to_i16(value))
        }
    }

    impl Conversion<Ch12, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch24 {
            let value = crate::upscale::i12_to_i32(value.into_inner());

            Ch24::new(crate::downscale::i32_to_i24(value))
        }
    }

    impl Conversion<Ch12, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch32 {
            let value = crate::upscale::i12_to_i32(value.into_inner());

            Ch32::new(crate::convert::i32_to_f32(value))
        }
    }

    impl Conversion<Ch12, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch12) -> Ch64 {
            let value = crate::upscale::i12_to_i64(value.into_inner());

            Ch64::new(crate::convert::i64_to_f64(value))
        }
    }

    impl Conversion<Ch16, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch8 {
            let value = crate::upscale::i16_to_i32(value.into_inner());

            Ch8::new(crate::downscale::i32_to_i8(value))
        }
    }

    impl Conversion<Ch16, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch12 {
            let value = crate::upscale::i16_to_i32(value.into_inner());

            Ch12::new(crate::downscale::i32_to_i12(value))
        }
    }

    impl Conversion<Ch16, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch24 {
            let value = crate::upscale::i16_to_i32(value.into_inner());

            Ch24::new(crate::downscale::i32_to_i24(value))
        }
    }

    impl Conversion<Ch16, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch32 {
            let value = crate::upscale::i16_to_i32(value.into_inner());

            Ch32::new(crate::convert::i32_to_f32(value))
        }
    }

    impl Conversion<Ch16, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch16) -> Ch64 {
            let value = crate::upscale::i16_to_i64(value.into_inner());

            Ch64::new(crate::convert::i64_to_f64(value))
        }
    }

    impl Conversion<Ch24, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch8 {
            let value = crate::upscale::i24_to_i32(value.into_inner());

            Ch8::new(crate::downscale::i32_to_i8(value))
        }
    }

    impl Conversion<Ch24, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch12 {
            let value = crate::upscale::i24_to_i32(value.into_inner());

            Ch12::new(crate::downscale::i32_to_i12(value))
        }
    }

    impl Conversion<Ch24, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch16 {
            let value = crate::upscale::i24_to_i32(value.into_inner());

            Ch16::new(crate::downscale::i32_to_i16(value))
        }
    }

    impl Conversion<Ch24, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch32 {
            let value = crate::upscale::i24_to_i32(value.into_inner());

            Ch32::new(crate::convert::i32_to_f32(value))
        }
    }

    impl Conversion<Ch24, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch24) -> Ch64 {
            let value = crate::upscale::i24_to_i64(value.into_inner());

            Ch64::new(crate::convert::i64_to_f64(value))
        }
    }

    impl Conversion<Ch32, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch8 {
            let value = crate::convert::f32_to_i32(value.into_inner());

            Ch8::new(crate::downscale::i32_to_i8(value))
        }
    }

    impl Conversion<Ch32, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch12 {
            let value = crate::convert::f32_to_i32(value.into_inner());

            Ch12::new(crate::downscale::i32_to_i12(value))
        }
    }

    impl Conversion<Ch32, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch16 {
            let value = crate::convert::f32_to_i32(value.into_inner());

            Ch16::new(crate::downscale::i32_to_i16(value))
        }
    }

    impl Conversion<Ch32, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch24 {
            let value = crate::convert::f32_to_i32(value.into_inner());

            Ch24::new(crate::downscale::i32_to_i24(value))
        }
    }

    impl Conversion<Ch32, Ch64> {
        /// Convert between types.
        pub const fn conv(value: Ch32) -> Ch64 {
            let value = crate::upscale::f32_to_f64(value.into_inner());

            Ch64::new(value)
        }
    }

    impl Conversion<Ch64, Ch8> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch8 {
            let value = crate::convert::f64_to_i64(value.into_inner());

            Ch8::new(crate::downscale::i64_to_i8(value))
        }
    }

    impl Conversion<Ch64, Ch12> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch12 {
            let value = crate::convert::f64_to_i64(value.into_inner());

            Ch12::new(crate::downscale::i64_to_i12(value))
        }
    }

    impl Conversion<Ch64, Ch16> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch16 {
            let value = crate::convert::f64_to_i64(value.into_inner());

            Ch16::new(crate::downscale::i64_to_i16(value))
        }
    }

    impl Conversion<Ch64, Ch24> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch24 {
            let value = crate::convert::f64_to_i64(value.into_inner());

            Ch24::new(crate::downscale::i64_to_i24(value))
        }
    }

    impl Conversion<Ch64, Ch32> {
        /// Convert between types.
        pub const fn conv(value: Ch64) -> Ch32 {
            let value = crate::downscale::f64_to_f32(value.into_inner());

            Ch32::new(value)
        }
    }

    #[cfg(feature = "unsigned")]
    mod unsigned {
        use super::*;
        use crate::unsigned;

        impl Conversion<unsigned::Ch8, Ch8> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch8) -> Ch8 {
                let value = crate::upscale::u8_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch8::new(crate::downscale::i32_to_i8(value))
            }
        }

        impl Conversion<unsigned::Ch12, Ch12> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch12) -> Ch12 {
                let value = crate::upscale::u12_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch12::new(crate::downscale::i32_to_i12(value))
            }
        }

        impl Conversion<unsigned::Ch16, Ch16> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch16) -> Ch16 {
                let value = crate::upscale::u16_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch16::new(crate::downscale::i32_to_i16(value))
            }
        }

        impl Conversion<unsigned::Ch24, Ch24> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch24) -> Ch24 {
                let value = crate::upscale::u24_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch24::new(crate::downscale::i32_to_i24(value))
            }
        }

        impl Conversion<unsigned::Ch32, Ch32> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch32) -> Ch32 {
                let value = crate::convert::f32_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch32::new(crate::convert::i32_to_f32(value))
            }
        }

        impl Conversion<unsigned::Ch64, Ch64> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch64) -> Ch64 {
                let value = crate::convert::f64_to_u64(value.into_inner());
                let value = crate::convert::u64_to_i64(value);

                Ch64::new(crate::convert::i64_to_f64(value))
            }
        }

        impl Conversion<unsigned::Ch8, Ch12> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch8) -> Ch12 {
                let value = crate::upscale::u8_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch12::new(crate::downscale::i32_to_i12(value))
            }
        }

        impl Conversion<unsigned::Ch8, Ch16> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch8) -> Ch16 {
                let value = crate::upscale::u8_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch16::new(crate::downscale::i32_to_i16(value))
            }
        }

        impl Conversion<unsigned::Ch8, Ch24> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch8) -> Ch24 {
                let value = crate::upscale::u8_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch24::new(crate::downscale::i32_to_i24(value))
            }
        }

        impl Conversion<unsigned::Ch8, Ch32> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch8) -> Ch32 {
                let value = crate::upscale::u8_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch32::new(crate::convert::i32_to_f32(value))
            }
        }

        impl Conversion<unsigned::Ch8, Ch64> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch8) -> Ch64 {
                let value = crate::upscale::u8_to_u64(value.into_inner());
                let value = crate::convert::u64_to_i64(value);

                Ch64::new(crate::convert::i64_to_f64(value))
            }
        }

        impl Conversion<unsigned::Ch12, Ch8> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch12) -> Ch8 {
                let value = crate::upscale::u12_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch8::new(crate::downscale::i32_to_i8(value))
            }
        }

        impl Conversion<unsigned::Ch12, Ch16> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch12) -> Ch16 {
                let value = crate::upscale::u12_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch16::new(crate::downscale::i32_to_i16(value))
            }
        }

        impl Conversion<unsigned::Ch12, Ch24> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch12) -> Ch24 {
                let value = crate::upscale::u12_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch24::new(crate::downscale::i32_to_i24(value))
            }
        }

        impl Conversion<unsigned::Ch12, Ch32> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch12) -> Ch32 {
                let value = crate::upscale::u12_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch32::new(crate::convert::i32_to_f32(value))
            }
        }

        impl Conversion<unsigned::Ch12, Ch64> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch12) -> Ch64 {
                let value = crate::upscale::u12_to_u64(value.into_inner());
                let value = crate::convert::u64_to_i64(value);

                Ch64::new(crate::convert::i64_to_f64(value))
            }
        }

        impl Conversion<unsigned::Ch16, Ch8> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch16) -> Ch8 {
                let value = crate::upscale::u16_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch8::new(crate::downscale::i32_to_i8(value))
            }
        }

        impl Conversion<unsigned::Ch16, Ch12> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch16) -> Ch12 {
                let value = crate::upscale::u16_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch12::new(crate::downscale::i32_to_i12(value))
            }
        }

        impl Conversion<unsigned::Ch16, Ch24> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch16) -> Ch24 {
                let value = crate::upscale::u16_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch24::new(crate::downscale::i32_to_i24(value))
            }
        }

        impl Conversion<unsigned::Ch16, Ch32> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch16) -> Ch32 {
                let value = crate::upscale::u16_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch32::new(crate::convert::i32_to_f32(value))
            }
        }

        impl Conversion<unsigned::Ch16, Ch64> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch16) -> Ch64 {
                let value = crate::upscale::u16_to_u64(value.into_inner());
                let value = crate::convert::u64_to_i64(value);

                Ch64::new(crate::convert::i64_to_f64(value))
            }
        }

        impl Conversion<unsigned::Ch24, Ch8> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch24) -> Ch8 {
                let value = crate::upscale::u24_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch8::new(crate::downscale::i32_to_i8(value))
            }
        }

        impl Conversion<unsigned::Ch24, Ch12> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch24) -> Ch12 {
                let value = crate::upscale::u24_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch12::new(crate::downscale::i32_to_i12(value))
            }
        }

        impl Conversion<unsigned::Ch24, Ch16> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch24) -> Ch16 {
                let value = crate::upscale::u24_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch16::new(crate::downscale::i32_to_i16(value))
            }
        }

        impl Conversion<unsigned::Ch24, Ch32> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch24) -> Ch32 {
                let value = crate::upscale::u24_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch32::new(crate::convert::i32_to_f32(value))
            }
        }

        impl Conversion<unsigned::Ch24, Ch64> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch24) -> Ch64 {
                let value = crate::upscale::u24_to_u64(value.into_inner());
                let value = crate::convert::u64_to_i64(value);

                Ch64::new(crate::convert::i64_to_f64(value))
            }
        }

        impl Conversion<unsigned::Ch32, Ch8> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch32) -> Ch8 {
                let value = crate::convert::f32_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch8::new(crate::downscale::i32_to_i8(value))
            }
        }

        impl Conversion<unsigned::Ch32, Ch12> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch32) -> Ch12 {
                let value = crate::convert::f32_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch12::new(crate::downscale::i32_to_i12(value))
            }
        }

        impl Conversion<unsigned::Ch32, Ch16> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch32) -> Ch16 {
                let value = crate::convert::f32_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch16::new(crate::downscale::i32_to_i16(value))
            }
        }

        impl Conversion<unsigned::Ch32, Ch24> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch32) -> Ch24 {
                let value = crate::convert::f32_to_u32(value.into_inner());
                let value = crate::convert::u32_to_i32(value);

                Ch24::new(crate::downscale::i32_to_i24(value))
            }
        }

        impl Conversion<unsigned::Ch32, Ch64> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch32) -> Ch64 {
                let value = crate::upscale::f32_to_f64(value.into_inner());
                let value = crate::convert::f64_to_u64(value);
                let value = crate::convert::u64_to_i64(value);

                Ch64::new(crate::convert::i64_to_f64(value))
            }
        }

        impl Conversion<unsigned::Ch64, Ch8> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch64) -> Ch8 {
                let value = crate::convert::f64_to_u64(value.into_inner());
                let value = crate::convert::u64_to_i64(value);

                Ch8::new(crate::downscale::i64_to_i8(value))
            }
        }

        impl Conversion<unsigned::Ch64, Ch12> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch64) -> Ch12 {
                let value = crate::convert::f64_to_u64(value.into_inner());
                let value = crate::convert::u64_to_i64(value);

                Ch12::new(crate::downscale::i64_to_i12(value))
            }
        }

        impl Conversion<unsigned::Ch64, Ch16> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch64) -> Ch16 {
                let value = crate::convert::f64_to_u64(value.into_inner());
                let value = crate::convert::u64_to_i64(value);

                Ch16::new(crate::downscale::i64_to_i16(value))
            }
        }

        impl Conversion<unsigned::Ch64, Ch24> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch64) -> Ch24 {
                let value = crate::convert::f64_to_u64(value.into_inner());
                let value = crate::convert::u64_to_i64(value);

                Ch24::new(crate::downscale::i64_to_i24(value))
            }
        }

        impl Conversion<unsigned::Ch64, Ch32> {
            /// Convert between types.
            pub const fn conv(value: unsigned::Ch64) -> Ch32 {
                let value = crate::downscale::f64_to_f32(value.into_inner());
                let value = crate::convert::f32_to_u32(value);
                let value = crate::convert::u32_to_i32(value);

                Ch32::new(crate::convert::i32_to_f32(value))
            }
        }
    }
}
