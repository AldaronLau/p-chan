macro_rules! ch_int {
    (
        ($ty: ident, $p: ty, $b: ty, $normalize: path, $midpoint: item),
        $docs: meta $(,)?
    ) => {
        #[$docs]
        #[derive(
            Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default,
        )]
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
            pub const MAX: Self = Self::new($normalize(<$p>::MAX));
            /// Middle value
            pub const MID: Self = Self::MAX.midpoint(Self::MIN);
            /// Minimum value
            pub const MIN: Self = Self::new($normalize(<$p>::MIN));

            /// Create a new channel value.
            pub const fn new(value: $p) -> Self {
                Self($normalize(value))
            }

            /// Get the inner primitive channel value.
            pub const fn into_inner(self) -> $p {
                self.0
            }

            $midpoint

            /// Returns `max` if `self` is greater than `max`, and `min` if
            /// `self` is less than `min`. Otherwise this returns `self`.
            ///
            /// # Panics
            ///
            /// Panics if `min > max`.
            pub const fn clamp(self, min: Self, max: Self) -> Self {
                let (mut this, min, max) = (self.0, min.0, max.0);

                assert!(min <= max, "min > max");

                if this < min {
                    this = min;
                }

                if this > max {
                    this = max;
                }

                Self(this)
            }
        }

        impl core::ops::Add for $ty {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self($normalize(self.0.saturating_add(rhs.0)))
            }
        }

        impl core::ops::Sub for $ty {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self($normalize(self.0.saturating_sub(rhs.0)))
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
                Self($normalize(value))
            }

            /// Get the inner primitive channel value.
            pub const fn into_inner(self) -> $p {
                self.0
            }

            /// Calculates the middle point of `self` and `rhs` (clamped).
            ///
            /// `midpoint(a, b)` is `(a + b) / 2`.
            pub const fn midpoint(self, rhs: Self) -> Self {
                // Overflow is impossible since maximum value is 1 (would need
                // to be over (float::MAX / 2.0)
                Self($normalize((self.0 + rhs.0) / 2.0))
            }

            /// Returns `max` if `self` is greater than `max`, and `min` if
            /// `self` is less than `min`. Otherwise this returns `self`.
            ///
            /// # Panics
            ///
            /// Panics if `min > max`.
            pub const fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.clamp(min.0, max.0))
            }
        }

        impl core::ops::Add for $ty {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self($normalize(self.0 + rhs.0))
            }
        }

        impl core::ops::Sub for $ty {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self($normalize(self.0 - rhs.0))
            }
        }
    };
}
