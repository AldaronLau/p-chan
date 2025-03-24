use std::{
    fmt::{Debug, Display},
    ops::{Add, Sub},
};

#[test]
fn impls_traits() {
    // FIXME: Add `Hash`, `Ord`, `Eq`

    trait AllTraits:
        Add
        + Sub
        + Clone
        + Copy
        + Debug
        + Display
        + Default
        + PartialEq
        + PartialOrd
    {
    }

    impl<T> AllTraits for T where
        T: Add
            + Sub
            + Clone
            + Copy
            + Debug
            + Display
            + Default
            + PartialEq
            + PartialOrd
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
