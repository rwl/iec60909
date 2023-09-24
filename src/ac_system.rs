use std::collections::HashMap;

use anyhow::{format_err, Result};
use derive_builder::Builder;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use sparsetools::{csc::CSC, dok::DoK};
use spsolve::FactorSolver;

use crate::busbar::{Busbar, BusbarIndex};
use crate::cable::Cable;
use crate::fault::Fault;
use crate::feeder::NetworkFeeder;
use crate::generator::SynchronousGenerator;
use crate::line::OverheadLine;
use crate::motor::AsynchronousMotor;
use crate::reactor::Reactor;
use crate::station::PowerStationUnit;
use crate::transformer::NetworkTransformer;
use crate::transformer3::ThreeWindingTransformer;

const ONE: Complex64 = Complex64 { re: 1.0, im: 0.0 };

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Builder)]
#[builder(default, setter(into))]
pub struct ACSystem<N: Clone + Default> {
    /// System nominal frequency.
    pub frequency: f64,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "busbar")))]
    pub busbars: Vec<Busbar<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "feeder")))]
    pub feeders: Vec<NetworkFeeder<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "power_station")))]
    pub power_stations: Vec<PowerStationUnit<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "generator")))]
    pub generators: Vec<SynchronousGenerator<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "transformer")))]
    pub transformers: Vec<NetworkTransformer<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "motor")))]
    pub motors: Vec<AsynchronousMotor<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "reactor")))]
    pub reactors: Vec<Reactor<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "line")))]
    pub lines: Vec<OverheadLine<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "cable")))]
    pub cables: Vec<Cable<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "three_winding_transformer")))]
    pub three_winding_transformers: Vec<ThreeWindingTransformer<N>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each(name = "fault")))]
    pub faults: Vec<Fault<N>>,
}

impl<N: Clone + Default + Eq + core::hash::Hash> ACSystem<N> {
    pub fn new() -> ACSystemBuilder<N> {
        ACSystemBuilder::default()
    }

    pub fn fault_impedance<F>(
        &self,
        solver: impl FactorSolver<usize, Complex64, F>,
    ) -> Result<HashMap<N, Complex64>>
    where
        N: Clone + Default + Eq + core::hash::Hash,
    {
        let (ix, nn) = self.nodes();
        let y_mat: CSC<usize, Complex64> = self.admittance_matrix(&ix, nn)?;

        // let Z = zmat.Identity(nn);

        let factors =
            solver.factor(y_mat.cols(), y_mat.rowidx(), y_mat.colptr(), y_mat.values())?;

        let mut z = vec![Complex64::default(); nn];
        let mut z_diag = vec![Complex64::default(); nn];
        for i in 0..nn {
            z[i] = ONE;
            solver.solve(&factors, &mut z, false)?;
            z_diag[i] = z[i];
            z[i] = Complex64::default();
        }

        let mut zk = HashMap::new();
        for (t, i) in &ix {
            zk.insert(t.clone(), z_diag[*i]);
        }
        Ok(zk)
    }

    fn admittance_matrix(
        &self,
        ix: &HashMap<N, usize>,
        nn: usize,
    ) -> Result<CSC<usize, Complex64>> {
        let ib = BusbarIndex::new(&self.busbars);

        let mut y_mat = DoK::new(nn, nn);

        for (i, f) in self.feeders.iter().enumerate() {
            let z = match f.impedance(false, &ib) {
                Ok(z) => z,
                Err(err) => {
                    return Err(format_err!("feeder {} error: {}", i + 1, err));
                }
            };
            if z == Complex64::default() {
                return Err(format_err!("feeder {} error: zero impedance", i + 1));
            }
            let j: usize = ix[&f.node];
            y_mat.add(j, j, ONE / z)?;
        }

        for (i, t) in self.transformers.iter().enumerate() {
            let z = match t.impedance(false, &ib) {
                Ok(z) => z,
                Err(err) => {
                    return Err(format_err!("transformer {} error: {}", i + 1, err));
                }
            };
            if z == Complex64::default() {
                return Err(format_err!("transformer {} error: zero impedance", i + 1));
            }
            let j = ix[&t.node_hv];
            let k = ix[&t.node_lv];
            let y = ONE / z;

            y_mat.add(j, j, y)?;
            y_mat.sub(j, k, y)?;
            y_mat.sub(k, j, y)?;
            y_mat.add(k, k, y)?;
        }

        for (i, c) in self.cables.iter().enumerate() {
            let z = match c.impedance() {
                Ok(z) => z,
                Err(err) => {
                    return Err(format_err!("cable {} error: {}", i + 1, err));
                }
            };
            if z == Complex64::default() {
                return Err(format_err!("cable {} error: zero impedance", i + 1));
            }
            let j = ix[&c.node_i];
            let k = ix[&c.node_j];
            let y = ONE / z;

            y_mat.add(j, j, y)?;
            y_mat.sub(j, k, y)?;
            y_mat.sub(k, j, y)?;
            y_mat.add(k, k, y)?;
        }

        for (i, l) in self.lines.iter().enumerate() {
            let z = match l.impedance(self.frequency) {
                Ok(z) => z,
                Err(err) => {
                    return Err(format_err!("line {} error: {}", i + 1, err));
                }
            };
            if z == Complex64::default() {
                return Err(format_err!("line {}: zero impedance", i + 1));
            }
            let j = ix[&l.node_i];
            let k = ix[&l.node_j];
            let y = ONE / z;

            y_mat.add(j, j, y)?;
            y_mat.sub(j, k, y)?;
            y_mat.sub(k, j, y)?;
            y_mat.add(k, k, y)?;
        }

        Ok(y_mat.to_csc())
    }

    fn nodes(&self) -> (HashMap<N, usize>, usize) {
        let mut nodes = HashMap::new();
        let mut n = 0;

        for (i, b) in self.busbars.iter().enumerate() {
            // if b.node != "" {
            //     nodes[b.Node] = i
            // }
            for t in &b.nodes {
                nodes.insert(t.clone(), i);
            }
            n += 1;
        }

        let mut add = |t: &N| {
            if !nodes.contains_key(t) {
                nodes.insert(t.clone(), n);
                n += 1;
            }
        };

        for f in &self.feeders {
            add(&f.node);
        }
        for s in &self.power_stations {
            // if s.Generator != nil {
            add(&s.generator.node);
            // }
            // if s.Transformer != nil {
            add(&s.transformer.node_hv);
            add(&s.transformer.node_lv);
            // }
        }
        for g in &self.generators {
            add(&g.node);
        }
        for t in &self.transformers {
            add(&t.node_hv);
            add(&t.node_lv);
        }
        for t in &self.three_winding_transformers {
            add(&t.node_hv);
            add(&t.node_mv);
            add(&t.node_lv);
        }
        for m in &self.motors {
            add(&m.node);
        }
        for l in &self.lines {
            add(&l.node_i);
            add(&l.node_j);
        }
        for c in &self.cables {
            add(&c.node_i);
            add(&c.node_j);
        }
        (nodes, n)
    }
}
