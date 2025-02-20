macro_rules! ops_int {
    ($ty: ident, $p: ty, $b: ty, $normalize: path) => {
        impl From<$p> for $ty {
            fn from(value: $p) -> Self {
                Self::new(value)
            }
        }

        impl From<$ty> for $p {
            fn from(chan: $ty) -> Self {
                chan.0
            }
        }

        impl $ty {
            /// Maximum value
            pub const MAX: Self = Self::new(<$p>::MAX);
            /// Middle value
            pub const MID: Self = Self::new(<$p>::MAX >> 1);
            /// Minimum value
            pub const MIN: Self = Self::new(<$p>::MIN);

            /// Create a new channel value.
            pub const fn new(value: $p) -> Self {
                Self(value)
            }
        }

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

macro_rules! ch_float {
    (
        ($ty: ident, $p: ty, $normalize: path, $min: literal, $mid: literal),
        $docs: meta $(,)?
    ) => {
        #[$docs]
        #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
        #[repr(transparent)]
        pub struct $ty($p);

        impl From<$p> for $ty {
            fn from(value: $p) -> Self {
                Self::new(value)
            }
        }

        impl From<$ty> for $p {
            fn from(chan: $ty) -> Self {
                chan.0
            }
        }

        impl $ty {
            /// Maximum value
            pub const MAX: Self = Self::new(1.0);
            /// Middle value
            pub const MID: Self = Self::new($mid);
            /// Minimum value
            pub const MIN: Self = Self::new($min);

            /// Create a new channel value.
            pub const fn new(value: $p) -> Self {
                Self(value)
            }
        }

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
