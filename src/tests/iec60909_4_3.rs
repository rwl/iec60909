use crate::busbar::BusbarIndex;
use crate::part4::section3::iec60909_4_3;
use crate::{assert_cmplx_eq, cmplx};
use anyhow::Result;

#[test]
fn iec60909_4_3_feeder() -> Result<()> {
    let z_qt = cmplx!(0.053, 0.531) / cmplx!(1000); // Ohms

    let net = iec60909_4_3()?;

    let z = net.feeders[0].impedance(false, &BusbarIndex::new(&net.busbars))?;
    assert_cmplx_eq!(z, z_qt, epsilon = 1e-5);

    Ok(())
}

#[test]
fn iec60909_4_3_transformer() -> Result<()> {
    let z1_t1 = cmplx!(2.684, 10.054) / cmplx!(1000);
    let _z0_t1 = cmplx!(2.684, 9.551) / cmplx!(1000);

    let z1_t2 = cmplx!(4.712, 15.698) / cmplx!(1000);
    let _z0_t2 = cmplx!(4.712, 14.913) / cmplx!(1000);

    let net = iec60909_4_3()?;

    let busbar_index = BusbarIndex::new(&net.busbars);

    let z = net.transformers[0].impedance(false, &busbar_index)?;
    assert_cmplx_eq!(z, z1_t1, epsilon = 1e-5);

    let z = net.transformers[1].impedance(false, &busbar_index)?;
    assert_cmplx_eq!(z, z1_t2, epsilon = 1e-5);

    Ok(())
}

#[test]
fn iec60909_4_3_line_cable() -> Result<()> {
    let z_l1 = cmplx!(0.385, 0.395) / cmplx!(1000);
    let z_l2 = cmplx!(0.416, 0.136) / cmplx!(1000);
    let z_l3 = cmplx!(5.420, 1.740) / cmplx!(1000);
    let z_l4 = cmplx!(18.50, 14.85) / cmplx!(1000);

    let net = iec60909_4_3()?;

    let z = net.cables[0].impedance()?;
    assert_cmplx_eq!(z, z_l1, epsilon = 1e-5);

    let z = net.cables[1].impedance()?;
    assert_cmplx_eq!(z, z_l2, epsilon = 1e-5);

    let z = net.cables[2].impedance()?;
    assert_cmplx_eq!(z, z_l3, epsilon = 1e-5);

    let z = net.lines[0].impedance(net.frequency)?;
    assert_cmplx_eq!(z, z_l4, epsilon = 1e-4);

    Ok(())
}
