use p_chan::convert::{
    f64_to_i64, f64_to_u64, i64_to_f64, i64_to_u64, u64_to_f64, u64_to_i64,
};

#[test]
fn unsigned_to_float() {
    for i in 0..10 {
        assert_eq!(
            u64_to_f64(u64::MAX / 2u64.pow(i)),
            0.5f64.powi(i.try_into().unwrap()),
        );
    }

    assert_eq!(u64_to_f64(0), 0.0);
    assert_eq!(u64_to_f64(1), 5.421010862427523e-20);
    assert_eq!(u64_to_f64(2), 1.0842021724855047e-19);
    assert_eq!(u64_to_f64(3), 1.6263032587282571e-19);
    assert_eq!(u64_to_f64(4), 2.1684043449710093e-19);
    assert_eq!(u64_to_f64(5), 2.7105054312137616e-19);
    assert_eq!(u64_to_f64(6), 3.2526065174565143e-19);
    assert_eq!(u64_to_f64(7), 3.7947076036992665e-19);
    assert_eq!(u64_to_f64(8), 4.336808689942019e-19);
    assert_eq!(u64_to_f64(9), 4.878909776184771e-19);
}

#[test]
fn signed_to_float() {
    // Since there are more negative than positive integers, zeros are not
    // an exact match between integer and floating point
    assert_eq!(i64_to_f64(i64::MAX), 1.0);
    assert_eq!(i64_to_f64(i64::MAX / 2), 0.5);
    assert_eq!(i64_to_f64(i64::MAX / 4), 0.25);
    assert_eq!(i64_to_f64(i64::MAX / 8), 0.125);
    assert_eq!(i64_to_f64(i64::MAX / 16), 0.0625);
    assert_eq!(i64_to_f64(i64::MAX / 32), 0.03125);
    assert_eq!(i64_to_f64(i64::MAX / 64), 0.015625);
    assert_eq!(i64_to_f64(i64::MAX / 128), 0.0078125);
    assert_eq!(i64_to_f64(i64::MAX / 256), 0.00390625);
    assert_eq!(i64_to_f64(i64::MAX / 512), 0.001953125);
    assert_eq!(i64_to_f64(7), 3.49246e-9);
    assert_eq!(i64_to_f64(6), 3.0267988e-9);
    assert_eq!(i64_to_f64(5), 2.5611373e-9);
    assert_eq!(i64_to_f64(4), 2.095476e-9);
    assert_eq!(i64_to_f64(3), 1.6298147e-9);
    assert_eq!(i64_to_f64(2), 1.1641533e-9);
    assert_eq!(i64_to_f64(1), 6.9849204e-10);
    assert_eq!(i64_to_f64(0), 2.3283067e-10);
    assert_eq!(i64_to_f64(-1), -2.3283067e-10);
    assert_eq!(i64_to_f64(-2), -6.9849204e-10);
    assert_eq!(i64_to_f64(-3), -1.1641533e-9);
    assert_eq!(i64_to_f64(-4), -1.6298147e-9);
    assert_eq!(i64_to_f64(-5), -2.095476e-9);
    assert_eq!(i64_to_f64(-6), -2.5611373e-9);
    assert_eq!(i64_to_f64(-7), -3.0267988e-9);
    assert_eq!(i64_to_f64(-8), -3.49246e-9);
    assert_eq!(i64_to_f64(i64::MIN / 512), -0.001953125);
    assert_eq!(i64_to_f64(i64::MIN / 256), -0.00390625);
    assert_eq!(i64_to_f64(i64::MIN / 128), -0.0078125);
    assert_eq!(i64_to_f64(i64::MIN / 64), -0.015625);
    assert_eq!(i64_to_f64(i64::MIN / 32), -0.03125);
    assert_eq!(i64_to_f64(i64::MIN / 16), -0.0625);
    assert_eq!(i64_to_f64(i64::MIN / 8), -0.125);
    assert_eq!(i64_to_f64(i64::MIN / 4), -0.25);
    assert_eq!(i64_to_f64(i64::MIN / 2), -0.5);
    assert_eq!(i64_to_f64(i64::MIN), -1.0);
}

#[test]
fn float_to_unsigned() {
    for i in 0..10 {
        assert_eq!(
            f64_to_u64(0.5f64.powi(i.try_into().unwrap())),
            u64::MAX / 2u64.pow(i),
        );
    }

    assert_eq!(f64_to_u64(0.0), 0);
    assert_eq!(f64_to_u64(2.3283067e-10), 1);
    assert_eq!(f64_to_u64(4.6566134e-10), 2);
    assert_eq!(f64_to_u64(6.9849204e-10), 3);
    assert_eq!(f64_to_u64(9.313227e-10), 4);
    assert_eq!(f64_to_u64(1.1641533e-9), 5);
    assert_eq!(f64_to_u64(1.3969841e-9), 6);
    assert_eq!(f64_to_u64(1.6298147e-9), 7);
    assert_eq!(f64_to_u64(1.8626454e-9), 8);
    assert_eq!(f64_to_u64(2.095476e-9), 9);
    assert_eq!(f64_to_u64(f64::INFINITY), u64::MAX);
    assert_eq!(f64_to_u64(f64::NEG_INFINITY), u64::MIN);
}

#[test]
fn float_to_signed() {
    assert_eq!(f64_to_i64(0.0), 0);
    assert_eq!(f64_to_i64(1.0), i64::MAX);
    assert_eq!(f64_to_i64(0.5), i64::MAX / 2);
    assert_eq!(f64_to_i64(0.25), i64::MAX / 4);
    assert_eq!(f64_to_i64(0.125), i64::MAX / 8);
    assert_eq!(f64_to_i64(0.0625), i64::MAX / 16);
    assert_eq!(f64_to_i64(0.03125), i64::MAX / 32);
    assert_eq!(f64_to_i64(0.015625), i64::MAX / 64);
    assert_eq!(f64_to_i64(0.0078125), i64::MAX / 128);
    assert_eq!(f64_to_i64(0.00390625), i64::MAX / 256);
    assert_eq!(f64_to_i64(0.001953125), i64::MAX / 512);
    assert_eq!(f64_to_i64(3.49246e-9), 7);
    assert_eq!(f64_to_i64(3.0267988e-9), 6);
    assert_eq!(f64_to_i64(2.5611373e-9), 5);
    assert_eq!(f64_to_i64(2.095476e-9), 4);
    assert_eq!(f64_to_i64(1.6298147e-9), 3);
    assert_eq!(f64_to_i64(1.1641533e-9), 2);
    assert_eq!(f64_to_i64(6.9849204e-10), 1);
    assert_eq!(f64_to_i64(2.3283067e-10), 0);
    assert_eq!(f64_to_i64(-2.3283067e-10), -1);
    assert_eq!(f64_to_i64(-6.9849204e-10), -2);
    assert_eq!(f64_to_i64(-1.1641533e-9), -3);
    assert_eq!(f64_to_i64(-1.6298147e-9), -4);
    assert_eq!(f64_to_i64(-2.095476e-9), -5);
    assert_eq!(f64_to_i64(-2.5611373e-9), -6);
    assert_eq!(f64_to_i64(-3.0267988e-9), -7);
    assert_eq!(f64_to_i64(-3.49246e-9), -8);
    assert_eq!(f64_to_i64(-0.001953125), i64::MIN / 512);
    assert_eq!(f64_to_i64(-0.00390625), i64::MIN / 256);
    assert_eq!(f64_to_i64(-0.0078125), i64::MIN / 128);
    assert_eq!(f64_to_i64(-0.015625), i64::MIN / 64);
    assert_eq!(f64_to_i64(-0.03125), i64::MIN / 32);
    assert_eq!(f64_to_i64(-0.0625), i64::MIN / 16);
    assert_eq!(f64_to_i64(-0.125), i64::MIN / 8);
    assert_eq!(f64_to_i64(-0.25), i64::MIN / 4);
    assert_eq!(f64_to_i64(-0.5), i64::MIN / 2);
    assert_eq!(f64_to_i64(-1.0), i64::MIN);
    assert_eq!(f64_to_i64(f64::INFINITY), i64::MAX);
    assert_eq!(f64_to_i64(f64::NEG_INFINITY), i64::MIN);
}

#[test]
fn unsigned_to_signed() {
    assert_eq!(u64_to_i64(u64::MIN), i64::MIN);
    assert_eq!(u64_to_i64(u64::MAX / 2), -1);
    assert_eq!(u64_to_i64(u64::MAX / 2 + 1), 0);
    assert_eq!(u64_to_i64(u64::MAX), i64::MAX);
}

#[test]
fn signed_to_unsigned() {
    assert_eq!(i64_to_u64(i64::MIN), u64::MIN);
    assert_eq!(i64_to_u64(-1), u64::MAX / 2);
    assert_eq!(i64_to_u64(0), u64::MAX / 2 + 1);
    assert_eq!(i64_to_u64(i64::MAX), u64::MAX);
}
