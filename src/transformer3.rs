use anyhow::{format_err, Result};
use derive_builder::Builder;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::busbar::{voltage_correction_factor, BusbarIndex};
use crate::cmplx;
use crate::traits::Sq;

#[derive(Clone, Copy)]
pub enum TransformerSide {
    HV,
    MV,
    LV,
}

pub enum TransformerSides {
    HvMv,
    HvLv,
    MvLv,
}

/// Network transformer with primary, secondary and tertiary windings.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct ThreeWindingTransformer<N: Default> {
    pub node_hv: N,

    pub node_lv: N,

    pub node_mv: N,

    /// Rated voltage of the transformer on the high-voltage side (kV).
    pub ur_hv: f64,

    /// Rated voltage of the transformer on the low-voltage side (kV).
    pub ur_lv: f64,

    /// Rated voltage of the transformer on the medium-voltage side (kV).
    pub ur_mv: f64,

    /// Rated apparent power between high-voltage and low-voltage sides (kVA).
    pub sr_hv_lv: f64,

    /// Rated apparent power between high-voltage and medium-voltage sides (kVA).
    pub sr_hv_mv: f64,

    /// Rated apparent power between medium-voltage and low-voltage sides (kVA).
    pub sr_mv_lv: f64,

    /// Rated short circuit voltage between HV and LV sides in per cent.
    pub ukr_hv_lv: f64,

    /// Rated short circuit voltage between HV and MV sides in per cent.
    pub ukr_hv_mv: f64,

    /// Rated short circuit voltage between MV and LV sides in per cent.
    pub ukr_mv_lv: f64,

    /// Total loss of the transformer in the HV and LV windings at rated current (kW).
    #[builder(default)]
    pub pkr_hv_lv: f64,

    /// Total loss of the transformer in the HV and MV windings at rated current (kW).
    #[builder(default)]
    pub pkr_hv_mv: f64,

    /// Total loss of the transformer in the MV and LV windings at rated current (kW).
    #[builder(default)]
    pub pkr_mv_lv: f64,

    /// Rated resistive components of the short-circuit voltage, given in per cent between HV and
    /// LV sides.
    pub urr_hv_lv: Option<f64>,

    /// Rated resistive components of the short-circuit voltage, given in per cent between HV and
    /// MV sides.
    pub urr_hv_mv: Option<f64>,

    /// Rated resistive components of the short-circuit voltage, given in per cent between MV and
    /// LV sides.
    pub urr_mv_lv: Option<f64>,

    /// Rated reactive component of the short-circuit voltage, given in per cent between HV and MV
    /// sides.
    pub uxr_hv_lv: f64,

    /// Rated reactive component of the short-circuit voltage, given in per cent between HV and MV
    /// sides.
    pub uxr_hv_mv: f64,

    /// Rated reactive component of the short-circuit voltage, given in per cent between MV and LV
    /// sides.
    pub uxr_mv_lv: f64,

    /// Range of transformer voltage adjustment (%).
    pub p: f64,
}

impl<N: Clone + Default + Eq + core::hash::Hash> ThreeWindingTransformer<N> {
    pub fn new() -> ThreeWindingTransformerBuilder<N> {
        ThreeWindingTransformerBuilder::default()
    }

    pub fn impedance(
        &self,
        side: TransformerSide,
        busbar_index: &BusbarIndex<N>,
    ) -> Result<(Complex64, Complex64, Complex64)> {
        let zk_hv_mv = self.side_impedance(side, TransformerSides::HvMv, busbar_index)?;

        let zk_hv_lv = self.side_impedance(side, TransformerSides::HvLv, busbar_index)?;

        let zk_mv_lv = self.side_impedance(side, TransformerSides::MvLv, busbar_index)?;

        let z_hv = 0.5 * (zk_hv_mv + zk_hv_lv - zk_mv_lv);
        let z_mv = 0.5 * (zk_mv_lv + zk_hv_mv - zk_hv_lv);
        let z_lv = 0.5 * (zk_hv_lv + zk_mv_lv - zk_hv_mv);

        Ok((z_hv, z_mv, z_lv))
    }

    fn side_impedance(
        &self,
        side: TransformerSide,
        sides: TransformerSides,
        busbar_index: &BusbarIndex<N>,
    ) -> Result<Complex64> {
        let ur = match side {
            TransformerSide::HV => self.ur_hv * 1e3,
            TransformerSide::MV => self.ur_mv * 1e3,
            TransformerSide::LV => self.ur_lv * 1e3,
        };
        let (sr, ukr, urr_opt, pkr) = match sides {
            TransformerSides::HvMv => (
                self.sr_hv_mv * 1e3,
                self.ukr_hv_mv,
                self.urr_hv_mv,
                self.pkr_hv_mv * 1e3,
            ),
            TransformerSides::HvLv => (
                self.sr_hv_lv * 1e3,
                self.ukr_hv_lv,
                self.urr_hv_lv,
                self.pkr_hv_lv * 1e3,
            ),
            TransformerSides::MvLv => (
                self.sr_mv_lv * 1e3,
                self.ukr_mv_lv,
                self.urr_mv_lv,
                self.pkr_mv_lv * 1e3,
            ),
        };
        let urr = urr_opt.unwrap_or((pkr / sr) * 100.0);
        if urr > ukr {
            return Err(format_err!("uRr ({}) must be < ukr ({})", urr, ukr));
        }
        let uxr = (ukr.sq() - urr.sq()).sqrt(); // (10d)

        let z = cmplx!(urr / 100.0, uxr / 100.0) * cmplx!(ur.sq() / sr); // (10)

        let mut cmax = voltage_correction_factor(self.ur_hv, false, true);
        // if let Some(node_hv) = self.node_hv.as_ref() {
        if let Some(busbar) = busbar_index.busbar(&self.node_hv) {
            if busbar.cmax.unwrap_or_default() != 0.0 {
                cmax = busbar.cmax.unwrap();
            }
        }
        // }

        let xr = z.im / (ur.sq() / sr); // Relative reactance of the transformer.

        let k = 0.95 * (cmax / (1.0 + (0.6 * xr))); // (12a)

        let zk = cmplx!(k) * z;

        Ok(zk)
    }
}
