#![allow(dead_code)]

macro_rules! reinterpret {
    ($wrapper: ident, $from: ty, $to: ty) => {
        impl $wrapper<$from> {
            #[inline(always)]
            pub(super) const fn reinterpret(self) -> $to {
                <$to>::from_ne_bytes(self.0.to_ne_bytes())
            }

            #[inline(always)]
            pub(super) const fn reinterpret_with_offset(self) -> $to {
                self.reinterpret() ^ (1 << (<$from>::BITS - 1))
            }
        }
    };
}

pub(super) struct Unsigned<T>(pub(super) T);

reinterpret!(Unsigned, i8, u8);
reinterpret!(Unsigned, i16, u16);
reinterpret!(Unsigned, i32, u32);
reinterpret!(Unsigned, i64, u64);
reinterpret!(Unsigned, i128, u128);

pub(super) struct Signed<T>(pub(super) T);

reinterpret!(Signed, u8, i8);
reinterpret!(Signed, u16, i16);
reinterpret!(Signed, u32, i32);
reinterpret!(Signed, u64, i64);
reinterpret!(Signed, u128, i128);

#[inline(always)]
pub(super) const fn word(b: bool) -> i32 {
    b as i32
}

#[inline(always)]
pub(super) const fn long(b: bool) -> i64 {
    b as i64
}

/// Flush subnormals and NaN to zero
#[inline(always)]
pub(super) const fn normalize_f32(float: f32) -> f32 {
    let no_flush =
        Unsigned(-word(float.is_normal() | float.is_infinite())).reinterpret();

    f32::from_bits(float.to_bits() & no_flush)
}

/// Flush subnormals and NaN to zero
#[inline(always)]
pub(super) const fn normalize_f64(float: f64) -> f64 {
    let no_flush =
        Unsigned(-long(float.is_normal() | float.is_infinite())).reinterpret();

    f64::from_bits(float.to_bits() & no_flush)
}
