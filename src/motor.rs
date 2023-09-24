use anyhow::Result;
use derive_builder::Builder;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::{cmplx, traits::Sq};

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct AsynchronousMotor<N: Default> {
    pub node: N,

    /// Rated voltage of the motor (kV).
    pub ur: f64,

    /// Rated power (kW).
    pub pr: f64,

    /// Rated power factor.
    #[builder(default = "1.0")]
    pub cos_phi: f64,

    /// Motor efficiency (%).
    #[builder(default = "100.0")]
    pub eta: f64,

    /// Ratio of the locked-rotor current to the rated current of the motor.
    pub ilr_ir: f64,

    /// Pairs of poles.
    #[builder(setter(into = false), default = "1")]
    pub p: usize,

    /// R/X ratio of short-circuit impedance.
    pub rx: Option<f64>,

    /// Number of motors in the group.
    #[builder(setter(into = false), default = "1")]
    pub n: usize,
}

impl<N: Clone + Default> AsynchronousMotor<N> {
    pub fn new() -> AsynchronousMotorBuilder<N> {
        AsynchronousMotorBuilder::default()
    }

    pub fn impedance(&self) -> Result<Complex64> {
        let ur = self.ur * 1e3;
        let pr = self.pr * 1e3;
        let cos_phi = self.cos_phi;
        let eta = self.eta / 100.0;
        let ilr_ir = self.ilr_ir;
        let p = self.p as f64;
        let n = self.n as f64;

        let rx = match self.rx {
            Some(rx) => rx,
            None => {
                if ur < 1e3 {
                    // LV
                    0.42
                } else {
                    // MV
                    if pr / p < 1e6 {
                        // 1MW per pair of poles
                        0.15
                    } else {
                        0.1
                    }
                }
            }
        };

        let sr_m = pr / (eta * cos_phi); // Rated apparent power.

        let zm = (1.0 / n) * (1.0 / ilr_ir) * (ur.sq() / sr_m); // (26)

        let xm = zm / (1.0 + rx.sq()).sqrt(); // (27)
        let rm = xm * rx;

        Ok(cmplx!(rm, xm))
    }
}
