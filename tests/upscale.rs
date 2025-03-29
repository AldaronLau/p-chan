use p_chan::upscale;

#[test]
fn f32_to_f64() {
    for i in 0..10 {
        assert_eq!(
            upscale::f32_to_f64(1.0 / 2.0f32.powi(i)),
            0.5f64.powi(i.try_into().unwrap()),
        );
    }

    assert_eq!(upscale::f32_to_f64(0.0), 0.0);
    assert_eq!(upscale::f32_to_f64(2.3283067e-10), 2.3283067e-10);
    assert_eq!(upscale::f32_to_f64(4.6566134e-10), 4.6566134e-10);
    assert_eq!(upscale::f32_to_f64(6.9849204e-10), 6.9849204e-10);
    assert_eq!(upscale::f32_to_f64(9.313227e-10), 9.313227e-10);
    assert_eq!(upscale::f32_to_f64(1.1641533e-9), 1.1641533e-9);
    assert_eq!(upscale::f32_to_f64(1.3969841e-9), 1.3969841e-9);
    assert_eq!(upscale::f32_to_f64(1.6298147e-9), 1.6298147e-9);
    assert_eq!(upscale::f32_to_f64(1.8626454e-9), 1.8626454e-9);
    assert_eq!(upscale::f32_to_f64(2.095476e-9), 2.095476e-9);
}
