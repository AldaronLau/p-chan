use p_chan::{downscale, signed, unsigned};

#[test]
fn f64_to_f32() {
    for i in 0..10 {
        assert_eq!(downscale::f64_to_f32(1.0 / 2.0f64.powi(i)), 0.5f32.powi(i));
    }

    assert_eq!(downscale::f64_to_f32(0.0), 0.0);
    assert_eq!(downscale::f64_to_f32(2.3283067e-10), 2.3283067e-10);
    assert_eq!(downscale::f64_to_f32(4.6566134e-10), 4.6566134e-10);
    assert_eq!(downscale::f64_to_f32(6.9849204e-10), 6.9849204e-10);
    assert_eq!(downscale::f64_to_f32(9.313227e-10), 9.313227e-10);
    assert_eq!(downscale::f64_to_f32(1.1641533e-9), 1.1641533e-9);
    assert_eq!(downscale::f64_to_f32(1.3969841e-9), 1.3969841e-9);
    assert_eq!(downscale::f64_to_f32(1.6298147e-9), 1.6298147e-9);
    assert_eq!(downscale::f64_to_f32(1.8626454e-9), 1.8626454e-9);
    assert_eq!(downscale::f64_to_f32(2.095476e-9), 2.095476e-9);
}

#[test]
fn downscale_i32() {
    let i8_list = [
        signed::Ch8::MAX.into_inner(),
        signed::Ch8::MAX.into_inner() / 2,
        signed::Ch8::MID.into_inner(),
        signed::Ch8::MIN.into_inner() / 2,
        signed::Ch8::MIN.into_inner(),
    ];
    let i12_list = [
        signed::Ch12::MAX.into_inner(),
        signed::Ch12::MAX.into_inner() / 2,
        signed::Ch12::MID.into_inner(),
        signed::Ch12::MIN.into_inner() / 2,
        signed::Ch12::MIN.into_inner(),
    ];
    let i16_list = [
        signed::Ch16::MAX.into_inner(),
        signed::Ch16::MAX.into_inner() / 2,
        signed::Ch16::MID.into_inner(),
        signed::Ch16::MIN.into_inner() / 2,
        signed::Ch16::MIN.into_inner(),
    ];
    let i24_list = [
        signed::Ch24::MAX.into_inner(),
        signed::Ch24::MAX.into_inner() / 2,
        signed::Ch24::MID.into_inner(),
        signed::Ch24::MIN.into_inner() / 2,
        signed::Ch24::MIN.into_inner(),
    ];
    let i32_list = [i32::MAX, i32::MAX / 2, -1, i32::MIN / 2, i32::MIN];

    for (i32, i24) in i32_list.iter().cloned().zip(i24_list.iter().cloned()) {
        assert_eq!(downscale::i32_to_i24(i32), i24);
    }

    for (i32, i16) in i32_list.iter().cloned().zip(i16_list.iter().cloned()) {
        assert_eq!(downscale::i32_to_i16(i32), i16);
    }

    for (i32, i12) in i32_list.iter().cloned().zip(i12_list.iter().cloned()) {
        assert_eq!(downscale::i32_to_i12(i32), i12);
    }

    for (i32, i8) in i32_list.iter().cloned().zip(i8_list.iter().cloned()) {
        assert_eq!(downscale::i32_to_i8(i32), i8);
    }
}

#[test]
fn downscale_u32() {
    let u8_list = [
        unsigned::Ch8::MAX.into_inner(),
        unsigned::Ch8::MID.into_inner(),
        unsigned::Ch8::MIN.into_inner(),
    ];
    let u12_list = [
        unsigned::Ch12::MAX.into_inner(),
        unsigned::Ch12::MID.into_inner(),
        unsigned::Ch12::MIN.into_inner(),
    ];
    let u16_list = [
        unsigned::Ch16::MAX.into_inner(),
        unsigned::Ch16::MID.into_inner(),
        unsigned::Ch16::MIN.into_inner(),
    ];
    let u24_list = [
        unsigned::Ch24::MAX.into_inner(),
        unsigned::Ch24::MID.into_inner(),
        unsigned::Ch24::MIN.into_inner(),
    ];
    let u32_list = [u32::MAX, u32::MAX / 2, u32::MIN];

    for (u32, u24) in u32_list.iter().cloned().zip(u24_list.iter().cloned()) {
        assert_eq!(downscale::u32_to_u24(u32), u24);
    }

    for (u32, u16) in u32_list.iter().cloned().zip(u16_list.iter().cloned()) {
        assert_eq!(downscale::u32_to_u16(u32), u16);
    }

    for (u32, u12) in u32_list.iter().cloned().zip(u12_list.iter().cloned()) {
        assert_eq!(downscale::u32_to_u12(u32), u12);
    }

    for (u32, u8) in u32_list.iter().cloned().zip(u8_list.iter().cloned()) {
        assert_eq!(downscale::u32_to_u8(u32), u8);
    }
}

#[test]
fn downscale_i64() {
    let i8_list = [
        signed::Ch8::MAX.into_inner(),
        signed::Ch8::MAX.into_inner() / 2,
        signed::Ch8::MID.into_inner(),
        signed::Ch8::MIN.into_inner() / 2,
        signed::Ch8::MIN.into_inner(),
    ];
    let i12_list = [
        signed::Ch12::MAX.into_inner(),
        signed::Ch12::MAX.into_inner() / 2,
        signed::Ch12::MID.into_inner(),
        signed::Ch12::MIN.into_inner() / 2,
        signed::Ch12::MIN.into_inner(),
    ];
    let i16_list = [
        signed::Ch16::MAX.into_inner(),
        signed::Ch16::MAX.into_inner() / 2,
        signed::Ch16::MID.into_inner(),
        signed::Ch16::MIN.into_inner() / 2,
        signed::Ch16::MIN.into_inner(),
    ];
    let i24_list = [
        signed::Ch24::MAX.into_inner(),
        signed::Ch24::MAX.into_inner() / 2,
        signed::Ch24::MID.into_inner(),
        signed::Ch24::MIN.into_inner() / 2,
        signed::Ch24::MIN.into_inner(),
    ];
    let i32_list = [i32::MAX, i32::MAX / 2, -1, i32::MIN / 2, i32::MIN];
    let i64_list = [i64::MAX, i64::MAX / 2, -1, i64::MIN / 2, i64::MIN];

    for (i64, i32) in i64_list.iter().cloned().zip(i32_list.iter().cloned()) {
        assert_eq!(downscale::i64_to_i32(i64), i32);
    }

    for (i64, i24) in i64_list.iter().cloned().zip(i24_list.iter().cloned()) {
        assert_eq!(downscale::i64_to_i24(i64), i24);
    }

    for (i64, i16) in i64_list.iter().cloned().zip(i16_list.iter().cloned()) {
        assert_eq!(downscale::i64_to_i16(i64), i16);
    }

    for (i64, i12) in i64_list.iter().cloned().zip(i12_list.iter().cloned()) {
        assert_eq!(downscale::i64_to_i12(i64), i12);
    }

    for (i64, i8) in i64_list.iter().cloned().zip(i8_list.iter().cloned()) {
        assert_eq!(downscale::i64_to_i8(i64), i8);
    }
}

#[test]
fn downscale_u64() {
    let u8_list = [
        unsigned::Ch8::MAX.into_inner(),
        unsigned::Ch8::MID.into_inner(),
        unsigned::Ch8::MIN.into_inner(),
    ];
    let u12_list = [
        unsigned::Ch12::MAX.into_inner(),
        unsigned::Ch12::MID.into_inner(),
        unsigned::Ch12::MIN.into_inner(),
    ];
    let u16_list = [
        unsigned::Ch16::MAX.into_inner(),
        unsigned::Ch16::MID.into_inner(),
        unsigned::Ch16::MIN.into_inner(),
    ];
    let u24_list = [
        unsigned::Ch24::MAX.into_inner(),
        unsigned::Ch24::MID.into_inner(),
        unsigned::Ch24::MIN.into_inner(),
    ];
    let u32_list = [u32::MAX, u32::MAX / 2, u32::MIN];
    let u64_list = [u64::MAX, u64::MAX / 2, u64::MIN];

    for (u64, u32) in u64_list.iter().cloned().zip(u32_list.iter().cloned()) {
        assert_eq!(downscale::u64_to_u32(u64), u32);
    }

    for (u64, u24) in u64_list.iter().cloned().zip(u24_list.iter().cloned()) {
        assert_eq!(downscale::u64_to_u24(u64), u24);
    }

    for (u64, u16) in u64_list.iter().cloned().zip(u16_list.iter().cloned()) {
        assert_eq!(downscale::u64_to_u16(u64), u16);
    }

    for (u64, u12) in u64_list.iter().cloned().zip(u12_list.iter().cloned()) {
        assert_eq!(downscale::u64_to_u12(u64), u12);
    }

    for (u64, u8) in u64_list.iter().cloned().zip(u8_list.iter().cloned()) {
        assert_eq!(downscale::u64_to_u8(u64), u8);
    }
}
