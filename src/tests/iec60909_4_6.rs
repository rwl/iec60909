use anyhow::Result;

use crate::busbar::BusbarIndex;
use crate::part4::section6::iec60909_4_6;
use crate::transformer3::TransformerSide;
use crate::{assert_cmplx_eq, cmplx};

#[test]
fn iec60909_4_6_feeder() -> Result<()> {
    let z_q1 = cmplx!(0.631933, 6.319335);
    let z_q1t = cmplx!(0.056874, 0.568740);
    let _z_q2 = cmplx!(0.434454, 4.344543);

    let mut net = iec60909_4_6()?;

    let q1 = &mut net.feeders[0];
    let tr = q1.tr.take();
    let busbar_index = BusbarIndex::new(&net.busbars);

    let z = q1.impedance(false, &busbar_index)?;
    assert_cmplx_eq!(z, z_q1, epsilon = 1e-6);

    q1.tr = tr;

    let z = q1.impedance(false, &busbar_index)?;
    assert_cmplx_eq!(z, z_q1t, epsilon = 1e-6);

    Ok(())
}

#[test]
fn iec60909_4_6_three_winding_transformer() -> Result<()> {
    let z3amv = cmplx!(0.045714, 8.096989); // KTAB = 0.928072
    let z3bmv = cmplx!(0.053563, -0.079062); // KTAC = 0.985856
    let z3cmv = cmplx!(0.408568, 20.292035); // KTBC = 1.002890

    let net = iec60909_4_6()?;

    let t3 = &net.three_winding_transformers[0];
    let t4 = &net.three_winding_transformers[1];
    let busbar_index = BusbarIndex::new(&net.busbars);

    let (z_hv, z_mv, z_lv) = t3.impedance(TransformerSide::MV, &busbar_index)?;

    assert_cmplx_eq!(z_hv, z3amv, epsilon = 1e-5);
    assert_cmplx_eq!(z_mv, z3bmv, epsilon = 1e-5);
    assert_cmplx_eq!(z_lv, z3cmv, epsilon = 1e-5);

    let (z_hv, z_mv, z_lv) = t4.impedance(TransformerSide::MV, &busbar_index)?;

    assert_cmplx_eq!(z_hv, z3amv, epsilon = 1e-5);
    assert_cmplx_eq!(z_mv, z3bmv, epsilon = 1e-5);
    assert_cmplx_eq!(z_lv, z3cmv, epsilon = 1e-5);

    Ok(())
}

#[test]
fn iec60909_4_6_transformer() -> Result<()> {
    let z5mv = cmplx!(2.046454, 49.072241); // KT = 0.974870

    let net = iec60909_4_6()?;

    let t5 = &net.transformers[0];
    let t6 = &net.transformers[1];
    let busbar_index = BusbarIndex::new(&net.busbars);

    let z = t5.impedance(true, &busbar_index)?;
    assert_cmplx_eq!(z, z5mv, epsilon = 1e-5);

    let z = t6.impedance(true, &busbar_index)?;
    assert_cmplx_eq!(z, z5mv, epsilon = 1e-5);

    Ok(())
}

#[test]
fn iec60909_4_6_power_station() -> Result<()> {
    let z_s1 = cmplx!(0.498795, 26.336676); // KS1 = 0.995975
    let z_s2 = cmplx!(1.203944, 35.340713); // KS2 = 0.876832

    let tol = 1e-6;

    let net = iec60909_4_6()?;

    let s1 = &net.power_stations[0];
    let s2 = &net.power_stations[1];
    let busbar_index = BusbarIndex::new(&net.busbars);

    let z1 = s1.impedance(true, true, false, &busbar_index)?;
    assert_cmplx_eq!(z1, z_s1, epsilon = tol);

    let z2 = s2.impedance(true, false, false, &busbar_index)?;
    assert_cmplx_eq!(z2, z_s2, epsilon = tol);

    Ok(())
}

#[test]
fn iec60909_4_6_generator() -> Result<()> {
    let z_g3 = cmplx!(0.017790, 1.089623); // KG = 0.988320
    let _z_g3t = cmplx!(2.133964, 130.705301); // TODO: Referred to the 110 kV side

    let net = iec60909_4_6()?;

    let g3 = &net.generators[0];
    let busbar_index = BusbarIndex::new(&net.busbars);

    let z = g3.impedance(1.0, false, &busbar_index)?;
    assert_cmplx_eq!(z, z_g3, epsilon = 1e-6);

    Ok(())
}

#[test]
fn iec60909_4_6_motor() -> Result<()> {
    let z_m1 = cmplx!(0.341497, 3.414968);
    let _z_m1t = cmplx!(40.964124, 409.641243);

    let z_m2 = cmplx!(0.412137, 4.121368); // 2 parallel motors 2MW
    let _z_m2t = cmplx!(49.437719, 494.377190);

    // ZM1t parallel to ZM2t: 22.401898 + 224.018979i Ω

    let net = iec60909_4_6()?;

    let m1 = &net.motors[0];
    let m2 = &net.motors[1];

    let z1 = m1.impedance()?;
    assert_cmplx_eq!(z1, z_m1, epsilon = 1e-6);

    let z2 = m2.impedance()?;
    assert_cmplx_eq!(z2, z_m2, epsilon = 1e-6);

    Ok(())
}

#[test]
fn iec60909_4_6_line() -> Result<()> {
    let z_l1 = cmplx!(2.4, 7.8);
    let z_l2 = cmplx!(1.2, 3.9);
    let z_l3 = cmplx!(0.3, 0.975); // double line
    let z_l4 = cmplx!(0.96, 3.88);
    let z_l5 = cmplx!(1.8, 5.79);
    let z_l6 = cmplx!(0.082, 0.086); // 10kV cable
    let _z_l6t = cmplx!(9.836281, 10.316100);

    let tol = 1e-9;

    let net = iec60909_4_6()?;

    let l1 = &net.lines[0];
    let l2 = &net.lines[1];
    let l3 = &net.lines[2];
    let l4 = &net.lines[3];
    let l5 = &net.lines[4];

    let l6 = &net.cables[0];

    let z1 = l1.impedance(net.frequency)?;
    assert_cmplx_eq!(z1, z_l1, epsilon = tol);

    let z2 = l2.impedance(net.frequency)?;
    assert_cmplx_eq!(z2, z_l2, epsilon = tol);

    let z3 = l3.impedance(net.frequency)?;
    assert_cmplx_eq!(z3, z_l3, epsilon = tol);

    let z4 = l4.impedance(net.frequency)?;
    assert_cmplx_eq!(z4, z_l4, epsilon = tol);

    let z5 = l5.impedance(net.frequency)?;
    assert_cmplx_eq!(z5, z_l5, epsilon = tol);

    let z6 = l6.impedance()?;
    assert_cmplx_eq!(z6, z_l6, epsilon = tol);

    Ok(())
}
