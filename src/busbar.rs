use gridjson::Busbar;
use std::collections::HashMap;

pub struct BusbarIndex<'a> {
    index: HashMap<String, &'a Busbar>,
    // index: HashMap<String, usize>,
    // busbars: Vec<&'a Busbar>,
}

impl<'a> BusbarIndex<'a> {
    pub(crate) fn new(busbars: &'a [Busbar]) -> Self {
        let mut ix = BusbarIndex {
            index: HashMap::new(),
            // busbars: vec![],
        };
        for busbar in busbars {
            if busbar.node.as_ref().unwrap() != "" {
                ix.index
                    .insert(busbar.node.as_ref().unwrap().to_string(), busbar);
            }
            for node in busbar.nodes.as_ref().unwrap() {
                ix.index.insert(node.to_string(), busbar);
            }
        }
        ix
    }

    pub(crate) fn empty() -> Self {
        BusbarIndex {
            index: HashMap::new(),
        }
    }

    pub(crate) fn busbar(&self, node: &str) -> Option<&'a Busbar> {
        // Some(self.busbars[*self.index.get(node).unwrap()])
        Some(*self.index.get(node).unwrap())
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

#[macro_export]
macro_rules! busbar {
    ($un:expr, $( $args:expr ),*) => {
        {
            let un = f64::from($un);
            let mut nodes: Vec<String> = vec![];
            $(
                nodes.push(String::from($args));
            )*
            Busbar{
                un: Some(un),
                node: None,
                nodes: Some(nodes),
                cmax: Some(crate::busbar::voltage_correction_factor(un, false, true)),
		        cmin: Some(crate::busbar::voltage_correction_factor(un, true, true))
            }
        }
    }
}
