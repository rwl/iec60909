use anyhow::{format_err, Result};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use crate::busbar::{c_or_default, voltage_correction_factor, BusbarIndex};
use crate::generator::SynchronousGenerator;
use crate::traits::Sq;
use crate::transformer::NetworkTransformer;

use crate::cmplx;

/// Generator and unit transformer with on-load tap-changer.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct PowerStationUnit<N: Default> {
    pub generator: SynchronousGenerator<N>,

    pub transformer: NetworkTransformer<N>,
}

impl<N: Clone + Default + Eq + core::hash::Hash> PowerStationUnit<N> {
    pub fn impedance(
        &self,
        hv: bool,
        oltc: bool,
        peak: bool,
        busbar_index: &BusbarIndex<N>,
    ) -> Result<Complex64> {
        let t = &self.transformer;
        let g = &self.generator;

        let busbar = busbar_index.busbar(&t.node_hv).unwrap();

        let mut un = busbar.un * 1e3;
        let mut c = c_or_default(busbar);
        if un == 0.0 {
            un = t.ur_hv * 1e3;
        }
        if c == 0.0 {
            c = voltage_correction_factor(t.ur_lv, false, true);
        }
        // Generator impedance.
        let mut ur_g = g.ur * 1e3;
        let sr_g = g.sr * 1e3;
        let xdpp_pu = g.xdpp; // pu
        let rg = g.r;
        let phi = g.cos_phi.acos();
        let pg = g.p / 100.0;

        if t.ur_lv > ur_g {
            // step-up
            ur_g = ur_g * (1.0 + pg); // For three phase short-circuit currents.
        }
        let zr_g = ur_g.sq() / sr_g;
        let xdpp = xdpp_pu * zr_g; // Ohm

        // Subtransient impedance of the generator (without correction factor Kg).
        let mut zg = cmplx!(rg, xdpp);

        if peak {
            let rgf: f64 = match ur_g {
                _ if ur_g > 1e3 && sr_g >= 100e6 => 0.05 * xdpp,
                _ if ur_g > 1e3 && sr_g < 100e6 => 0.07 * xdpp,
                _ => 0.15 * xdpp,
            };
            zg = cmplx!(rgf, xdpp);
        }

        // Unit transformer impedance.

        let ulv = t.ur_lv * 1e3;
        let uhv = t.ur_hv * 1e3;
        let ukr = t.ukr;
        let sr_t = t.sr * 1e3;
        let pkr = t.pkr * 1e3;

        let tr2 = cmplx!((t.ur_hv / t.ur_lv).sq());
        let ur_t = if hv {
            t.ur_hv * 1000.0
        } else {
            t.ur_lv * 1000.0
        };

        let z = (ukr / 100.0) * (ur_t.sq() / sr_t); // Zt (7)

        // (8)
        let rt = if let Some(urr) = t.urr {
            (urr / 100.0) * (ur_t.sq() / (sr_t))
        } else if pkr != 0.0 {
            (pkr * ur_t.sq()) / sr_t.sq()
        } else {
            return Err(format_err!("uRr or Pkr must be specified"));
        };

        //let urr = (pkr / sr) * 100.0;
        //let uxr = (ukr*ukr - urr*urr).sqrt();

        let xt = (z * z - rt * rt).sqrt(); // (9)

        // Impedance of the unit transformer related to the high-voltage
        // side (without correction factor KT).
        let zt_hv = cmplx!(rt, xt);

        // Power station correction factor (S3.7).
        let ks = if oltc {
            let xt = xt / (ur_t.sq() / sr_t); // Relative reactance of the transformer.

            (un.sq() / ur_g.sq())
                * (ulv.sq() / uhv.sq())
                * (c / (1.0 + (xdpp_pu - xt).abs() * phi.sin())) // (22)
        } else {
            ur_g = g.ur * 1e3 * (1.0 + pg); // Always use pg no OLTC.
            (un / ur_g) * (ulv / uhv) * (c / (1.0 + xdpp_pu * phi.sin())) // (24)

            /*if permanent_tap {
                let pt = tr.p / 100.0;
                if highest {
                    k *= 1 - pt;
                } else {
                    k *= 1 + pt;
                }
            }*/
        };

        let zs = cmplx!(ks) * (tr2 * zg + zt_hv);

        Ok(zs)
    }
}
