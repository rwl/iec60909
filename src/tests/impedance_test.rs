use anyhow::Result;
use num_complex::Complex64;

use crate::{assert_cmplx_eq, part4::section3::iec60909_4_3};

#[test]
fn test_fault_impedance() -> Result<()> {
    const ZK_T1: Complex64 = Complex64 {
        re: 1.881 / 1000.0,
        im: 6.746 / 1000.0,
    };

    let net = iec60909_4_3()?;
    let solver = spsolve::rlu::RLU::default();

    let zk = net.fault_impedance(solver)?;

    assert_cmplx_eq!(zk["T1"], ZK_T1, epsilon = 1e-6);

    Ok(())
}
