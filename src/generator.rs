use anyhow::Result;
use derive_builder::Builder;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::busbar::{c_or_default, voltage_correction_factor, BusbarIndex};
use crate::cmplx;
use crate::traits::Sq;

/// A generator without a unit transformer.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct SynchronousGenerator<N: Default> {
    pub node: N,

    /// Rated voltage of the generator (kV).
    pub ur: f64,

    /// Rated apparent power (kVA).
    pub sr: f64,

    /// Rated power factor.
    #[builder(default = "1.0")]
    pub cos_phi: f64,

    /// Resistance of the synchronous machine (Ohms).
    pub r: f64,

    /// Relative subtransient reactance of the generator related to the rated impedance (p.u.).
    pub xdpp: f64,

    /// Saturated synchronous reactance (p.u.).
    pub xdsat: f64,
    /// Range of generator voltage regulation (%).
    #[builder(default = "0.0")]
    pub p: f64,
}

impl<N: Clone + Default + Eq + core::hash::Hash> SynchronousGenerator<N> {
    pub fn new() -> SynchronousGeneratorBuilder<N> {
        SynchronousGeneratorBuilder::default()
    }

    pub fn impedance(
        &self,
        tol: f64,
        peak: bool,
        busbar_index: &BusbarIndex<N>,
    ) -> Result<Complex64> {
        let busbar = busbar_index.busbar(&self.node).unwrap();

        let mut un = busbar.un * 1e3; // Nominal voltage of the system.
        let mut c = c_or_default(busbar); // Voltage correction factor.

        if un == 0.0 {
            un = self.ur * 1e3;
        }
        if c == 0.0 {
            c = voltage_correction_factor(self.ur, false, true);
        }

        let mut ur_g = self.ur * 1e3; // Rated voltage of the generator.
        let sr_g = self.sr * 1e3;
        let xdpp_pu = self.xdpp; // pu
        let rg = self.r;
        let phi = self.cos_phi.acos(); // Phase angle between IrG and UrG/sqrt(3).
        let pg = self.p / 100.0;

        if (un - ur_g).abs() > tol * 1e3 {
            ur_g = ur_g * (1.0 + pg); // For three phase short-circuit currents.
        }
        let zr_g = ur_g.sq() / sr_g;
        let xdpp = xdpp_pu * zr_g; // Ohms

        let kg = (un / ur_g) * (c / (1.0 + xdpp_pu * phi.sin())); // Correction factor (18).

        let zg = cmplx!(rg, xdpp); // Subtransient impedance of the generator in the positive-sequence system.

        let mut zg_k = zg * cmplx!(kg); // Corrected subtransient impedance of the generator (17).

        if peak {
            let rgf = match ur_g {
                _ if ur_g > 1e3 && sr_g >= 100e6 => 0.05 * xdpp,
                _ if ur_g > 1e3 && sr_g < 100e6 => 0.07 * xdpp,
                _ => 0.15 * xdpp,
            };
            zg_k = cmplx!(rgf, xdpp);
        }

        Ok(zg_k)
    }
}
