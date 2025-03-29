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
}

#[cfg(feature = "signed")]
mod signed {
    use super::*;
    use crate::signed::{Ch8, Ch12, Ch16, Ch24, Ch32, Ch64};

    // FIXME: conversion

    impl Conversion<Ch8, Ch16> {
        /// Convert between types.
        pub const fn conv(from: Ch8) -> Ch16 {
            let little = from.into_inner() as i16;
            let big = little * 256;

            Ch16::new(little + big)
        }
    }

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
}
