use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Short-circuit limiting reactor.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Reactor<N: Default> {
    pub node: N,

    /// Rated short circuit voltage.
    pub ukr: f64,

    pub irr: f64,

    /// Rated resistance (Ohms).
    pub rr: f64,

    /// Rated reactance (Ohms).
    pub xr: f64,
}

impl<N: Clone + Default> Reactor<N> {
    pub fn new() -> ReactorBuilder<N> {
        ReactorBuilder::default()
    }
}
