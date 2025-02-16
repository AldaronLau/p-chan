macro_rules! ops_int {
    ($ty: ty, $p: ty, $b: ty, $normalize: path) => {
        impl $ty {
            /// Minimum value
            pub const MIN: Self = Self::new(<$p>::MIN);
            /// Middle value
            pub const MID: Self = Self::new(<$p>::MAX >> 1);
            /// Maximum value
            pub const MAX: Self = Self::new(<$p>::MAX);

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

macro_rules! ops_float {
    ($ty: ty, $p: ty, $normalize: path, $min: literal, $mid: literal) => {
        impl $ty {
            /// Minimum value
            pub const MIN: Self = Self::new($min);
            /// Middle value
            pub const MID: Self = Self::new($mid);
            /// Maximum value
            pub const MAX: Self = Self::new(1.0);

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
