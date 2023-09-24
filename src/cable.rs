use anyhow::Result;
use derive_builder::Builder;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::{cmplx, traits::Sq};

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Cable<N: Default> {
    pub node_i: N,

    pub node_j: N,

    /// Rated voltage of the cable (kV).
    pub ur: f64,

    /// Length (km).
    #[builder(default = "1.0")]
    pub l: f64,

    /// Positive-sequence short-circuit resistance (Ohms/km).
    pub rl: f64,

    /// Positive-sequence short-circuit reactance (Ohms/km).
    pub xl: f64,

    /// Zero-sequence short-circuit resistance (Ohms/km).
    pub r0: f64,

    /// Zero-sequence short-circuit reactance (Ohms/km).
    pub x0: f64,

    /// Number of parallel cables.
    #[builder(setter(into = false), default = "1")]
    pub parallel: usize,

    /// Rated transformation ratio at which the on-load tap-changer is in the main position (>= 1).
    pub tr: Option<f64>,
}

impl<N: Clone + Default> Cable<N> {
    pub fn new() -> CableBuilder<N> {
        CableBuilder::default()
    }

    pub fn impedance(&self) -> Result<Complex64> {
        let n = self.parallel as f64;
        let rl = self.rl;
        let xl = self.xl;
        let l = self.l;

        let mut r = (1.0 / n) * rl * l;
        let mut x = (1.0 / n) * xl * l;
        if let Some(tr) = self.tr {
            r *= 1.0 / tr.sq();
            x *= 1.0 / tr.sq();
        }

        let zl = cmplx!(r, x);

        Ok(zl)
    }
}
