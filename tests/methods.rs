#[test]
fn midpoint_unsigned() {
    assert_eq!(
        p_chan::unsigned::Ch8::new(100)
            .midpoint(p_chan::unsigned::Ch8::new(50))
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch12::new(100)
            .midpoint(p_chan::unsigned::Ch12::new(50))
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch16::new(100)
            .midpoint(p_chan::unsigned::Ch16::new(50))
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch24::new(100)
            .midpoint(p_chan::unsigned::Ch24::new(50))
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch32::new(1.0)
            .midpoint(p_chan::unsigned::Ch32::new(0.5))
            .into_inner(),
        0.75,
    );
    assert_eq!(
        p_chan::unsigned::Ch64::new(1.0)
            .midpoint(p_chan::unsigned::Ch64::new(0.5))
            .into_inner(),
        0.75,
    );
}
#[test]
fn midpoint_signed() {
    assert_eq!(
        p_chan::signed::Ch8::new(-100)
            .midpoint(p_chan::signed::Ch8::new(50))
            .into_inner(),
        -25,
    );
    assert_eq!(
        p_chan::signed::Ch12::new(-100)
            .midpoint(p_chan::signed::Ch12::new(50))
            .into_inner(),
        -25,
    );
    assert_eq!(
        p_chan::signed::Ch16::new(100)
            .midpoint(p_chan::signed::Ch16::new(50))
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch24::new(100)
            .midpoint(p_chan::signed::Ch24::new(50))
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch32::new(1.0)
            .midpoint(p_chan::signed::Ch32::new(0.5))
            .into_inner(),
        0.75,
    );
    assert_eq!(
        p_chan::signed::Ch64::new(-1.0)
            .midpoint(p_chan::signed::Ch64::new(0.5))
            .into_inner(),
        -0.25,
    );
}

#[test]
fn new() {
    // 12-bit unsigned
    assert_eq!(p_chan::unsigned::Ch12::new(0).into_inner(), 0);
    assert_eq!(p_chan::unsigned::Ch12::new(256).into_inner(), 256);
    assert_eq!(p_chan::unsigned::Ch12::new(4_095).into_inner(), 4_095);
    assert_eq!(p_chan::unsigned::Ch12::new(u16::MAX).into_inner(), 4_095);
    // 24-bit unsigned
    assert_eq!(p_chan::unsigned::Ch24::new(0).into_inner(), 0);
    assert_eq!(p_chan::unsigned::Ch24::new(65536).into_inner(), 65536);
    assert_eq!(
        p_chan::unsigned::Ch24::new(16_777_215).into_inner(),
        16_777_215,
    );
    assert_eq!(
        p_chan::unsigned::Ch24::new(u32::MAX).into_inner(),
        16_777_215,
    );
    // 12-bit signed
    assert_eq!(p_chan::signed::Ch12::new(-256).into_inner(), -256);
    assert_eq!(p_chan::signed::Ch12::new(-2_048).into_inner(), -2_048);
    assert_eq!(p_chan::signed::Ch12::new(i16::MIN).into_inner(), -2_048);
    assert_eq!(p_chan::signed::Ch12::new(0).into_inner(), 0);
    assert_eq!(p_chan::signed::Ch12::new(256).into_inner(), 256);
    assert_eq!(p_chan::signed::Ch12::new(2_047).into_inner(), 2_047);
    assert_eq!(p_chan::signed::Ch12::new(i16::MAX).into_inner(), 2_047);
    // 24-bit unsigned
    assert_eq!(p_chan::signed::Ch24::new(-65536).into_inner(), -65536);
    assert_eq!(
        p_chan::signed::Ch24::new(-8_388_608).into_inner(),
        -8_388_608,
    );
    assert_eq!(p_chan::signed::Ch24::new(i32::MIN).into_inner(), -8_388_608,);
    assert_eq!(p_chan::signed::Ch24::new(0).into_inner(), 0);
    assert_eq!(p_chan::signed::Ch24::new(65536).into_inner(), 65536);
    assert_eq!(p_chan::signed::Ch24::new(8_388_607).into_inner(), 8_388_607,);
    assert_eq!(p_chan::signed::Ch24::new(i32::MAX).into_inner(), 8_388_607,);
}

#[test]
fn clamp_unsigned_float_channels() {
    assert_eq!(p_chan::unsigned::Ch32::new(-2.0).into_inner(), -2.0);
    assert_eq!(p_chan::unsigned::Ch64::new(-2.0).into_inner(), -2.0);
    assert_eq!(
        p_chan::unsigned::Ch32::new(-2.0)
            .clamp(p_chan::unsigned::Ch32::MIN, p_chan::unsigned::Ch32::MAX)
            .into_inner(),
        0.0,
    );
    assert_eq!(
        p_chan::unsigned::Ch64::new(-2.0)
            .clamp(p_chan::unsigned::Ch64::MIN, p_chan::unsigned::Ch64::MAX)
            .into_inner(),
        0.0,
    );
    assert_eq!(p_chan::unsigned::Ch32::new(0.25).into_inner(), 0.25);
    assert_eq!(p_chan::unsigned::Ch64::new(0.25).into_inner(), 0.25);
    assert_eq!(
        p_chan::unsigned::Ch32::new(2.0)
            .clamp(p_chan::unsigned::Ch32::MIN, p_chan::unsigned::Ch32::MAX)
            .into_inner(),
        1.0,
    );
    assert_eq!(
        p_chan::unsigned::Ch64::new(2.0)
            .clamp(p_chan::unsigned::Ch64::MIN, p_chan::unsigned::Ch64::MAX)
            .into_inner(),
        1.0,
    );
    assert_eq!(
        p_chan::unsigned::Ch32::new(0.5)
            .clamp(p_chan::unsigned::Ch32::MIN, p_chan::unsigned::Ch32::MAX)
            .into_inner(),
        0.5,
    );
    assert_eq!(
        p_chan::unsigned::Ch64::new(0.5)
            .clamp(p_chan::unsigned::Ch64::MIN, p_chan::unsigned::Ch64::MAX)
            .into_inner(),
        0.5,
    );
}

#[test]
fn clamp_signed_float_channels() {
    assert_eq!(p_chan::signed::Ch32::new(-2.0).into_inner(), -2.0);
    assert_eq!(p_chan::signed::Ch64::new(-2.0).into_inner(), -2.0);
    assert_eq!(
        p_chan::signed::Ch32::new(-2.0)
            .clamp(p_chan::signed::Ch32::MIN, p_chan::signed::Ch32::MAX)
            .into_inner(),
        -1.0,
    );
    assert_eq!(
        p_chan::signed::Ch64::new(-2.0)
            .clamp(p_chan::signed::Ch64::MIN, p_chan::signed::Ch64::MAX)
            .into_inner(),
        -1.0,
    );
    assert_eq!(p_chan::signed::Ch32::new(0.25).into_inner(), 0.25);
    assert_eq!(p_chan::signed::Ch64::new(0.25).into_inner(), 0.25);
    assert_eq!(
        p_chan::signed::Ch32::new(2.0)
            .clamp(p_chan::signed::Ch32::MIN, p_chan::signed::Ch32::MAX)
            .into_inner(),
        1.0,
    );
    assert_eq!(
        p_chan::signed::Ch64::new(2.0)
            .clamp(p_chan::signed::Ch64::MIN, p_chan::signed::Ch64::MAX)
            .into_inner(),
        1.0,
    );
    assert_eq!(
        p_chan::signed::Ch32::new(0.5)
            .clamp(p_chan::signed::Ch32::MIN, p_chan::signed::Ch32::MAX)
            .into_inner(),
        0.5,
    );
    assert_eq!(
        p_chan::signed::Ch64::new(0.5)
            .clamp(p_chan::signed::Ch64::MIN, p_chan::signed::Ch64::MAX)
            .into_inner(),
        0.5,
    );
}

#[test]
fn clamp_unsigned_int_channels() {
    assert_eq!(p_chan::unsigned::Ch8::new(50).into_inner(), 50);
    assert_eq!(p_chan::unsigned::Ch12::new(50).into_inner(), 50);
    assert_eq!(p_chan::unsigned::Ch16::new(50).into_inner(), 50);
    assert_eq!(p_chan::unsigned::Ch24::new(50).into_inner(), 50);
    // 8-bit
    assert_eq!(
        p_chan::unsigned::Ch8::new(50)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch8::new(75)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch8::new(80)
            .clamp(75.into(), 100.into())
            .into_inner(),
        80,
    );
    assert_eq!(
        p_chan::unsigned::Ch8::new(100)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    assert_eq!(
        p_chan::unsigned::Ch8::new(150)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    // 12-bit
    assert_eq!(
        p_chan::unsigned::Ch12::new(50)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch12::new(75)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch12::new(80)
            .clamp(75.into(), 100.into())
            .into_inner(),
        80,
    );
    assert_eq!(
        p_chan::unsigned::Ch12::new(100)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    assert_eq!(
        p_chan::unsigned::Ch12::new(150)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    // 16-bit
    assert_eq!(
        p_chan::unsigned::Ch16::new(50)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch16::new(75)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch16::new(80)
            .clamp(75.into(), 100.into())
            .into_inner(),
        80,
    );
    assert_eq!(
        p_chan::unsigned::Ch16::new(100)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    assert_eq!(
        p_chan::unsigned::Ch16::new(150)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    // 24-bit
    assert_eq!(
        p_chan::unsigned::Ch24::new(50)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch24::new(75)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::unsigned::Ch24::new(80)
            .clamp(75.into(), 100.into())
            .into_inner(),
        80,
    );
    assert_eq!(
        p_chan::unsigned::Ch24::new(100)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    assert_eq!(
        p_chan::unsigned::Ch24::new(150)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
}

#[test]
fn clamp_signed_int_channels() {
    assert_eq!(p_chan::signed::Ch8::new(50).into_inner(), 50);
    assert_eq!(p_chan::signed::Ch12::new(50).into_inner(), 50);
    assert_eq!(p_chan::signed::Ch16::new(50).into_inner(), 50);
    assert_eq!(p_chan::signed::Ch24::new(50).into_inner(), 50);
    // 8-bit
    assert_eq!(
        p_chan::signed::Ch8::new(50)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch8::new(75)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch8::new(80)
            .clamp(75.into(), 100.into())
            .into_inner(),
        80,
    );
    assert_eq!(
        p_chan::signed::Ch8::new(100)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    assert_eq!(
        p_chan::signed::Ch8::new(125)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    // 12-bit
    assert_eq!(
        p_chan::signed::Ch12::new(50)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch12::new(75)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch12::new(80)
            .clamp(75.into(), 100.into())
            .into_inner(),
        80,
    );
    assert_eq!(
        p_chan::signed::Ch12::new(100)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    assert_eq!(
        p_chan::signed::Ch12::new(150)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    // 16-bit
    assert_eq!(
        p_chan::signed::Ch16::new(50)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch16::new(75)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch16::new(80)
            .clamp(75.into(), 100.into())
            .into_inner(),
        80,
    );
    assert_eq!(
        p_chan::signed::Ch16::new(100)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    assert_eq!(
        p_chan::signed::Ch16::new(150)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    // 24-bit
    assert_eq!(
        p_chan::signed::Ch24::new(50)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch24::new(75)
            .clamp(75.into(), 100.into())
            .into_inner(),
        75,
    );
    assert_eq!(
        p_chan::signed::Ch24::new(80)
            .clamp(75.into(), 100.into())
            .into_inner(),
        80,
    );
    assert_eq!(
        p_chan::signed::Ch24::new(100)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
    assert_eq!(
        p_chan::signed::Ch24::new(150)
            .clamp(75.into(), 100.into())
            .into_inner(),
        100,
    );
}
