use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into))]
pub struct Busbar<N: Clone + Default> {
    #[builder(setter(each(name = "node")))]
    pub nodes: Vec<N>,

    /// Nominal system voltage of the busbar (kV).
    pub un: f64,

    /// Voltage correction factor for maximum short-circuit current. Typical value is 1.1 or 1.05
    /// for nominal voltages below 1kV.
    #[builder(setter(strip_option))]
    pub cmax: Option<f64>,

    /// Voltage correction factor for minimum short-circuit current. Typical value is 1 or 0.95 for
    /// nominal voltages below 1kV.
    #[builder(setter(strip_option))]
    pub cmin: Option<f64>,
}

impl<N: Clone + Default> Busbar<N> {
    pub fn new() -> BusbarBuilder<N> {
        BusbarBuilder::default()
    }
}

pub struct BusbarIndex<'a, N: Clone + Default + Eq + core::hash::Hash> {
    index: HashMap<N, &'a Busbar<N>>,
    // index: HashMap<String, usize>,
    // busbars: Vec<&'a Busbar>,
}

impl<'a, N: Clone + Default + Eq + core::hash::Hash> BusbarIndex<'a, N> {
    pub fn new(busbars: &'a [Busbar<N>]) -> Self {
        let mut ix = BusbarIndex {
            index: HashMap::new(),
            // busbars: vec![],
        };
        for busbar in busbars {
            for node in &busbar.nodes {
                ix.index.insert(node.clone(), busbar);
            }
        }
        ix
    }

    pub fn empty() -> Self {
        BusbarIndex {
            index: HashMap::new(),
        }
    }

    pub fn busbar(&self, node: &N) -> Option<&'a Busbar<N>> {
        match self.index.get(node) {
            Some(b) => Some(b),
            None => None,
        }
    }
}

pub(crate) fn voltage_correction_factor(un: f64, min: bool, six_percent: bool) -> f64 {
    if min {
        match un {
            _ if un <= 1.0 => {
                // Low voltage
                0.95
            }
            _ if un > 1.0 && un <= 35.0 => {
                // Medium voltage
                1.0
            }
            _ => {
                // High voltage
                1.0
            }
        }
    } else {
        match un {
            _ if un <= 1.0 => {
                // Low voltage
                if six_percent {
                    1.05
                } else {
                    1.10
                }
            }
            _ if un > 1.0 && un <= 35.0 => {
                // Medium voltage
                1.10
            }
            _ => {
                // High voltage
                1.10
            }
        }
    }
}

pub(crate) fn c_or_default<N: Clone + Default>(busbar: &Busbar<N>) -> f64 {
    match busbar.cmax {
        Some(cmax) => cmax,
        None => voltage_correction_factor(busbar.un, false, true),
    }
}

#[macro_export]
macro_rules! busbar {
    ($un:expr, $( $args:expr ),*) => {
        {
            let un = f64::from($un);
            let mut nodes = vec![];
            $(
                nodes.push($args);
            )*
            crate::busbar::Busbar{
                un,
                // node: None,
                nodes,
                cmax: None,
                cmin: None,
            }
        }
    }
}
