use crate::assert_cmplx_eq;
use crate::busbar::BusbarIndex;
use crate::gridsc::{
    cable_impedance, feeder_impedance, overhead_line_limpedance, transformer_impedance,
};
use crate::iec60909_4_3::iec60909_4_3;
use num_complex::Complex64;

#[test]
fn iec60909_4_3_feeder() -> Result<(), String> {
    let z_qt = Complex64::new(0.053, 0.531) / Complex64::new(1000.0, 0.0); // Ohms

    let net = iec60909_4_3();

    let z = feeder_impedance(
        net.feeders.unwrap().get(0).unwrap(),
        false,
        &BusbarIndex::new(&net.busbars.unwrap()),
    )?;
    assert_cmplx_eq!(z, z_qt, epsilon = 1e-5);

    Ok(())
}

#[test]
fn iec60909_4_3_transformer() -> Result<(), String> {
    let z1_t1 = Complex64::new(2.684, 10.054) / Complex64::new(1000.0, 0.0);
    let _z0_t1 = Complex64::new(2.684, 9.551) / Complex64::new(1000.0, 0.0);

    let z1_t2 = Complex64::new(4.712, 15.698) / Complex64::new(1000.0, 0.0);
    let _z0_t2 = Complex64::new(4.712, 14.913) / Complex64::new(1000.0, 0.0);

    let net = iec60909_4_3();

    let busbar_index = BusbarIndex::new(net.busbars.as_ref().unwrap());

    let z = transformer_impedance(
        net.transformers.as_ref().unwrap().get(0).unwrap(),
        false,
        &busbar_index,
    )?;
    assert_cmplx_eq!(z, z1_t1, epsilon = 1e-5);

    let z = transformer_impedance(
        net.transformers.as_ref().unwrap().get(1).unwrap(),
        false,
        &busbar_index,
    )?;
    assert_cmplx_eq!(z, z1_t2, epsilon = 1e-5);

    Ok(())
}

#[test]
fn iec60909_4_3_line_cable() -> Result<(), String> {
    let z_l1 = Complex64::new(0.385, 0.395) / Complex64::new(1000.0, 0.0);
    let z_l2 = Complex64::new(0.416, 0.136) / Complex64::new(1000.0, 0.0);
    let z_l3 = Complex64::new(5.420, 1.740) / Complex64::new(1000.0, 0.0);
    let z_l4 = Complex64::new(18.50, 14.85) / Complex64::new(1000.0, 0.0);

    let net = iec60909_4_3();

    let z = cable_impedance(net.cables.as_ref().unwrap().get(0).unwrap())?;
    assert_cmplx_eq!(z, z_l1, epsilon = 1e-5);

    let z = cable_impedance(net.cables.as_ref().unwrap().get(1).unwrap())?;
    assert_cmplx_eq!(z, z_l2, epsilon = 1e-5);

    let z = cable_impedance(net.cables.as_ref().unwrap().get(2).unwrap())?;
    assert_cmplx_eq!(z, z_l3, epsilon = 1e-5);

    let z = overhead_line_limpedance(
        net.frequency.unwrap(),
        net.lines.as_ref().unwrap().get(0).unwrap(),
    )?;
    assert_cmplx_eq!(z, z_l4, epsilon = 1e-4);

    Ok(())
}
