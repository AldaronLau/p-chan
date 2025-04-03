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

impl From<Ch8> for Ch16 {
    fn from(value: Ch8) -> Ch16 {
        Conversion::<Ch8, Ch16>::conv(value)
    }
}
