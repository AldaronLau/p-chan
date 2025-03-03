use p_chan::chan::{f32_to_i32, f32_to_u32, i32_to_f32, u32_to_f32};

#[test]
fn unsigned_to_float() {
    for i in 0..10 {
        assert_eq!(
            u32_to_f32(u32::MAX / 2u32.pow(i)),
            0.5f32.powi(i.try_into().unwrap()),
        );
    }

    assert_eq!(u32_to_f32(0), 0.0);
    assert_eq!(u32_to_f32(1), 2.3283067e-10);
    assert_eq!(u32_to_f32(2), 4.6566134e-10);
    assert_eq!(u32_to_f32(3), 6.9849204e-10);
    assert_eq!(u32_to_f32(4), 9.313227e-10);
    assert_eq!(u32_to_f32(5), 1.1641533e-9);
    assert_eq!(u32_to_f32(6), 1.3969841e-9);
    assert_eq!(u32_to_f32(7), 1.6298147e-9);
    assert_eq!(u32_to_f32(8), 1.8626454e-9);
    assert_eq!(u32_to_f32(9), 2.095476e-9);
}

#[test]
fn signed_to_float() {
    // Since there are more negative than positive integers, zeros are not
    // an exact match between integer and floating point
    assert_eq!(i32_to_f32(i32::MAX), 1.0);
    assert_eq!(i32_to_f32(i32::MAX / 2), 0.5);
    assert_eq!(i32_to_f32(i32::MAX / 4), 0.25);
    assert_eq!(i32_to_f32(i32::MAX / 8), 0.125);
    assert_eq!(i32_to_f32(i32::MAX / 16), 0.0625);
    assert_eq!(i32_to_f32(i32::MAX / 32), 0.03125);
    assert_eq!(i32_to_f32(i32::MAX / 64), 0.015625);
    assert_eq!(i32_to_f32(i32::MAX / 128), 0.0078125);
    assert_eq!(i32_to_f32(i32::MAX / 256), 0.00390625);
    assert_eq!(i32_to_f32(i32::MAX / 512), 0.001953125);
    assert_eq!(i32_to_f32(7), 3.49246e-9);
    assert_eq!(i32_to_f32(6), 3.0267988e-9);
    assert_eq!(i32_to_f32(5), 2.5611373e-9);
    assert_eq!(i32_to_f32(4), 2.095476e-9);
    assert_eq!(i32_to_f32(3), 1.6298147e-9);
    assert_eq!(i32_to_f32(2), 1.1641533e-9);
    assert_eq!(i32_to_f32(1), 6.9849204e-10);
    assert_eq!(i32_to_f32(0), 2.3283067e-10);
    assert_eq!(i32_to_f32(-1), -2.3283067e-10);
    assert_eq!(i32_to_f32(-2), -6.9849204e-10);
    assert_eq!(i32_to_f32(-3), -1.1641533e-9);
    assert_eq!(i32_to_f32(-4), -1.6298147e-9);
    assert_eq!(i32_to_f32(-5), -2.095476e-9);
    assert_eq!(i32_to_f32(-6), -2.5611373e-9);
    assert_eq!(i32_to_f32(-7), -3.0267988e-9);
    assert_eq!(i32_to_f32(-8), -3.49246e-9);
    assert_eq!(i32_to_f32(i32::MIN / 512), -0.001953125);
    assert_eq!(i32_to_f32(i32::MIN / 256), -0.00390625);
    assert_eq!(i32_to_f32(i32::MIN / 128), -0.0078125);
    assert_eq!(i32_to_f32(i32::MIN / 64), -0.015625);
    assert_eq!(i32_to_f32(i32::MIN / 32), -0.03125);
    assert_eq!(i32_to_f32(i32::MIN / 16), -0.0625);
    assert_eq!(i32_to_f32(i32::MIN / 8), -0.125);
    assert_eq!(i32_to_f32(i32::MIN / 4), -0.25);
    assert_eq!(i32_to_f32(i32::MIN / 2), -0.5);
    assert_eq!(i32_to_f32(i32::MIN), -1.0);
}

#[test]
fn float_to_unsigned() {
    for i in 0..10 {
        assert_eq!(
            f32_to_u32(0.5f32.powi(i.try_into().unwrap())),
            u32::MAX / 2u32.pow(i),
        );
    }

    assert_eq!(f32_to_u32(0.0), 0);
    assert_eq!(f32_to_u32(2.3283067e-10), 1);
    assert_eq!(f32_to_u32(4.6566134e-10), 2);
    assert_eq!(f32_to_u32(6.9849204e-10), 3);
    assert_eq!(f32_to_u32(9.313227e-10), 4);
    assert_eq!(f32_to_u32(1.1641533e-9), 5);
    assert_eq!(f32_to_u32(1.3969841e-9), 6);
    assert_eq!(f32_to_u32(1.6298147e-9), 7);
    assert_eq!(f32_to_u32(1.8626454e-9), 8);
    assert_eq!(f32_to_u32(2.095476e-9), 9);
    assert_eq!(f32_to_u32(f32::INFINITY), u32::MAX);
    assert_eq!(f32_to_u32(f32::NEG_INFINITY), u32::MIN);
}

#[test]
fn float_to_signed() {
    assert_eq!(f32_to_i32(0.0), 0);
    assert_eq!(f32_to_i32(1.0), i32::MAX);
    assert_eq!(f32_to_i32(0.5), i32::MAX / 2);
    assert_eq!(f32_to_i32(0.25), i32::MAX / 4);
    assert_eq!(f32_to_i32(0.125), i32::MAX / 8);
    assert_eq!(f32_to_i32(0.0625), i32::MAX / 16);
    assert_eq!(f32_to_i32(0.03125), i32::MAX / 32);
    assert_eq!(f32_to_i32(0.015625), i32::MAX / 64);
    assert_eq!(f32_to_i32(0.0078125), i32::MAX / 128);
    assert_eq!(f32_to_i32(0.00390625), i32::MAX / 256);
    assert_eq!(f32_to_i32(0.001953125), i32::MAX / 512);
    assert_eq!(f32_to_i32(3.49246e-9), 7);
    assert_eq!(f32_to_i32(3.0267988e-9), 6);
    assert_eq!(f32_to_i32(2.5611373e-9), 5);
    assert_eq!(f32_to_i32(2.095476e-9), 4);
    assert_eq!(f32_to_i32(1.6298147e-9), 3);
    assert_eq!(f32_to_i32(1.1641533e-9), 2);
    assert_eq!(f32_to_i32(6.9849204e-10), 1);
    assert_eq!(f32_to_i32(2.3283067e-10), 0);
    assert_eq!(f32_to_i32(-2.3283067e-10), -1);
    assert_eq!(f32_to_i32(-6.9849204e-10), -2);
    assert_eq!(f32_to_i32(-1.1641533e-9), -3);
    assert_eq!(f32_to_i32(-1.6298147e-9), -4);
    assert_eq!(f32_to_i32(-2.095476e-9), -5);
    assert_eq!(f32_to_i32(-2.5611373e-9), -6);
    assert_eq!(f32_to_i32(-3.0267988e-9), -7);
    assert_eq!(f32_to_i32(-3.49246e-9), -8);
    assert_eq!(f32_to_i32(-0.001953125), i32::MIN / 512);
    assert_eq!(f32_to_i32(-0.00390625), i32::MIN / 256);
    assert_eq!(f32_to_i32(-0.0078125), i32::MIN / 128);
    assert_eq!(f32_to_i32(-0.015625), i32::MIN / 64);
    assert_eq!(f32_to_i32(-0.03125), i32::MIN / 32);
    assert_eq!(f32_to_i32(-0.0625), i32::MIN / 16);
    assert_eq!(f32_to_i32(-0.125), i32::MIN / 8);
    assert_eq!(f32_to_i32(-0.25), i32::MIN / 4);
    assert_eq!(f32_to_i32(-0.5), i32::MIN / 2);
    assert_eq!(f32_to_i32(-1.0), i32::MIN);
    assert_eq!(f32_to_i32(f32::INFINITY), i32::MAX);
    assert_eq!(f32_to_i32(f32::NEG_INFINITY), i32::MIN);
}
