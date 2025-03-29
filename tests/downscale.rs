use p_chan::downscale;

#[test]
fn f64_to_f32() {
    for i in 0..10 {
        assert_eq!(
            downscale::f64_to_f32(1.0 / 2.0f64.powi(i)),
            0.5f32.powi(i.try_into().unwrap()),
        );
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
