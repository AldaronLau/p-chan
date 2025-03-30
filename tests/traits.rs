use std::{
    fmt::{Debug, Display},
    ops::{Add, Neg, Not, Sub},
};

#[test]
fn impls_traits() {
    // FIXME: Add `Hash`, `Ord`, `Eq`

    trait AllTraits:
        Add
        + Sub
        + Not
        + Neg
        + Clone
        + Copy
        + Debug
        + Display
        + Default
        + PartialEq
        + PartialOrd
        + Eq
        + Ord
        + bytemuck::Zeroable
        + bytemuck::Pod
    {
    }

    impl<T> AllTraits for T where
        T: Add
            + Sub
            + Not
            + Neg
            + Clone
            + Copy
            + Debug
            + Display
            + Default
            + PartialEq
            + PartialOrd
            + Eq
            + Ord
            + bytemuck::Zeroable
            + bytemuck::Pod
    {
    }

    fn assert_impl<T: AllTraits>() {}

    // unsigned
    assert_impl::<p_chan::unsigned::Ch8>();
    assert_impl::<p_chan::unsigned::Ch12>();
    assert_impl::<p_chan::unsigned::Ch16>();
    assert_impl::<p_chan::unsigned::Ch24>();
    assert_impl::<p_chan::unsigned::Ch32>();
    assert_impl::<p_chan::unsigned::Ch64>();
    // signed
    assert_impl::<p_chan::signed::Ch8>();
    assert_impl::<p_chan::signed::Ch12>();
    assert_impl::<p_chan::signed::Ch16>();
    assert_impl::<p_chan::signed::Ch24>();
    assert_impl::<p_chan::signed::Ch32>();
    assert_impl::<p_chan::signed::Ch64>();
}

#[test]
fn float_ops_add() {
    assert_eq!(
        p_chan::unsigned::Ch32::new(0.5),
        p_chan::unsigned::Ch32::new(0.4) + p_chan::unsigned::Ch32::new(0.1),
    );
    assert_eq!(
        p_chan::signed::Ch32::new(0.5),
        p_chan::signed::Ch32::new(0.4) + p_chan::signed::Ch32::new(0.1),
    );
    assert_eq!(
        p_chan::unsigned::Ch64::new(0.5),
        p_chan::unsigned::Ch64::new(0.4) + p_chan::unsigned::Ch64::new(0.1),
    );
    assert_eq!(
        p_chan::signed::Ch64::new(0.5),
        p_chan::signed::Ch64::new(0.4) + p_chan::signed::Ch64::new(0.1),
    );
}

#[test]
fn float_ops_sub() {
    assert_eq!(
        p_chan::unsigned::Ch32::new(0.4),
        p_chan::unsigned::Ch32::new(0.5) - p_chan::unsigned::Ch32::new(0.1),
    );
    assert_eq!(
        p_chan::signed::Ch32::new(0.4),
        p_chan::signed::Ch32::new(0.5) - p_chan::signed::Ch32::new(0.1),
    );
    assert_eq!(
        p_chan::unsigned::Ch64::new(0.4),
        p_chan::unsigned::Ch64::new(0.5) - p_chan::unsigned::Ch64::new(0.1),
    );
    assert_eq!(
        p_chan::signed::Ch64::new(0.4),
        p_chan::signed::Ch64::new(0.5) - p_chan::signed::Ch64::new(0.1),
    );
}

#[test]
fn int_ops_add() {
    assert_eq!(
        p_chan::unsigned::Ch8::new(150),
        p_chan::unsigned::Ch8::new(42) + p_chan::unsigned::Ch8::new(108),
    );
    assert_eq!(
        p_chan::signed::Ch8::new(66),
        p_chan::signed::Ch8::new(-42) + p_chan::signed::Ch8::new(108),
    );
    assert_eq!(
        p_chan::unsigned::Ch12::new(150),
        p_chan::unsigned::Ch12::new(42) + p_chan::unsigned::Ch12::new(108),
    );
    assert_eq!(
        p_chan::signed::Ch12::new(66),
        p_chan::signed::Ch12::new(-42) + p_chan::signed::Ch12::new(108),
    );
    assert_eq!(
        p_chan::unsigned::Ch16::new(150),
        p_chan::unsigned::Ch16::new(42) + p_chan::unsigned::Ch16::new(108),
    );
    assert_eq!(
        p_chan::signed::Ch16::new(66),
        p_chan::signed::Ch16::new(-42) + p_chan::signed::Ch16::new(108),
    );
    assert_eq!(
        p_chan::unsigned::Ch24::new(150),
        p_chan::unsigned::Ch24::new(42) + p_chan::unsigned::Ch24::new(108),
    );
    assert_eq!(
        p_chan::signed::Ch24::new(66),
        p_chan::signed::Ch24::new(-42) + p_chan::signed::Ch24::new(108),
    );
    // saturation
    assert_eq!(
        p_chan::unsigned::Ch8::MAX,
        p_chan::unsigned::Ch8::new(150) + p_chan::unsigned::Ch8::new(150),
    );
    assert_eq!(
        p_chan::signed::Ch8::MIN,
        p_chan::signed::Ch8::new(-108) + p_chan::signed::Ch8::new(-108),
    );
    assert_eq!(
        p_chan::unsigned::Ch12::MAX,
        p_chan::unsigned::Ch12::new(3000) + p_chan::unsigned::Ch12::new(3000),
    );
    assert_eq!(
        p_chan::signed::Ch12::MIN,
        p_chan::signed::Ch12::new(-1080) + p_chan::signed::Ch12::new(-1080),
    );
    assert_eq!(
        p_chan::unsigned::Ch16::MAX,
        p_chan::unsigned::Ch16::new(40_000)
            + p_chan::unsigned::Ch16::new(40_000),
    );
    assert_eq!(
        p_chan::signed::Ch16::MIN,
        p_chan::signed::Ch16::new(-30_000) + p_chan::signed::Ch16::new(-30_000),
    );
    assert_eq!(
        p_chan::unsigned::Ch24::MAX,
        p_chan::unsigned::Ch24::new(9_000_000)
            + p_chan::unsigned::Ch24::new(9_000_000),
    );
    assert_eq!(
        p_chan::signed::Ch24::MIN,
        p_chan::signed::Ch24::new(-6_000_000)
            + p_chan::signed::Ch24::new(-6_000_000),
    );
}

#[test]
fn int_ops_sub() {
    assert_eq!(
        p_chan::unsigned::Ch8::new(42),
        p_chan::unsigned::Ch8::new(150) - p_chan::unsigned::Ch8::new(108),
    );
    assert_eq!(
        p_chan::signed::Ch8::new(-42),
        p_chan::signed::Ch8::new(66) - p_chan::signed::Ch8::new(108),
    );
    assert_eq!(
        p_chan::unsigned::Ch12::new(42),
        p_chan::unsigned::Ch12::new(150) - p_chan::unsigned::Ch12::new(108),
    );
    assert_eq!(
        p_chan::signed::Ch12::new(-42),
        p_chan::signed::Ch12::new(66) - p_chan::signed::Ch12::new(108),
    );
    assert_eq!(
        p_chan::unsigned::Ch16::new(42),
        p_chan::unsigned::Ch16::new(150) - p_chan::unsigned::Ch16::new(108),
    );
    assert_eq!(
        p_chan::signed::Ch16::new(-42),
        p_chan::signed::Ch16::new(66) - p_chan::signed::Ch16::new(108),
    );
    assert_eq!(
        p_chan::unsigned::Ch24::new(42),
        p_chan::unsigned::Ch24::new(150) - p_chan::unsigned::Ch24::new(108),
    );
    assert_eq!(
        p_chan::signed::Ch24::new(-42),
        p_chan::signed::Ch24::new(66) - p_chan::signed::Ch24::new(108),
    );
    // saturation
    assert_eq!(
        p_chan::unsigned::Ch8::MIN,
        p_chan::unsigned::Ch8::new(150) - p_chan::unsigned::Ch8::new(151),
    );
    assert_eq!(
        p_chan::signed::Ch8::MIN,
        p_chan::signed::Ch8::new(-108) - p_chan::signed::Ch8::new(108),
    );
    assert_eq!(
        p_chan::unsigned::Ch12::MIN,
        p_chan::unsigned::Ch12::new(3000) - p_chan::unsigned::Ch12::new(3001),
    );
    assert_eq!(
        p_chan::signed::Ch12::MIN,
        p_chan::signed::Ch12::new(-1080) - p_chan::signed::Ch12::new(1080),
    );
    assert_eq!(
        p_chan::unsigned::Ch16::MIN,
        p_chan::unsigned::Ch16::new(40_000)
            - p_chan::unsigned::Ch16::new(40_001),
    );
    assert_eq!(
        p_chan::signed::Ch16::MIN,
        p_chan::signed::Ch16::new(-30_000) - p_chan::signed::Ch16::new(30_000),
    );
    assert_eq!(
        p_chan::unsigned::Ch24::MIN,
        p_chan::unsigned::Ch24::new(9_000_000)
            - p_chan::unsigned::Ch24::new(9_000_001),
    );
    assert_eq!(
        p_chan::signed::Ch24::MIN,
        p_chan::signed::Ch24::new(-6_000_000)
            - p_chan::signed::Ch24::new(6_000_000),
    );
}

#[test]
fn ops_neg() {
    assert_eq!(-p_chan::signed::Ch8::MIN, p_chan::signed::Ch8::MAX);
    assert_eq!(-p_chan::signed::Ch12::MIN, p_chan::signed::Ch12::MAX);
    assert_eq!(-p_chan::signed::Ch16::MIN, p_chan::signed::Ch16::MAX);
    assert_eq!(-p_chan::signed::Ch24::MIN, p_chan::signed::Ch24::MAX);
    assert_eq!(-p_chan::signed::Ch32::MIN, p_chan::signed::Ch32::MAX);
    assert_eq!(-p_chan::signed::Ch64::MIN, p_chan::signed::Ch64::MAX);
    assert_eq!(-p_chan::signed::Ch8::MAX, p_chan::signed::Ch8::MIN);
    assert_eq!(-p_chan::signed::Ch12::MAX, p_chan::signed::Ch12::MIN);
    assert_eq!(-p_chan::signed::Ch16::MAX, p_chan::signed::Ch16::MIN);
    assert_eq!(-p_chan::signed::Ch24::MAX, p_chan::signed::Ch24::MIN);
    assert_eq!(-p_chan::signed::Ch32::MAX, p_chan::signed::Ch32::MIN);
    assert_eq!(-p_chan::signed::Ch64::MAX, p_chan::signed::Ch64::MIN);

    assert_eq!(-p_chan::unsigned::Ch8::MIN, p_chan::unsigned::Ch8::MAX);
    assert_eq!(-p_chan::unsigned::Ch12::MIN, p_chan::unsigned::Ch12::MAX);
    assert_eq!(-p_chan::unsigned::Ch16::MIN, p_chan::unsigned::Ch16::MAX);
    assert_eq!(-p_chan::unsigned::Ch24::MIN, p_chan::unsigned::Ch24::MAX);
    assert_eq!((-p_chan::unsigned::Ch32::MIN).into_inner(), 0.0);
    assert_eq!((-p_chan::unsigned::Ch64::MIN).into_inner(), 0.0);
    assert_eq!(-p_chan::unsigned::Ch8::MAX, p_chan::unsigned::Ch8::MIN);
    assert_eq!(-p_chan::unsigned::Ch12::MAX, p_chan::unsigned::Ch12::MIN);
    assert_eq!(-p_chan::unsigned::Ch16::MAX, p_chan::unsigned::Ch16::MIN);
    assert_eq!(-p_chan::unsigned::Ch24::MAX, p_chan::unsigned::Ch24::MIN);
    assert_eq!((-p_chan::unsigned::Ch32::MAX).into_inner(), -1.0);
    assert_eq!((-p_chan::unsigned::Ch64::MAX).into_inner(), -1.0);
}

#[test]
fn ops_inv() {
    assert_eq!(!p_chan::signed::Ch8::MIN, p_chan::signed::Ch8::MAX);
    assert_eq!(!p_chan::signed::Ch12::MIN, p_chan::signed::Ch12::MAX);
    assert_eq!(!p_chan::signed::Ch16::MIN, p_chan::signed::Ch16::MAX);
    assert_eq!(!p_chan::signed::Ch24::MIN, p_chan::signed::Ch24::MAX);
    assert_eq!(!p_chan::signed::Ch32::MIN, p_chan::signed::Ch32::MAX);
    assert_eq!(!p_chan::signed::Ch64::MIN, p_chan::signed::Ch64::MAX);
    assert_eq!(!p_chan::signed::Ch8::MAX, p_chan::signed::Ch8::MIN);
    assert_eq!(!p_chan::signed::Ch12::MAX, p_chan::signed::Ch12::MIN);
    assert_eq!(!p_chan::signed::Ch16::MAX, p_chan::signed::Ch16::MIN);
    assert_eq!(!p_chan::signed::Ch24::MAX, p_chan::signed::Ch24::MIN);
    assert_eq!(!p_chan::signed::Ch32::MAX, p_chan::signed::Ch32::MIN);
    assert_eq!(!p_chan::signed::Ch64::MAX, p_chan::signed::Ch64::MIN);

    assert_eq!(!p_chan::unsigned::Ch8::MIN, p_chan::unsigned::Ch8::MAX);
    assert_eq!(!p_chan::unsigned::Ch12::MIN, p_chan::unsigned::Ch12::MAX);
    assert_eq!(!p_chan::unsigned::Ch16::MIN, p_chan::unsigned::Ch16::MAX);
    assert_eq!(!p_chan::unsigned::Ch24::MIN, p_chan::unsigned::Ch24::MAX);
    assert_eq!(!p_chan::unsigned::Ch32::MIN, p_chan::unsigned::Ch32::MAX);
    assert_eq!(!p_chan::unsigned::Ch64::MIN, p_chan::unsigned::Ch64::MAX);
    assert_eq!(!p_chan::unsigned::Ch8::MAX, p_chan::unsigned::Ch8::MIN);
    assert_eq!(!p_chan::unsigned::Ch12::MAX, p_chan::unsigned::Ch12::MIN);
    assert_eq!(!p_chan::unsigned::Ch16::MAX, p_chan::unsigned::Ch16::MIN);
    assert_eq!(!p_chan::unsigned::Ch24::MAX, p_chan::unsigned::Ch24::MIN);
    assert_eq!(!p_chan::unsigned::Ch32::MAX, p_chan::unsigned::Ch32::MIN);
    assert_eq!(!p_chan::unsigned::Ch64::MAX, p_chan::unsigned::Ch64::MIN);
}
