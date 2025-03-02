use std::{
    fmt::{Display, Debug},
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
