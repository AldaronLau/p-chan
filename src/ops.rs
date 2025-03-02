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
//! This is the same as inversion for signed floating-point channels.
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

/// Constant conversion operation (`convert`)
///
///  - `Conversion::<_, U>::convert(_)`
#[derive(Debug)]
pub struct Conversion<T, U>(T, PhantomData<fn() -> U>);

/// Constant inversion operation (`invert`)
///
///  - `Inversion(_)::invert()`
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
macro_rules! float_channel {
    ($type:ty) => {
        impl<const N: usize> Sum<$type, N> {
            /// Add up the sum.
            pub const fn add(self) -> $type {
                let mut ret = 0.0;
                let mut i = 0;

                loop {
                    if i >= N {
                        break <$type>::new(ret).normalize();
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
                        break <$type>::new(ret).normalize();
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
                        break <$type>::new(ret).normalize();
                    }

                    let value = self.0[i];

                    ret *= value.into_inner();
                    i += 1;
                }
            }
        }
    };
}

#[cfg(feature = "unsigned")]
mod unsigned {
    use super::*;
    use crate::unsigned::{Ch32, Ch64};

    float_channel!(Ch32);
    float_channel!(Ch64);

    impl Inversion<Ch32> {
        /// Invert the value.
        pub const fn invert(self) -> Ch32 {
            Ch32::new(1.0 - self.0.into_inner())
        }
    }

    impl Inversion<Ch64> {
        /// Invert the value.
        pub const fn invert(self) -> Ch64 {
            Ch64::new(1.0 - self.0.into_inner())
        }
    }

    impl Negation<Ch32> {
        /// Negate the value.
        pub const fn neg(self) -> Ch32 {
            Ch32::new(-self.0.into_inner())
        }
    }

    impl Negation<Ch64> {
        /// Negate the value.
        pub const fn neg(self) -> Ch64 {
            Ch64::new(-self.0.into_inner())
        }
    }
}

#[cfg(feature = "signed")]
mod signed {
    use super::*;
    use crate::signed::{Ch8, Ch16, Ch32, Ch64};

    impl Conversion<Ch8, Ch16> {
        /// Convert between types.
        pub const fn convert(from: Ch8) -> Ch16 {
            let little = from.into_inner() as i16;
            let big = little * 256;

            Ch16::new(little + big)
        }
    }

    float_channel!(Ch32);
    float_channel!(Ch64);

    impl Inversion<Ch32> {
        /// Invert the value.
        pub const fn invert(self) -> Ch32 {
            Ch32::new(-self.0.into_inner())
        }
    }

    impl Inversion<Ch64> {
        /// Invert the value.
        pub const fn invert(self) -> Ch64 {
            Ch64::new(-self.0.into_inner())
        }
    }

    impl Negation<Ch32> {
        /// Negate the value.
        pub const fn neg(self) -> Ch32 {
            Ch32::new(-self.0.into_inner())
        }
    }

    impl Negation<Ch64> {
        /// Negate the value.
        pub const fn neg(self) -> Ch64 {
            Ch64::new(-self.0.into_inner())
        }
    }
}
