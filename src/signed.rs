//! Signed channel newtypes

use crate::ops::Conversion;

macro_rules! midpoint {
    () => {
        /// Calculates the middle point of `self` and `rhs`.
        ///
        /// `midpoint(a, b)` is `(a + b) / 2` calculated without overflow,
        /// rounded down.
        pub const fn midpoint(self, rhs: Self) -> Self {
            use crate::math::{Signed, Unsigned};

            let this = Unsigned(self.0).reinterpret_with_offset();
            let rhs = Unsigned(rhs.0).reinterpret_with_offset();

            Self(Signed(this.midpoint(rhs)).reinterpret_with_offset())
        }
    };
}

ch_int!(
    (Ch8, i8, i16, core::convert::identity, midpoint! {}),
    doc = "8-bit signed integer (-128 to 127) channel value",
);

ch_int!(
    (Ch12, i16, i32, normalize_ch12, midpoint! {}),
    doc = "12-bit signed integer (-2\\_048 to 2\\_047) channel value",
);

ch_int!(
    (Ch16, i16, i32, core::convert::identity, midpoint! {}),
    doc = "16-bit signed integer (-32\\_768 to 32\\_767) channel value",
);

ch_int!(
    (Ch24, i32, i64, normalize_ch24, midpoint! {}),
    doc =
        "24-bit signed integer (-8\\_388\\_608 to 8\\_388\\_607) channel value",
);

ch_float!(
    (Ch32, f32, crate::math::normalize_f32, -1.0, 0.0),
    doc = "32-bit float (-1 to 1) channel value",
);

ch_float!(
    (Ch64, f64, crate::math::normalize_f64, -1.0, 0.0),
    doc = "64-bit float (-1 to 1) channel value",
);

const fn normalize_ch12(mut chan: i16) -> i16 {
    if chan > 2_i16.pow(11) - 1 {
        chan = 2_i16.pow(11) - 1;
    }

    if chan < -2_i16.pow(11) {
        chan = -2_i16.pow(11);
    }

    chan
}

const fn normalize_ch24(mut chan: i32) -> i32 {
    if chan > 2_i32.pow(23) - 1 {
        chan = 2_i32.pow(23) - 1;
    }

    if chan < -2_i32.pow(23) {
        chan = -2_i32.pow(23);
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

#[cfg(feature = "unsigned")]
mod unsigned {
    use crate::unsigned;
    use super::*;

    conversion!(unsigned::Ch8, Ch12);
    conversion!(unsigned::Ch8, Ch16);
    conversion!(unsigned::Ch8, Ch24);
    conversion!(unsigned::Ch8, Ch32);
    conversion!(unsigned::Ch8, Ch64);

    conversion!(unsigned::Ch12, Ch8);
    conversion!(unsigned::Ch12, Ch16);
    conversion!(unsigned::Ch12, Ch24);
    conversion!(unsigned::Ch12, Ch32);
    conversion!(unsigned::Ch12, Ch64);

    conversion!(unsigned::Ch16, Ch8);
    conversion!(unsigned::Ch16, Ch12);
    conversion!(unsigned::Ch16, Ch24);
    conversion!(unsigned::Ch16, Ch32);
    conversion!(unsigned::Ch16, Ch64);

    conversion!(unsigned::Ch24, Ch8);
    conversion!(unsigned::Ch24, Ch12);
    conversion!(unsigned::Ch24, Ch16);
    conversion!(unsigned::Ch24, Ch32);
    conversion!(unsigned::Ch24, Ch64);

    conversion!(unsigned::Ch32, Ch8);
    conversion!(unsigned::Ch32, Ch12);
    conversion!(unsigned::Ch32, Ch16);
    conversion!(unsigned::Ch32, Ch24);
    conversion!(unsigned::Ch32, Ch64);

    conversion!(unsigned::Ch64, Ch8);
    conversion!(unsigned::Ch64, Ch12);
    conversion!(unsigned::Ch64, Ch16);
    conversion!(unsigned::Ch64, Ch24);
    conversion!(unsigned::Ch64, Ch32);

    conversion!(unsigned::Ch8, Ch8);
    conversion!(unsigned::Ch12, Ch12);
    conversion!(unsigned::Ch16, Ch16);
    conversion!(unsigned::Ch24, Ch24);
    conversion!(unsigned::Ch32, Ch32);
    conversion!(unsigned::Ch64, Ch64);
}
