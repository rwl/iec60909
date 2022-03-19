use crate::assert_cmplx_eq;
use crate::busbar::BusbarIndex;
use crate::gridsc::{cable_impedance, feeder_impedance, motor_impedance, transformer_impedance};
use crate::iec60909_4_4::iec60909_4_4;
use num_complex::Complex64;

#[test]
fn iec60909_4_4_feeder() -> Result<(), String> {
    let z_qt = Complex64::new(0.0058, 0.0579);

    let net = iec60909_4_4();

    let z = feeder_impedance(
        net.feeders.unwrap().get(0).unwrap(),
        false,
        &BusbarIndex::new(&net.busbars.unwrap()),
    )?;

    assert_cmplx_eq!(z, z_qt, epsilon = 1e-4);

    Ok(())
}

#[test]
fn iec60909_4_4_cable() -> Result<(), String> {
    let zl = Complex64::new(0.0177, 0.0177);

    let net = iec60909_4_4();

    let z = cable_impedance(net.cables.as_ref().unwrap().get(0).unwrap())?;

    assert_cmplx_eq!(z, zl, epsilon = 1e-4);

    let z = cable_impedance(net.cables.as_ref().unwrap().get(1).unwrap())?;

    assert_cmplx_eq!(z, zl, epsilon = 1e-4);

    Ok(())
}

#[test]
fn iec60909_4_4_transformer() -> Result<(), String> {
    let zt = Complex64::new(0.0152, 0.3803);

    let net = iec60909_4_4();

    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let z = transformer_impedance(
        net.transformers.as_ref().unwrap().get(0).unwrap(),
        false,
        &busbar_index,
    )?;

    assert_cmplx_eq!(z, zt, epsilon = 1e-4);

    let z = transformer_impedance(
        net.transformers.as_ref().unwrap().get(1).unwrap(),
        false,
        &busbar_index,
    )?;

    assert_cmplx_eq!(z, zt, epsilon = 1e-4);

    Ok(())
}

#[test]
fn iec60909_4_4_cable_transformer() -> Result<(), String> {
    let z_lt = Complex64::new(0.0329, 0.3980);
    let z_ltp = Complex64::new(0.0165, 0.1990);
    let z_shc = Complex64::new(0.0223, 0.2569);

    let net = iec60909_4_4();

    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let zl = cable_impedance(net.cables.as_ref().unwrap().get(0).unwrap())?;
    let zt = transformer_impedance(
        net.transformers.as_ref().unwrap().get(0).unwrap(),
        false,
        &busbar_index,
    )?;
    assert_cmplx_eq!(zl + zt, z_lt, epsilon = 1e-4);

    let zl = cable_impedance(net.cables.as_ref().unwrap().get(1).unwrap())?;
    let zt = transformer_impedance(
        net.transformers.as_ref().unwrap().get(1).unwrap(),
        false,
        &busbar_index,
    )?;
    assert_cmplx_eq!(zl + zt, z_lt, epsilon = 1e-4);

    let zp = (zl + zt) / Complex64::new(2.0, 0.0);
    assert_cmplx_eq!(zp, z_ltp, epsilon = 1e-4);

    let zq = feeder_impedance(net.feeders.unwrap().get(0).unwrap(), false, &busbar_index)?;
    assert_cmplx_eq!(zq + zp, z_shc, epsilon = 1e-4);

    Ok(())
}

#[test]
fn iec60909_4_4_motor() -> Result<(), String> {
    //zm1 = 1.500
    //zm2 = 1.703 //1.705 Sr rounding
    let zm1 = Complex64::new(0.149, 1.494); // 1.493i
    let zm2 = Complex64::new(0.170, 1.694); // 1.696i

    let net = iec60909_4_4();

    let zm = motor_impedance(net.motors.as_ref().unwrap().get(0).unwrap())?;
    assert_cmplx_eq!(zm, zm1, epsilon = 1e-3);

    let zm = motor_impedance(net.motors.as_ref().unwrap().get(1).unwrap())?;
    assert_cmplx_eq!(zm, zm2, epsilon = 1e-3);

    Ok(())
}
