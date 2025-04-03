use p_chan::{signed, unsigned};

macro_rules! test_conversion {
    ($from:ty, $into:ty) => {
        assert_eq!(<$into>::from(<$from>::MIN), <$into>::MIN);
        assert_eq!(
            <$into>::from(<$from>::MID).midpoint(!<$into>::from(<$from>::MID)),
            <$into>::MID,
        );
        assert_eq!(<$into>::from(<$from>::MAX), <$into>::MAX);

        assert_eq!(<$from>::from(<$into>::MIN), <$from>::MIN);
        assert_eq!(
            <$from>::from(<$into>::MID).midpoint(!<$from>::from(<$into>::MID)),
            <$from>::MID,
        );
        assert_eq!(<$from>::from(<$into>::MAX), <$from>::MAX);
    };
}

#[test]
fn from_conversions() {
    // Unsigned conversions
    test_conversion!(unsigned::Ch64, unsigned::Ch8);
    test_conversion!(unsigned::Ch64, unsigned::Ch12);
    test_conversion!(unsigned::Ch64, unsigned::Ch16);
    test_conversion!(unsigned::Ch64, unsigned::Ch24);
    test_conversion!(unsigned::Ch64, unsigned::Ch32);

    test_conversion!(unsigned::Ch32, unsigned::Ch8);
    test_conversion!(unsigned::Ch32, unsigned::Ch12);
    test_conversion!(unsigned::Ch32, unsigned::Ch16);
    test_conversion!(unsigned::Ch32, unsigned::Ch24);

    test_conversion!(unsigned::Ch24, unsigned::Ch8);
    test_conversion!(unsigned::Ch24, unsigned::Ch12);
    test_conversion!(unsigned::Ch24, unsigned::Ch16);

    test_conversion!(unsigned::Ch16, unsigned::Ch8);
    test_conversion!(unsigned::Ch16, unsigned::Ch12);

    test_conversion!(unsigned::Ch12, unsigned::Ch8);

    // Signed conversions
    test_conversion!(signed::Ch64, signed::Ch8);
    test_conversion!(signed::Ch64, signed::Ch12);
    test_conversion!(signed::Ch64, signed::Ch16);
    test_conversion!(signed::Ch64, signed::Ch24);
    test_conversion!(signed::Ch64, signed::Ch32);

    test_conversion!(signed::Ch32, signed::Ch8);
    test_conversion!(signed::Ch32, signed::Ch12);
    test_conversion!(signed::Ch32, signed::Ch16);
    test_conversion!(signed::Ch32, signed::Ch24);

    test_conversion!(signed::Ch24, signed::Ch8);
    test_conversion!(signed::Ch24, signed::Ch12);
    test_conversion!(signed::Ch24, signed::Ch16);

    test_conversion!(signed::Ch16, signed::Ch8);
    test_conversion!(signed::Ch16, signed::Ch12);

    test_conversion!(signed::Ch12, signed::Ch8);

    // unsigned <-> signed
    test_conversion!(unsigned::Ch8, signed::Ch8);
    test_conversion!(unsigned::Ch8, signed::Ch12);
    test_conversion!(unsigned::Ch8, signed::Ch16);
    test_conversion!(unsigned::Ch8, signed::Ch24);
    test_conversion!(unsigned::Ch8, signed::Ch32);
    test_conversion!(unsigned::Ch8, signed::Ch64);

    test_conversion!(unsigned::Ch12, signed::Ch8);
    test_conversion!(unsigned::Ch12, signed::Ch12);
    test_conversion!(unsigned::Ch12, signed::Ch16);
    test_conversion!(unsigned::Ch12, signed::Ch24);
    test_conversion!(unsigned::Ch12, signed::Ch32);
    test_conversion!(unsigned::Ch12, signed::Ch64);

    test_conversion!(unsigned::Ch16, signed::Ch8);
    test_conversion!(unsigned::Ch16, signed::Ch12);
    test_conversion!(unsigned::Ch16, signed::Ch16);
    test_conversion!(unsigned::Ch16, signed::Ch24);
    test_conversion!(unsigned::Ch16, signed::Ch32);
    test_conversion!(unsigned::Ch16, signed::Ch64);

    test_conversion!(unsigned::Ch24, signed::Ch8);
    test_conversion!(unsigned::Ch24, signed::Ch12);
    test_conversion!(unsigned::Ch24, signed::Ch16);
    test_conversion!(unsigned::Ch24, signed::Ch24);
    test_conversion!(unsigned::Ch24, signed::Ch32);
    test_conversion!(unsigned::Ch24, signed::Ch64);

    test_conversion!(unsigned::Ch32, signed::Ch8);
    test_conversion!(unsigned::Ch32, signed::Ch12);
    test_conversion!(unsigned::Ch32, signed::Ch16);
    test_conversion!(unsigned::Ch32, signed::Ch24);
    test_conversion!(unsigned::Ch32, signed::Ch32);
    test_conversion!(unsigned::Ch32, signed::Ch64);

    test_conversion!(unsigned::Ch64, signed::Ch8);
    test_conversion!(unsigned::Ch64, signed::Ch12);
    test_conversion!(unsigned::Ch64, signed::Ch16);
    test_conversion!(unsigned::Ch64, signed::Ch24);
    test_conversion!(unsigned::Ch64, signed::Ch32);
    test_conversion!(unsigned::Ch64, signed::Ch64);
}
