use anyhow::{format_err, Result};
use derive_builder::Builder;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::busbar::{c_or_default, voltage_correction_factor, BusbarIndex};
use crate::cmplx;
use crate::traits::Sq;

/// Two-winding network transformer.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct NetworkTransformer<N: Default> {
    pub node_hv: N,

    pub node_lv: N,

    /// Rated voltage of the transformer on the high-voltage side (kV).
    pub ur_hv: f64,

    /// Rated voltage of the transformer on the low-voltage side (kV).
    pub ur_lv: f64,

    /// Rated apparent power of the transformer (kVA).
    pub sr: f64,

    /// Total loss of the transformer in the windings at rated current (kW).
    pub pkr: f64,

    /// Short-circuit voltage at rated current in per cent.
    pub ukr: f64,

    /// Rated resistive component of the short-circuit voltage in per cent.
    pub urr: Option<f64>,

    /// Impedance correction factor if the long-term operating conditions of network transformers
    /// before the short circuit are known. Highest operating voltage before short circuit.
    pub ub: Option<f64>,

    /// Highest operating current before short circuit.
    pub ib: f64,

    /// Angle of power factor before short circuit.
    pub phib: f64,

    /// Range of transformer voltage regulation (%).
    pub p: f64,

    pub x0x: f64,

    pub r0r: f64,
}

impl<N: Clone + Default + Eq + core::hash::Hash> NetworkTransformer<N> {
    pub fn new() -> NetworkTransformerBuilder<N> {
        NetworkTransformerBuilder::default()
    }

    pub fn impedance(&self, hv: bool, busbar_index: &BusbarIndex<N>) -> Result<Complex64> {
        let ukr = self.ukr;
        let sr = self.sr * 1000.0;
        let pkr = self.pkr * 1000.0;

        let (ur, un, c) = if hv {
            let ur = self.ur_hv * 1000.0;

            let (mut un, mut c) = if let Some(busbar) = busbar_index.busbar(&self.node_hv) {
                let un = busbar.un * 1000.0;
                let c = c_or_default(busbar);
                (un, c)
            } else {
                (0.0, 0.0)
            };
            if un == 0.0 {
                un = ur;
            }
            if c == 0.0 {
                let ur_hv = self.ur_hv;
                c = voltage_correction_factor(ur_hv, false, true);
            }
            (ur, un, c)
        } else {
            let ur = self.ur_lv * 1000.0;

            let (mut un, mut c) = if let Some(busbar) = busbar_index.busbar(&self.node_lv) {
                let un = busbar.un * 1000.0;
                let c = c_or_default(busbar);
                (un, c)
            } else {
                (0.0, 0.0)
            };
            if un == 0.0 {
                un = ur;
            }
            if c == 0.0 {
                let ur_lv = self.ur_lv;
                c = voltage_correction_factor(ur_lv, false, true);
            }
            (ur, un, c)
        };

        let z = (ukr / 100.0) * (ur.sq() / sr); // (7)

        // (8)
        let r = if let Some(urr) = self.urr {
            (urr / 100.0) * (ur.sq() / sr)
        } else if pkr != 0.0 {
            (pkr * ur.sq()) / sr.sq()
        } else {
            return Err(format_err!("uRr or Pkr must be specified"));
        };

        //let urr = (pkr / sr) * 100.0;
        //let uxr = (ukr*ukr - urr*urr).sqrt();

        let x = (z * z - r * r).sqrt(); // (9)

        let zz = cmplx!(r, x);

        let xr = x / (ur.sq() / sr); // relative reactance of the transformer
        let k = if self.ub.is_some() {
            let ub = self.ub.unwrap() * 1000.0;
            let ib = self.ib * 1000.0;
            let ir = sr / ur; // TODO: test
            let phib = self.phib;

            (un / ub) * (c / (1.0 + xr * (ib / ir) * phib.sin())) // (12b) TODO: test
        } else {
            0.95 * (c / (1.0 + (0.6 * xr))) // (12a)
        };

        let zk = zz * cmplx!(k);

        Ok(zk)
    }
}
