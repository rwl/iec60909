use crate::assert_cmplx_eq;
use crate::busbar::BusbarIndex;
use crate::gridsc::{
    cable_impedance, feeder_impedance, generator_impedance, motor_impedance,
    overhead_line_limpedance, power_station_impedance, three_winding_transformer_impedance,
    transformer_impedance, TransformerSide,
};
use crate::iec60909_4_6::iec60909_4_6;
use num_complex::Complex64;

#[test]
fn iec60909_4_6_feeder() -> Result<(), String> {
    let z_q1 = Complex64::new(0.631933, 6.319335);
    let z_q1t = Complex64::new(0.056874, 0.568740);
    let _z_q2 = Complex64::new(0.434454, 4.344543);

    let mut net = iec60909_4_6();

    let q1 = net.feeders.as_mut().unwrap().get_mut(0).unwrap();
    let tr = q1.tr.unwrap();
    q1.tr = Some(0.0);
    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let z = feeder_impedance(q1, false, &busbar_index)?;
    assert_cmplx_eq!(z, z_q1, epsilon = 1e-6);

    q1.tr = Some(tr);

    let z = feeder_impedance(q1, false, &busbar_index)?;
    assert_cmplx_eq!(z, z_q1t, epsilon = 1e-6);

    Ok(())
}

#[test]
fn iec60909_4_6_three_winding_transformer() -> Result<(), String> {
    let z3amv = Complex64::new(0.045714, 8.096989); // KTAB = 0.928072
    let z3bmv = Complex64::new(0.053563, -0.079062); // KTAC = 0.985856
    let z3cmv = Complex64::new(0.408568, 20.292035); // KTBC = 1.002890

    let net = iec60909_4_6();

    let t3 = net
        .three_winding_transformers
        .as_ref()
        .unwrap()
        .get(0)
        .unwrap();
    let t4 = net
        .three_winding_transformers
        .as_ref()
        .unwrap()
        .get(1)
        .unwrap();
    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let (z_hv, z_mv, z_lv) =
        three_winding_transformer_impedance(t3, TransformerSide::MV, &busbar_index)?;

    assert_cmplx_eq!(z_hv, z3amv, epsilon = 1e-5);
    assert_cmplx_eq!(z_mv, z3bmv, epsilon = 1e-5);
    assert_cmplx_eq!(z_lv, z3cmv, epsilon = 1e-5);

    let (z_hv, z_mv, z_lv) =
        three_winding_transformer_impedance(t4, TransformerSide::MV, &busbar_index)?;

    assert_cmplx_eq!(z_hv, z3amv, epsilon = 1e-5);
    assert_cmplx_eq!(z_mv, z3bmv, epsilon = 1e-5);
    assert_cmplx_eq!(z_lv, z3cmv, epsilon = 1e-5);

    Ok(())
}

#[test]
fn iec60909_4_6_transformer() -> Result<(), String> {
    let z5mv = Complex64::new(2.046454, 49.072241); // KT = 0.974870

    let net = iec60909_4_6();

    let t5 = net.transformers.as_ref().unwrap().get(0).unwrap();
    let t6 = net.transformers.as_ref().unwrap().get(1).unwrap();
    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let z = transformer_impedance(t5, true, &busbar_index)?;
    assert_cmplx_eq!(z, z5mv, epsilon = 1e-5);

    let z = transformer_impedance(t6, true, &busbar_index)?;
    assert_cmplx_eq!(z, z5mv, epsilon = 1e-5);

    Ok(())
}

#[test]
fn iec60909_4_6_power_station() -> Result<(), String> {
    let z_s1 = Complex64::new(0.498795, 26.336676); // KS1 = 0.995975
    let z_s2 = Complex64::new(1.203944, 35.340713); // KS2 = 0.876832

    let tol = 1e-6;

    let net = iec60909_4_6();

    let s1 = net.power_stations.as_ref().unwrap().get(0).unwrap();
    let s2 = net.power_stations.as_ref().unwrap().get(1).unwrap();
    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let z1 = power_station_impedance(s1, true, true, false, &busbar_index)?;
    assert_cmplx_eq!(z1, z_s1, epsilon = tol);

    let z2 = power_station_impedance(s2, true, false, false, &busbar_index)?;
    assert_cmplx_eq!(z2, z_s2, epsilon = tol);

    Ok(())
}

#[test]
fn iec60909_4_6_generator() -> Result<(), String> {
    let z_g3 = Complex64::new(0.017790, 1.089623); // KG = 0.988320
    let _z_g3t = Complex64::new(2.133964, 130.705301); // TODO: Referred to the 110 kV side

    let net = iec60909_4_6();

    let g3 = net.generators.as_ref().unwrap().get(0).unwrap();
    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let z = generator_impedance(g3, 1.0, false, &busbar_index)?;
    assert_cmplx_eq!(z, z_g3, epsilon = 1e-6);

    Ok(())
}

#[test]
fn iec60909_4_6_motor() -> Result<(), String> {
    let z_m1 = Complex64::new(0.341497, 3.414968);
    let _z_m1t = Complex64::new(40.964124, 409.641243);

    let z_m2 = Complex64::new(0.412137, 4.121368); // 2 parallel motors 2MW
    let _z_m2t = Complex64::new(49.437719, 494.377190);

    // ZM1t parallel to ZM2t: 22.401898 + 224.018979i Ω

    let net = iec60909_4_6();

    let m1 = net.motors.as_ref().unwrap().get(0).unwrap();
    let m2 = net.motors.as_ref().unwrap().get(1).unwrap();

    let z1 = motor_impedance(m1)?;
    assert_cmplx_eq!(z1, z_m1, epsilon = 1e-6);

    let z2 = motor_impedance(m2)?;
    assert_cmplx_eq!(z2, z_m2, epsilon = 1e-6);

    Ok(())
}

#[test]
fn iec60909_4_6_line() -> Result<(), String> {
    let z_l1 = Complex64::new(2.4, 7.8);
    let z_l2 = Complex64::new(1.2, 3.9);
    let z_l3 = Complex64::new(0.3, 0.975); // double line
    let z_l4 = Complex64::new(0.96, 3.88);
    let z_l5 = Complex64::new(1.8, 5.79);
    let z_l6 = Complex64::new(0.082, 0.086); // 10kV cable
    let _z_l6t = Complex64::new(9.836281, 10.316100);

    let tol = 1e-9;

    let net = iec60909_4_6();

    let l1 = net.lines.as_ref().unwrap().get(0).unwrap();
    let l2 = net.lines.as_ref().unwrap().get(1).unwrap();
    let l3 = net.lines.as_ref().unwrap().get(2).unwrap();
    let l4 = net.lines.as_ref().unwrap().get(3).unwrap();
    let l5 = net.lines.as_ref().unwrap().get(4).unwrap();

    let l6 = net.cables.as_ref().unwrap().get(0).unwrap();

    let z1 = overhead_line_limpedance(net.frequency.unwrap(), l1)?;
    assert_cmplx_eq!(z1, z_l1, epsilon = tol);

    let z2 = overhead_line_limpedance(net.frequency.unwrap(), l2)?;
    assert_cmplx_eq!(z2, z_l2, epsilon = tol);

    let z3 = overhead_line_limpedance(net.frequency.unwrap(), l3)?;
    assert_cmplx_eq!(z3, z_l3, epsilon = tol);

    let z4 = overhead_line_limpedance(net.frequency.unwrap(), l4)?;
    assert_cmplx_eq!(z4, z_l4, epsilon = tol);

    let z5 = overhead_line_limpedance(net.frequency.unwrap(), l5)?;
    assert_cmplx_eq!(z5, z_l5, epsilon = tol);

    let z6 = cable_impedance(l6)?;
    assert_cmplx_eq!(z6, z_l6, epsilon = tol);

    Ok(())
}
