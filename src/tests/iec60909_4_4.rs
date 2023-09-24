use anyhow::Result;

use crate::busbar::BusbarIndex;
use crate::part4::iec60909_4_4;
use crate::{assert_cmplx_eq, cmplx};

#[test]
fn iec60909_4_4_feeder() -> Result<()> {
    let z_qt = cmplx!(0.0058, 0.0579);

    let net = iec60909_4_4()?;
    let index = &BusbarIndex::new(&net.busbars);

    let z = net.feeders[0].impedance(false, &index)?;

    assert_cmplx_eq!(z, z_qt, epsilon = 1e-4);

    Ok(())
}

#[test]
fn iec60909_4_4_cable() -> Result<()> {
    let zl = cmplx!(0.0177, 0.0177);

    let net = iec60909_4_4()?;

    let z = net.cables[0].impedance()?;
    assert_cmplx_eq!(z, zl, epsilon = 1e-4);

    let z = net.cables[1].impedance()?;
    assert_cmplx_eq!(z, zl, epsilon = 1e-4);

    Ok(())
}

#[test]
fn iec60909_4_4_transformer() -> Result<()> {
    let zt = cmplx!(0.0152, 0.3803);

    let net = iec60909_4_4()?;
    let busbar_index = BusbarIndex::new(&net.busbars);

    let z = net.transformers[0].impedance(false, &busbar_index)?;
    assert_cmplx_eq!(z, zt, epsilon = 1e-4);

    let z = net.transformers[1].impedance(false, &busbar_index)?;
    assert_cmplx_eq!(z, zt, epsilon = 1e-4);

    Ok(())
}

#[test]
fn iec60909_4_4_cable_transformer() -> Result<()> {
    let z_lt = cmplx!(0.0329, 0.3980);
    let z_ltp = cmplx!(0.0165, 0.1990);
    let z_shc = cmplx!(0.0223, 0.2569);

    let net = iec60909_4_4()?;
    let busbar_index = BusbarIndex::new(&net.busbars);

    let zl = net.cables[0].impedance()?;
    let zt = net.transformers[0].impedance(false, &busbar_index)?;
    assert_cmplx_eq!(zl + zt, z_lt, epsilon = 1e-4);

    let zl = net.cables[1].impedance()?;
    let zt = net.transformers[1].impedance(false, &busbar_index)?;
    assert_cmplx_eq!(zl + zt, z_lt, epsilon = 1e-4);

    let zp = (zl + zt) / cmplx!(2);
    assert_cmplx_eq!(zp, z_ltp, epsilon = 1e-4);

    let zq = net.feeders[0].impedance(false, &busbar_index)?;
    assert_cmplx_eq!(zq + zp, z_shc, epsilon = 1e-4);

    Ok(())
}

#[test]
fn iec60909_4_4_motor() -> Result<()> {
    //zm1 = 1.500
    //zm2 = 1.703 //1.705 Sr rounding
    let zm1 = cmplx!(0.149, 1.494); // 1.493i
    let zm2 = cmplx!(0.170, 1.694); // 1.696i

    let net = iec60909_4_4()?;

    let zm = net.motors[0].impedance()?;
    assert_cmplx_eq!(zm, zm1, epsilon = 1e-3);

    let zm = net.motors[1].impedance()?;
    assert_cmplx_eq!(zm, zm2, epsilon = 1e-3);

    Ok(())
}
