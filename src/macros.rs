/// Declare a newtype channel wrapper.
///
/// ```
/// p_chan::channel!(
///     (Ch8, p_chan::unsigned::Ch8),
///     doc = "8-bit channel",
/// );
/// ```
#[macro_export]
macro_rules! channel {
    (($ty: ident, $inner: ty), $attr: meta $(,)?) => {
        #[$attr]
        #[derive(
            Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default,
        )]
        #[repr(transparent)]
        pub struct $ty($inner);

        impl $ty {
            fn __() {
                struct Impl<T>(core::marker::PhantomData<fn() -> T>)
                where
                    T: $crate::bytemuck::Pod + $crate::bytemuck::Zeroable;

                const INNER: Impl<$inner> = Impl(core::marker::PhantomData);
            }
        }

        impl From<$inner> for $ty {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl From<$ty> for $inner {
            fn from(chan: $ty) -> Self {
                chan.0
            }
        }

        #[allow(unsafe_code)]
        unsafe impl $crate::bytemuck::Zeroable for $ty {}

        #[allow(unsafe_code)]
        unsafe impl $crate::bytemuck::Pod for $ty {}
    };
}

/// Declare a channel group type.
///
/// ```
/// p_chan::group!(
///     (Pixel),
///     doc = "A pixel, made up of individual color channels",
/// );
///
/// p_chan::group!(
///     (Frame),
///     doc = "An audio frame, made up of individual speaker channels",
/// );
/// ```
#[macro_export]
macro_rules! group {
    (($ty: ident), $attr: meta $(,)?) => {
        #[$attr]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[repr(transparent)]
        pub struct $ty<Chan, const CH: usize>([Chan; CH]);

        impl<Chan, const CH: usize> Default for $ty<Chan, CH>
        where
            Chan: Default + Copy,
        {
            fn default() -> Self {
                Self([Chan::default(); CH])
            }
        }

        #[allow(unsafe_code)]
        unsafe impl<Chan, const CH: usize> $crate::bytemuck::Zeroable
            for $ty<Chan, CH>
        where
            Chan: $crate::bytemuck::Zeroable,
        {
        }

        #[allow(unsafe_code)]
        unsafe impl<Chan, const CH: usize> $crate::bytemuck::Pod
            for $ty<Chan, CH>
        where
            Chan: $crate::bytemuck::Pod,
        {
        }
    };
}

macro_rules! ch_int {
    (
        ($ty: ident, $p: ty, $b: ty, $normalize: path, $midpoint: item),
        $docs: meta $(,)?
    ) => {
        #[$docs]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
        #[repr(transparent)]
        pub struct $ty($p);

        impl core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
            {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl core::fmt::Display for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
            {
                core::fmt::Display::fmt(&self.0, f)
            }
        }

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

            pub(crate) const fn int_multiply(&mut self, rhs: Self) {
                let this = self.into_inner() as $b;
                let rhs = rhs.into_inner() as $b;
                let min = Self::MIN.into_inner() as $b;
                let max = Self::MAX.into_inner() as $b;
                let mut res = (this * rhs) / max;

                if res < min {
                    res = min;
                }

                if res > max {
                    res = max;
                }

                *self = Self(res as $p);
            }
        }

        impl core::ops::Add for $ty {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                crate::ops::Sum([self, rhs]).add()
            }
        }

        impl core::ops::AddAssign for $ty {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl core::ops::Sub for $ty {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                crate::ops::Difference(self, [rhs]).sub()
            }
        }

        impl core::ops::SubAssign for $ty {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl core::ops::Neg for $ty {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                crate::ops::Negation(self).neg()
            }
        }

        impl core::ops::Not for $ty {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
                crate::ops::Inversion(self).inv()
            }
        }

        impl core::ops::Mul for $ty {
            type Output = Self;

            #[inline(always)]
            fn mul(mut self, rhs: Self) -> Self {
                self.int_multiply(rhs);
                self
            }
        }

        impl core::ops::MulAssign for $ty {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        #[allow(unsafe_code)]
        unsafe impl bytemuck::Zeroable for $ty {}

        #[allow(unsafe_code)]
        unsafe impl bytemuck::Pod for $ty {}
    };
}

macro_rules! ch_float {
    (
        ($ty: ident, $p: ty, $normalize: path, $min: literal, $mid: literal),
        $docs: meta $(,)?
    ) => {
        #[$docs]
        #[derive(Copy, Clone, Default)]
        #[repr(transparent)]
        pub struct $ty($p);

        impl core::cmp::PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                $normalize(self.into_inner()) == $normalize(other.into_inner())
            }
        }

        impl core::cmp::Ord for $ty {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                $normalize(self.into_inner())
                    .partial_cmp(&$normalize(other.into_inner()))
                    .unwrap()
            }
        }

        impl core::cmp::Eq for $ty {}

        impl core::cmp::PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl core::fmt::Debug for $ty {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::fmt::Display for $ty {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

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
                crate::ops::Sum([self, rhs]).add()
            }
        }

        impl core::ops::AddAssign for $ty {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl core::ops::Sub for $ty {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                crate::ops::Difference(self, [rhs]).sub()
            }
        }

        impl core::ops::SubAssign for $ty {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl core::ops::Neg for $ty {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                crate::ops::Negation(self).neg()
            }
        }

        impl core::ops::Not for $ty {
            type Output = Self;

            #[inline(always)]
            fn not(self) -> Self {
                crate::ops::Inversion(self).inv()
            }
        }

        impl core::ops::Mul for $ty {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                crate::ops::Product([self, rhs]).mul()
            }
        }

        impl core::ops::MulAssign for $ty {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        #[allow(unsafe_code)]
        unsafe impl bytemuck::Zeroable for $ty {}

        #[allow(unsafe_code)]
        unsafe impl bytemuck::Pod for $ty {}
    };
}

macro_rules! conversion {
    ($from:ty, $into:ty) => {
        impl From<$from> for $into {
            fn from(value: $from) -> $into {
                Conversion::<$from, $into>::conv(value)
            }
        }
    };
}
