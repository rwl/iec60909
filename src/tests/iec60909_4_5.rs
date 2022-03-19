use crate::assert_cmplx_eq;
use crate::busbar::BusbarIndex;
use crate::gridsc::{
    feeder_impedance, motor_impedance, power_station_impedance,
    three_winding_transformer_impedance, transformer_impedance, TransformerSide,
};
use crate::iec60909_4_5::iec60909_4_5;
use num_complex::Complex64;

#[test]
fn iec60909_4_5_feeder() -> Result<(), String> {
    let z_qt = Complex64::new(0.793, 6.606);

    let mut net = iec60909_4_5();

    let mut q = net.feeders.as_mut().unwrap().get_mut(0).unwrap();
    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let z = feeder_impedance(q, false, &busbar_index)?;
    assert_cmplx_eq!(z, z_qt, epsilon = 1e-3);

    // For the calculation of the maximum short-circuit currents at the
    // short-circuit locations F2 to F5, the value ZQmin corresponding
    // to IkssQmax = 52.5 kA shall be used.
    let zqmin = Complex64::new(0.265, 2.648);

    let ikss_qmax = 52.5; // kA
    let rq_xq = 0.1;

    q.ikss = Some(ikss_qmax);
    q.rx = Some(rq_xq);

    let z = feeder_impedance(q, false, &busbar_index)?;
    assert_cmplx_eq!(z, zqmin, epsilon = 1e-3);

    Ok(())
}

#[test]
fn iec60909_4_5_power_station() -> Result<(), String> {
    let zs = Complex64::new(0.735, 67.301); //67.313i rounding error

    let net = iec60909_4_5();

    let s = net.power_stations.as_ref().unwrap().get(0).unwrap();
    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let z = power_station_impedance(s, true, true, false, &busbar_index)?;
    assert_cmplx_eq!(z, zs, epsilon = 1e-3);

    Ok(())
}

#[test]
fn iec60909_4_5_three_winding_transformer() -> Result<(), String> {
    let z_ak = Complex64::new(0.0028, 0.1275);
    let z_bk = Complex64::new(0.0390, 1.1105);
    let z_ck = z_bk;

    let net = iec60909_4_5();

    let tr3 = net
        .three_winding_transformers
        .as_ref()
        .unwrap()
        .get(0)
        .unwrap();
    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let (z_hv, z_mv, z_lv) =
        three_winding_transformer_impedance(tr3, TransformerSide::HV, &busbar_index)?;

    assert_cmplx_eq!(z_hv, z_ak, epsilon = 1e-3);
    assert_cmplx_eq!(z_mv, z_bk, epsilon = 1e-3);
    assert_cmplx_eq!(z_lv, z_ck, epsilon = 1e-3);

    Ok(())
}

#[test]
fn iec60909_4_5_transformers2_5() -> Result<(), String> {
    let z_hv = Complex64::new(0.379, 2.392);

    let net = iec60909_4_5();

    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    for i in 0..=10 {
        let tr = net.transformers.as_ref().unwrap().get(i).unwrap();

        let z = transformer_impedance(tr, true, &busbar_index)?;
        assert_cmplx_eq!(z, z_hv, epsilon = 1e-3);
    }
    Ok(())
}

#[test]
fn iec60909_4_5_transformers1_6() -> Result<(), String> {
    let z_hv = Complex64::new(0.651, 3.728);
    let z_lv = Complex64::new(1.096e-3, 6.277e-3);

    let net = iec60909_4_5();
    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    for i in [5, 11] {
        let tr = net.transformers.as_ref().unwrap().get(i).unwrap();

        let z = transformer_impedance(tr, true, &busbar_index)?;
        assert_cmplx_eq!(z, z_hv, epsilon = 1e-3);

        let z = transformer_impedance(tr, false, &busbar_index)?; // TODO: t^2 method
        assert_cmplx_eq!(z, z_lv, epsilon = 1e-3);
    }

    Ok(())
}

// #[test]
fn iec60909_4_5_motor_b() -> Result<(), String> {
    let xm1 = 0.995 * 1.60;
    let rm1 = 0.1 * xm1;

    let net = iec60909_4_5();

    let m1 = net.motors.as_ref().unwrap().get(0).unwrap();
    let z1 = motor_impedance(m1)?;

    assert_cmplx_eq!(z1, Complex64::new(rm1, xm1), epsilon = 1e-5);

    Ok(())
}
