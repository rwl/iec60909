use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Fault<N: Default> {
    pub node: N,

    /// Peak short-circuit current (50Hz method) (kA).
    pub ip50: f64,

    /// Peak short-circuit current (20Hz method) (kA).
    pub ip20: f64,

    /// Symmetrical short-circuit breaking current (r.m.s.)"]
    pub ib: f64,

    /// Steady-state short-circuit current (r.m.s.)"]
    pub ik: f64,

    /// Thermal equivalent short-circuit current (kA).
    pub ith: f64,
}
