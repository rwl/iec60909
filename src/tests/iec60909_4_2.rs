use anyhow::Result;
use num_complex::Complex64;

use crate::busbar::BusbarIndex;
use crate::transformer3::{ThreeWindingTransformer, TransformerSide};
use crate::{assert_cmplx_eq, cmplx};

const ZAK: Complex64 = cmplx!(0.045714, 8.096989);
const ZBK: Complex64 = cmplx!(0.053563, -0.079062);
const ZCK: Complex64 = cmplx!(0.408568, 20.292035);

#[test]
fn iec60909_4_2() -> Result<()> {
    let tr: ThreeWindingTransformer<String> = ThreeWindingTransformer::new()
        .ur_hv(400)
        .ur_mv(120)
        .ur_lv(30)
        .sr_hv_mv(350_000)
        .sr_hv_lv(50_000)
        .sr_mv_lv(50_000)
        .ukr_hv_mv(21)
        .ukr_hv_lv(10)
        .ukr_mv_lv(7)
        .urr_hv_mv(0.26)
        .urr_hv_lv(0.16)
        .urr_mv_lv(0.16)
        .build()?;

    let (z_hv, z_mv, z_lv) = tr.impedance(TransformerSide::MV, &BusbarIndex::empty())?;

    assert_cmplx_eq!(z_hv, ZAK, epsilon = 1e-5);
    assert_cmplx_eq!(z_mv, ZBK, epsilon = 1e-5);
    assert_cmplx_eq!(z_lv, ZCK, epsilon = 1e-5);

    Ok(())
}
