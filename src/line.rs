use std::f64::consts::PI;

use anyhow::Result;
use derive_builder::Builder;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::cmplx;

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct OverheadLine<N: Default> {
    pub node_i: N,

    pub node_j: N,

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

    /// Number of parallel lines.
    pub parallel: Option<i64>,

    /// Cross-section (mm^2).
    pub qn: f64,

    /// Resistivity (Ohm mm^2 / m). Cu: 1/54; Al: 1/34; Al alloy: 1/31"]
    pub rho: f64,

    /// Geometric mean distance between conductors (m). d = cuberoot(d12*d23*d31)"]
    pub d: Option<f64>,

    /// Number of bundled conductors.
    pub n: Option<i64>,
}

impl<N: Clone + Default> OverheadLine<N> {
    pub fn new() -> OverheadLineBuilder<N> {
        OverheadLineBuilder::default()
    }

    pub fn impedance(&self, f: f64) -> Result<Complex64> {
        let (rl, xl) = if let Some(d) = self.d {
            let rho = self.rho;
            let qn = self.qn;
            let n = self.n.unwrap_or(1) as f64;

            let mu0 = 4.0 * PI * 0.0000001; // H/m

            let rl = (rho / qn) * 1000.0; // (14)
            let r = (1.14 * (qn / PI).sqrt()) / 1000.0; // m

            let xl =
                2.0 * PI * f * ((mu0 * 1000.0) / (2.0 * PI)) * ((1.0 / (4.0 * n)) + (d / r).ln());

            (rl, xl)
        } else {
            (self.rl, self.xl)
        };

        let mut zl = cmplx!(rl, xl) * cmplx!(self.l);
        if let Some(p) = self.parallel {
            zl /= cmplx!(p);
        }

        Ok(zl)
    }
}
