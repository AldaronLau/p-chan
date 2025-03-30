use p_chan::{signed, unsigned, upscale};

#[test]
fn f32_to_f64() {
    for i in 0..10 {
        assert_eq!(upscale::f32_to_f64(1.0 / 2.0f32.powi(i)), 0.5f64.powi(i));
    }

    assert_eq!(upscale::f32_to_f64(0.0), 0.0);
    assert_eq!(upscale::f32_to_f64(2.3283067e-10), 2.3283067140944524e-10);
    assert_eq!(upscale::f32_to_f64(4.6566134e-10), 4.656613428188905e-10);
    assert_eq!(upscale::f32_to_f64(6.9849204e-10), 6.984920419839113e-10);
    assert_eq!(upscale::f32_to_f64(9.313227e-10), 9.31322685637781e-10);
    assert_eq!(upscale::f32_to_f64(1.1641533e-9), 1.1641533292916506e-9);
    assert_eq!(upscale::f32_to_f64(1.3969841e-9), 1.3969840839678227e-9);
    assert_eq!(upscale::f32_to_f64(1.6298147e-9), 1.6298147276216923e-9);
    assert_eq!(upscale::f32_to_f64(1.8626454e-9), 1.862645371275562e-9);
    assert_eq!(upscale::f32_to_f64(2.095476e-9), 2.0954760149294316e-9);
}

#[test]
fn upscale_i32() {
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
        assert_eq!(upscale::i24_to_i32(i24) >> 8, i32 >> 8);
    }

    for (i32, i16) in i32_list.iter().cloned().zip(i16_list.iter().cloned()) {
        assert_eq!(upscale::i16_to_i32(i16) >> 16, i32 >> 16);
    }

    for (i32, i12) in i32_list.iter().cloned().zip(i12_list.iter().cloned()) {
        assert_eq!(upscale::i12_to_i32(i12) >> 20, i32 >> 20);
    }

    for (i32, i8) in i32_list.iter().cloned().zip(i8_list.iter().cloned()) {
        assert_eq!(upscale::i8_to_i32(i8) >> 24, i32 >> 24);
    }
}

#[test]
fn upscale_u32() {
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
    let u32_list = [
        u32::MAX,
        0b01111111111111111111111101111111,
        u32::MIN,
    ];

    for (u32, u24) in u32_list.iter().cloned().zip(u24_list.iter().cloned()) {
        assert_eq!(upscale::u24_to_u32(u24), u32);
    }
    
    let u32_list = [
        u32::MAX,
        0b01111111111111110111111111111111,
        u32::MIN,
    ];

    for (u32, u16) in u32_list.iter().cloned().zip(u16_list.iter().cloned()) {
        assert_eq!(upscale::u16_to_u32(u16), u32);
    }
    
    let u32_list = [
        u32::MAX,
        0b01111111111101111111111101111111,
        u32::MIN,
    ];

    for (u32, u12) in u32_list.iter().cloned().zip(u12_list.iter().cloned()) {
        assert_eq!(upscale::u12_to_u32(u12), u32);
    }
    
    let u32_list = [
        u32::MAX,
        0b01111111011111110111111101111111,
        u32::MIN,
    ];

    for (u32, u8) in u32_list.iter().cloned().zip(u8_list.iter().cloned()) {
        assert_eq!(upscale::u8_to_u32(u8), u32);
    }
}

#[test]
fn upscale_i64() {
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
        assert_eq!(upscale::i32_to_i64(i32) >> 32, i64 >> 32);
    }

    for (i64, i24) in i64_list.iter().cloned().zip(i24_list.iter().cloned()) {
        assert_eq!(upscale::i24_to_i64(i24) >> 40, i64 >> 40);
    }

    for (i64, i16) in i64_list.iter().cloned().zip(i16_list.iter().cloned()) {
        assert_eq!(upscale::i16_to_i64(i16) >> 48, i64 >> 48);
    }

    for (i64, i12) in i64_list.iter().cloned().zip(i12_list.iter().cloned()) {
        assert_eq!(upscale::i12_to_i64(i12) >> 52, i64 >> 52);
    }

    for (i64, i8) in i64_list.iter().cloned().zip(i8_list.iter().cloned()) {
        assert_eq!(upscale::i8_to_i64(i8) >> 56, i64 >> 56);
    }
}

#[test]
fn upscale_u64() {
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
    let u64_list = [
        u64::MAX,
        0b0111111111111111111111111111111101111111111111111111111111111111,
        u64::MIN,
    ];

    for (u64, u32) in u64_list.iter().cloned().zip(u32_list.iter().cloned()) {
        assert_eq!(upscale::u32_to_u64(u32), u64);
    }

    let u64_list = [
        u64::MAX,
        0b0111111111111111111111110111111111111111111111110111111111111111,
        u64::MIN,
    ];

    for (u64, u24) in u64_list.iter().cloned().zip(u24_list.iter().cloned()) {
        assert_eq!(upscale::u24_to_u64(u24), u64);
    }

    let u64_list = [
        u64::MAX,
        0b0111111111111111011111111111111101111111111111110111111111111111,
        u64::MIN,
    ];

    for (u64, u16) in u64_list.iter().cloned().zip(u16_list.iter().cloned()) {
        assert_eq!(upscale::u16_to_u64(u16), u64);
    }

    let u64_list = [
        u64::MAX,
        0b0111111111110111111111110111111111110111111111110111111111110111,
        u64::MIN,
    ];

    for (u64, u12) in u64_list.iter().cloned().zip(u12_list.iter().cloned()) {
        assert_eq!(upscale::u12_to_u64(u12), u64);
    }

    let u64_list = [
        u64::MAX,
        0b0111111101111111011111110111111101111111011111110111111101111111,
        u64::MIN,
    ];

    for (u64, u8) in u64_list.iter().cloned().zip(u8_list.iter().cloned()) {
        assert_eq!(upscale::u8_to_u64(u8), u64);
    }
}
