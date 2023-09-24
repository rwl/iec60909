use anyhow::Result;
use derive_builder::Builder;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::busbar::{c_or_default, BusbarIndex};
use crate::cmplx;
use crate::math::SQRT_3;
use crate::traits::Sq;

/// An external grid connection.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct NetworkFeeder<N: Default> {
    pub node: N,

    /// Rated voltage of the feeder (kV).
    pub ur: f64,

    /// Initial symmetrical short-circuit current (kA).
    pub ikss: f64,

    /// R/X ratio of short-circuit impedance.
    pub rx: Option<f64>,

    /// Rated transformation ratio at which the on-load tap-changer is in the main position (>= 1).
    pub tr: Option<f64>,

    pub r0x: f64,

    pub x0x: f64,
}

impl<N: Clone + Default + Eq + core::hash::Hash> NetworkFeeder<N> {
    pub fn new() -> NetworkFeederBuilder<N> {
        NetworkFeederBuilder::default()
    }

    pub fn impedance(&self, ohl: bool, busbar_index: &BusbarIndex<N>) -> Result<Complex64> {
        let busbar = busbar_index.busbar(&self.node).unwrap();

        let c = c_or_default(busbar);
        let un = busbar.un;
        let ikss = self.ikss;

        let mut z = (c * un) / (SQRT_3 * ikss);
        if let Some(tr) = self.tr {
            z *= 1.0 / tr.sq();
        }

        let x = if let Some(rx) = self.rx {
            z / (1.0 + rx.sq()).sqrt()
        } else {
            z
        };

        let z = if un > 35.0 && ohl {
            cmplx!(0, x)
        } else if let Some(rx) = self.rx {
            let r = rx * x;
            cmplx!(r, x)
        } else {
            let x = 0.995 * x;
            let r = 0.1 * x;
            cmplx!(r, x)
        };

        Ok(z)
    }
}
