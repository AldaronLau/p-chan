//! Constant "trait" operations
//!
//! This module provides wrapper structs for performing operations on channel
//! values (which are `const` compatible).

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

/// Constant conversion operation (`into`)
///
///  - `Convert::<_, U>(_, []).into()`
#[derive(Debug)]
pub struct Convert<T, U>(T, PhantomData<fn() -> U>);

#[cfg(feature = "signed")]
mod signed {
    use super::*;
    use crate::signed::{Ch8, Ch16, Ch32};

    impl Convert<Ch8, Ch16> {
        /// Convert between types.
        pub const fn from(from: Ch8) -> Ch16 {
            let little = from.into_inner() as i16;
            let big = little * 256;

            Ch16::new(little + big)
        }
    }

    impl<const N: usize> Sum<Ch32, N> {
        /// Add up the sum.
        pub const fn add(self) -> Ch32 {
            let mut ret = 0.0;
            let mut i = 0;

            loop {
                if i >= N {
                    break Ch32::new(ret);
                }

                let value = self.0[i];

                ret += value.into_inner();
                i += 1;
            }
        }
    }
}
