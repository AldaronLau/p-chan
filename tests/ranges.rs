#[test]
fn ranges() {
    #![expect(dead_code)]

    #[derive(Debug)]
    struct MinMidMax<T> {
        min: T,
        mid: T,
        max: T,
    }

    #[derive(Debug)]
    struct Ranges {
        ch8_unsigned: MinMidMax<p_chan::unsigned::Ch8>,
        ch12_unsigned: MinMidMax<p_chan::unsigned::Ch12>,
        ch16_unsigned: MinMidMax<p_chan::unsigned::Ch16>,
        ch24_unsigned: MinMidMax<p_chan::unsigned::Ch24>,
        ch8_signed: MinMidMax<p_chan::signed::Ch8>,
        ch12_signed: MinMidMax<p_chan::signed::Ch12>,
        ch16_signed: MinMidMax<p_chan::signed::Ch16>,
        ch24_signed: MinMidMax<p_chan::signed::Ch24>,
    }

    const RANGES: Ranges = Ranges {
        ch8_unsigned: MinMidMax {
            min: p_chan::unsigned::Ch8::MIN,
            mid: p_chan::unsigned::Ch8::MID,
            max: p_chan::unsigned::Ch8::MAX,
        },
        ch12_unsigned: MinMidMax {
            min: p_chan::unsigned::Ch12::MIN,
            mid: p_chan::unsigned::Ch12::MID,
            max: p_chan::unsigned::Ch12::MAX,
        },
        ch16_unsigned: MinMidMax {
            min: p_chan::unsigned::Ch16::MIN,
            mid: p_chan::unsigned::Ch16::MID,
            max: p_chan::unsigned::Ch16::MAX,
        },
        ch24_unsigned: MinMidMax {
            min: p_chan::unsigned::Ch24::MIN,
            mid: p_chan::unsigned::Ch24::MID,
            max: p_chan::unsigned::Ch24::MAX,
        },

        ch8_signed: MinMidMax {
            min: p_chan::signed::Ch8::MIN,
            mid: p_chan::signed::Ch8::MID,
            max: p_chan::signed::Ch8::MAX,
        },
        ch12_signed: MinMidMax {
            min: p_chan::signed::Ch12::MIN,
            mid: p_chan::signed::Ch12::MID,
            max: p_chan::signed::Ch12::MAX,
        },
        ch16_signed: MinMidMax {
            min: p_chan::signed::Ch16::MIN,
            mid: p_chan::signed::Ch16::MID,
            max: p_chan::signed::Ch16::MAX,
        },
        ch24_signed: MinMidMax {
            min: p_chan::signed::Ch24::MIN,
            mid: p_chan::signed::Ch24::MID,
            max: p_chan::signed::Ch24::MAX,
        },
    };

    insta::assert_debug_snapshot!(RANGES);
}
