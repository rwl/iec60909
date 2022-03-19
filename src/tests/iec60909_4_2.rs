use crate::assert_cmplx_eq;
use crate::busbar::BusbarIndex;
use crate::gridsc::{three_winding_transformer_impedance, TransformerSide};
use gridjson::ThreeWindingTransformer;
use num_complex::Complex64;

const ZAK: Complex64 = Complex64::new(0.045714, 8.096989);
const ZBK: Complex64 = Complex64::new(0.053563, -0.079062);
const ZCK: Complex64 = Complex64::new(0.408568, 20.292035);

#[test]
fn iec60909_4_2() -> Result<(), String> {
    let tr = ThreeWindingTransformer {
        ur_hv: Some(400.0),
        ur_mv: Some(120.0),
        ur_lv: Some(30.0),

        sr_hv_mv: Some(350e3),
        sr_hv_lv: Some(50e3),
        sr_mv_lv: Some(50e3),

        ukr_hv_mv: Some(21.0),
        ukr_hv_lv: Some(10.0),
        ukr_mv_lv: Some(7.0),

        urr_hv_mv: Some(0.26),
        urr_hv_lv: Some(0.16),
        urr_mv_lv: Some(0.16),

        ..Default::default()
    };

    let (z_hv, z_mv, z_lv) =
        three_winding_transformer_impedance(&tr, TransformerSide::MV, &BusbarIndex::empty())?;

    assert_cmplx_eq!(z_hv, ZAK, epsilon = 1e-5);
    assert_cmplx_eq!(z_mv, ZBK, epsilon = 1e-5);
    assert_cmplx_eq!(z_lv, ZCK, epsilon = 1e-5);

    Ok(())
}
