//! Unsigned channel newtypes

use crate::ops::Conversion;

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
    (Ch32, f32, crate::math::normalize_f32, 0.0, 0.5),
    doc = "32-bit float (0 to 1) channel value",
);

ch_float!(
    (Ch64, f64, crate::math::normalize_f64, 0.0, 0.5),
    doc = "64-bit float (0 to 1) channel value",
);

const fn normalize_ch12(mut chan: u16) -> u16 {
    if chan > 2_u16.pow(12) - 1 {
        chan = 2_u16.pow(12) - 1;
    }

    chan
}

const fn normalize_ch24(mut chan: u32) -> u32 {
    if chan > 2_u32.pow(24) - 1 {
        chan = 2_u32.pow(24) - 1;
    }

    chan
}

conversion!(Ch8, Ch12);
conversion!(Ch8, Ch16);
conversion!(Ch8, Ch24);
conversion!(Ch8, Ch32);
conversion!(Ch8, Ch64);

conversion!(Ch12, Ch8);
conversion!(Ch12, Ch16);
conversion!(Ch12, Ch24);
conversion!(Ch12, Ch32);
conversion!(Ch12, Ch64);

conversion!(Ch16, Ch8);
conversion!(Ch16, Ch12);
conversion!(Ch16, Ch24);
conversion!(Ch16, Ch32);
conversion!(Ch16, Ch64);

conversion!(Ch24, Ch8);
conversion!(Ch24, Ch12);
conversion!(Ch24, Ch16);
conversion!(Ch24, Ch32);
conversion!(Ch24, Ch64);

conversion!(Ch32, Ch8);
conversion!(Ch32, Ch12);
conversion!(Ch32, Ch16);
conversion!(Ch32, Ch24);
conversion!(Ch32, Ch64);

conversion!(Ch64, Ch8);
conversion!(Ch64, Ch12);
conversion!(Ch64, Ch16);
conversion!(Ch64, Ch24);
conversion!(Ch64, Ch32);

#[cfg(feature = "signed")]
mod signed {
    use super::*;
    use crate::signed;

    conversion!(signed::Ch8, Ch12);
    conversion!(signed::Ch8, Ch16);
    conversion!(signed::Ch8, Ch24);
    conversion!(signed::Ch8, Ch32);
    conversion!(signed::Ch8, Ch64);

    conversion!(signed::Ch12, Ch8);
    conversion!(signed::Ch12, Ch16);
    conversion!(signed::Ch12, Ch24);
    conversion!(signed::Ch12, Ch32);
    conversion!(signed::Ch12, Ch64);

    conversion!(signed::Ch16, Ch8);
    conversion!(signed::Ch16, Ch12);
    conversion!(signed::Ch16, Ch24);
    conversion!(signed::Ch16, Ch32);
    conversion!(signed::Ch16, Ch64);

    conversion!(signed::Ch24, Ch8);
    conversion!(signed::Ch24, Ch12);
    conversion!(signed::Ch24, Ch16);
    conversion!(signed::Ch24, Ch32);
    conversion!(signed::Ch24, Ch64);

    conversion!(signed::Ch32, Ch8);
    conversion!(signed::Ch32, Ch12);
    conversion!(signed::Ch32, Ch16);
    conversion!(signed::Ch32, Ch24);
    conversion!(signed::Ch32, Ch64);

    conversion!(signed::Ch64, Ch8);
    conversion!(signed::Ch64, Ch12);
    conversion!(signed::Ch64, Ch16);
    conversion!(signed::Ch64, Ch24);
    conversion!(signed::Ch64, Ch32);

    conversion!(signed::Ch8, Ch8);
    conversion!(signed::Ch12, Ch12);
    conversion!(signed::Ch16, Ch16);
    conversion!(signed::Ch24, Ch24);
    conversion!(signed::Ch32, Ch32);
    conversion!(signed::Ch64, Ch64);
}
