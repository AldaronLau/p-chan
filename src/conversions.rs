#![allow(dead_code)]

macro_rules! reinterpret {
    ($wrapper: ident, $from: ty, $to: ty) => {
        impl $wrapper<$from> {
            #[inline(always)]
            pub(super) const fn reinterpret(self) -> $to {
                <$to>::from_ne_bytes(self.0.to_ne_bytes())
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
