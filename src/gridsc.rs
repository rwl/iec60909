// Copyright 2018-2022 Richard Lincoln. All rights reserved.

use crate::busbar::{voltage_correction_factor, BusbarIndex};
use crate::traits::Sq;
use gridjson::*;
use num_complex::{Complex, Complex64};
use std::f64::consts::PI;

const SQRT_3: f64 = 1.732050807568877293527446341505872366942805253810380628055;

pub fn feeder_impedance(
    feeder: &NetworkFeeder,
    ohl: bool,
    busbar_index: &BusbarIndex,
) -> Result<Complex64, String> {
    let busbar = busbar_index.busbar(feeder.node.as_ref().unwrap()).unwrap();

    let c = busbar.cmax.unwrap_or_default();
    let un = busbar.un.unwrap();
    let ikss = feeder.ikss.unwrap();
    let rx = feeder.rx.unwrap();
    let tr = feeder.tr.unwrap();

    let mut z = (c * un) / (SQRT_3 * ikss);
    if tr != 0.0 {
        z *= 1.0 / tr.sq();
    }

    let x = if rx != 0.0 {
        z / (1.0 + rx.sq()).sqrt()
    } else {
        z
    };

    let z = if un > 35.0 && ohl {
        Complex64::new(0.0, x)
    } else if rx != 0.0 {
        let r = rx * x;
        Complex64::new(r, x)
    } else {
        let x = 0.995 * x;
        let r = 0.1 * x;
        Complex64::new(r, x)
    };

    Ok(z)
}

pub fn transformer_impedance(
    tr: &NetworkTransformer,
    hv: bool,
    busbar_index: &BusbarIndex,
) -> Result<Complex64, String> {
    let ukr = tr.ukr.unwrap();
    let sr = tr.sr.unwrap() * 1000.0;
    let pkr = tr.pkr.unwrap() * 1000.0;
    let urr = tr.urr.unwrap();

    let (u_r, u_n, c) = if hv {
        let ur = tr.ur_hv.unwrap() * 1000.0;

        let (mut un, mut c) =
            if let Some(busbar) = busbar_index.busbar(tr.node_hv.as_ref().unwrap()) {
                let un = busbar.un.unwrap() * 1000.0;
                let c = busbar.cmax.unwrap();
                (un, c)
            } else {
                (0.0, 0.0)
            };
        if un == 0.0 {
            un = ur;
        }
        if c == 0.0 {
            let ur_hv = tr.ur_hv.unwrap();
            c = voltage_correction_factor(ur_hv, false, true);
        }
        (ur, un, c)
    } else {
        let ur = tr.ur_lv.unwrap() * 1000.0;

        let (mut un, mut c) =
            if let Some(busbar) = busbar_index.busbar(tr.node_lv.as_ref().unwrap()) {
                // if busbar, ok := busbarIndex[tr.NodeLv]; ok {
                let un = busbar.un.unwrap() * 1000.0;
                let c = busbar.cmax.unwrap();
                (un, c)
            } else {
                (0.0, 0.0)
            };
        if un == 0.0 {
            un = ur;
        }
        if c == 0.0 {
            let ur_lv = tr.ur_lv.unwrap();
            c = voltage_correction_factor(ur_lv, false, true);
        }
        (ur, un, c)
    };

    let z = (ukr / 100.0) * (u_r.sq() / sr); // (7)

    // (8)
    let r = if urr != 0.0 {
        (urr / 100.0) * (u_r.sq() / sr)
    } else if pkr != 0.0 {
        (pkr * u_r.sq()) / sr.sq()
    } else {
        return Err("uRr or Pkr must be specified".to_string());
    };

    //let urr = (Pkr / Sr) * 100.0;
    //let uxr = (ukr*ukr - urr*urr).sqrt();

    let x = (z * z - r * r).sqrt(); // (9)

    let zz = Complex64::new(r, x);

    let xr = x / (u_r.sq() / sr); // relative reactance of the transformer
    let k = if tr.ub.unwrap() != 0.0 {
        // var (
        let ub = tr.ub.unwrap() * 1000.0;
        let ib = tr.ib.unwrap() * 1000.0;
        let ir = sr / u_r; // TODO: test
        let phib = tr.phib.unwrap();
        // )

        (u_n / ub) * (c / (1.0 + xr * (ib / ir) * phib.sin())) // (12b) TODO: test
    } else {
        0.95 * (c / (1.0 + (0.6 * xr))) // (12a)
    };

    let zk = zz * Complex64::new(k, 0.0);

    Ok(zk)
}

pub fn power_station_impedance(
    psu: &PowerStationUnit,
    hv: bool,
    oltc: bool,
    peak: bool,
    busbar_index: &BusbarIndex,
) -> Result<Complex64, String> {
    let t = psu.transformer.as_ref().unwrap();
    let g = psu.generator.as_ref().unwrap();

    let busbar = busbar_index.busbar(t.node_hv.as_ref().unwrap()).unwrap();

    let mut un = busbar.un.unwrap() * 1e3;
    let mut c = busbar.cmax.unwrap();
    if un == 0.0 {
        un = t.ur_hv.unwrap() * 1e3;
    }
    if c == 0.0 {
        c = voltage_correction_factor(t.ur_lv.unwrap(), false, true);
    }
    // Generator impedance.
    let mut ur_g = g.ur.unwrap() * 1e3;
    let sr_g = g.sr.unwrap() * 1e3;
    let xdpp_pu = g.xdpp.unwrap(); // pu
    let rg = g.r.unwrap();
    let phi = g.cos_phi.unwrap().acos();
    let pg = g.p.unwrap() / 100.0;

    if t.ur_lv.unwrap() > ur_g {
        // step-up
        ur_g = ur_g * (1.0 + pg); // For three phase short-circuit currents.
    }
    let zr_g = ur_g.sq() / sr_g;
    let xdpp = xdpp_pu * zr_g; // Ohm

    // Subtransient impedance of the generator (without correction factor Kg).
    let mut zg = Complex64::new(rg, xdpp);

    if peak {
        let rgf: f64 = match ur_g {
            _ if ur_g > 1e3 && sr_g >= 100e6 => 0.05 * xdpp,
            _ if ur_g > 1e3 && sr_g < 100e6 => 0.07 * xdpp,
            _ => 0.15 * xdpp,
        };
        zg = Complex64::new(rgf, xdpp);
    }

    // Unit transformer impedance.

    let ulv = t.ur_lv.unwrap() * 1e3;
    let uhv = t.ur_hv.unwrap() * 1e3;
    let ukr = t.ukr.unwrap();
    let sr_t = t.sr.unwrap() * 1e3;
    let pkr = t.pkr.unwrap() * 1e3;
    let urr = t.urr.unwrap();

    // let UrT: float64
    let tr2 = Complex64::new((t.ur_hv.unwrap() / t.ur_lv.unwrap()).sq(), 0.0);
    let ur_t = if hv {
        t.ur_hv.unwrap() * 1000.0
    } else {
        t.ur_lv.unwrap() * 1000.0
    };

    let z = (ukr / 100.0) * (ur_t.sq() / sr_t); // Zt (7)

    // (8)
    let rt = if urr != 0.0 {
        (urr / 100.0) * (ur_t.sq() / (sr_t))
    } else if pkr != 0.0 {
        (pkr * ur_t.sq()) / sr_t.sq()
    } else {
        return Err("uRr or Pkr must be specified".to_string());
    };

    //let urr = (pkr / sr) * 100.0;
    //let uxr = (ukr*ukr - urr*urr).sqrt();

    let xt = (z * z - rt * rt).sqrt(); // (9)

    // Impedance of the unit transformer related to the high-voltage
    // side (without correction factor KT).
    let zt_hv = Complex64::new(rt, xt);

    // Power station correction factor (S3.7).
    let ks = if oltc {
        let xt = xt / (ur_t.sq() / sr_t); // Relative reactance of the transformer.

        (un.sq() / ur_g.sq())
            * (ulv.sq() / uhv.sq())
            * (c / (1.0 + (xdpp_pu - xt).abs() * phi.sin()))
    // (22)
    } else {
        ur_g = g.ur.unwrap() * 1e3 * (1.0 + pg); // Always use pg no OLTC.
        (un / ur_g) * (ulv / uhv) * (c / (1.0 + xdpp_pu * phi.sin())) // (24)

        /*if permanentTap {
            let pt = tr.P / 100.0;
            if highest {
                K *= 1 - pt;
            } else {
                K *= 1 + pt;
            }
        }*/
    };

    let zs = Complex64::new(ks, 0.0) * (tr2 * zg + zt_hv);

    Ok(zs)
}

pub fn overhead_line_limpedance(f: f64, line: &OverheadLine) -> Result<Complex64, String> {
    let d = line.d.unwrap();
    let rho = line.rho.unwrap();
    let qn = line.qn.unwrap();
    let mut n = line.n.unwrap() as f64;
    let l = Complex64::new(line.l.unwrap(), 0.0);
    let p = Complex64::new(line.parallel.unwrap() as f64, 0.0);

    let (rl, xl) = if d != 0.0 {
        let mu0 = 4.0 * PI * 0.0000001; // H/m
        if n <= 0.0 {
            n = 1.0;
        }

        let rl = (rho / qn) * 1000.0; // (14)
        let r = (1.14 * (qn / PI).sqrt()) / 1000.0; // m

        let xl = 2.0 * PI * f * ((mu0 * 1000.0) / (2.0 * PI)) * ((1.0 / (4.0 * n)) + (d / r).ln());

        (rl, xl)
    } else {
        (line.rl.unwrap(), line.xl.unwrap())
    };

    let mut zl = Complex64::new(rl, xl);
    if line.l.unwrap() != 0.0 {
        zl *= l;
    }
    if line.parallel.unwrap() != 0 {
        zl /= p;
    }

    Ok(zl)
}

pub fn cable_impedance(cable: &Cable) -> Result<Complex64, String> {
    let mut n = cable.parallel.unwrap() as f64;
    let rl = cable.rl.unwrap();
    let xl = cable.xl.unwrap();
    let mut l = cable.l.unwrap();
    let tr = cable.tr.unwrap();

    if n <= 0.0 {
        n = 1.0;
    }
    if l <= 0.0 {
        l = 1.0;
    }

    let mut r = (1.0 / n) * rl * l;
    let mut x = (1.0 / n) * xl * l;
    if tr != 0.0 {
        r *= 1.0 / tr.sq();
        x *= 1.0 / tr.sq();
    }

    let zl = Complex64::new(r, x);

    Ok(zl)
}

pub fn motor_impedance(motor: &AsynchronousMotor) -> Result<Complex64, String> {
    let ur = motor.ur.unwrap() * 1e3;
    let pr = motor.pr.unwrap() * 1e3;
    let mut cos_phi = motor.cos_phi.unwrap();
    let mut eta = motor.eta.unwrap() / 100.0;
    let ilr_ir = motor.ilr_ir.unwrap();
    let mut p = motor.p.unwrap() as f64;
    let mut rx = motor.rx.unwrap();
    let mut n = motor.n.unwrap() as f64;

    if eta <= 0.0 {
        eta = 1.0;
    }
    if cos_phi == 0.0 {
        cos_phi = 1.0;
    }
    if p <= 0.0 {
        p = 1.0;
    }
    if rx == 0.0 {
        if ur < 1e3 {
            // LV
            rx = 0.42;
        } else {
            // MV
            if pr / p < 1e6 {
                // 1MW per pair of poles
                rx = 0.15;
            } else {
                rx = 0.1;
            }
        }
    }
    if n <= 0.0 {
        n = 1.0;
    }

    let sr_m = pr / (eta * cos_phi); // Rated apparent power.

    let zm = (1.0 / n) * (1.0 / ilr_ir) * (ur.sq() / sr_m); // (26)

    let xm = zm / (1.0 + rx.sq()).sqrt(); // (27)
    let rm = xm * rx;

    Ok(Complex::new(rm, xm))
}

pub fn generator_impedance(
    sym: &SynchronousGenerator,
    tol: f64,
    peak: bool,
    busbar_index: &BusbarIndex,
) -> Result<Complex64, String> {
    let busbar = busbar_index.busbar(sym.node.as_ref().unwrap()).unwrap();

    let mut un = busbar.un.unwrap() * 1e3; // Nominal voltage of the system.
    let mut c = busbar.cmax.unwrap(); // Voltage correction factor.

    if un == 0.0 {
        un = sym.ur.unwrap() * 1e3;
    }
    if c == 0.0 {
        c = voltage_correction_factor(sym.ur.unwrap(), false, true);
    }

    let mut ur_g = sym.ur.unwrap() * 1e3; // Rated voltage of the generator.
    let sr_g = sym.sr.unwrap() * 1e3;
    let xdpp_pu = sym.xdpp.unwrap(); // pu
    let rg = sym.r.unwrap();
    let phi = sym.cos_phi.unwrap().acos(); // Phase angle between IrG and UrG/sqrt(3).
    let pg = sym.p.unwrap() / 100.0;

    if (un - ur_g).abs() > tol * 1e3 {
        ur_g = ur_g * (1.0 + pg); // For three phase short-circuit currents.
    }
    let zr_g = ur_g.sq() / sr_g;
    let xdpp = xdpp_pu * zr_g; // Ohms

    let kg = (un / ur_g) * (c / (1.0 + xdpp_pu * phi.sin())); // Correction factor (18).

    let zg = Complex64::new(rg, xdpp); // Subtransient impedance of the generator in the positive-sequence system.

    let mut zg_k = zg * Complex64::new(kg, 0.0); // Corrected subtransient impedance of the generator (17).

    if peak {
        let rgf = match ur_g {
            _ if ur_g > 1e3 && sr_g >= 100e6 => 0.05 * xdpp,
            _ if ur_g > 1e3 && sr_g < 100e6 => 0.07 * xdpp,
            _ => 0.15 * xdpp,
        };
        zg_k = Complex64::new(rgf, xdpp);
    }

    Ok(zg_k)
}

#[derive(Clone, Copy)]
pub enum TransformerSide {
    HV,
    MV,
    LV,
}

pub enum TransformerSides {
    HvMv,
    HvLv,
    MvLv,
}

pub fn three_winding_transformer_side_impedance(
    tr: &ThreeWindingTransformer,
    side: TransformerSide,
    sides: TransformerSides,
    busbar_index: &BusbarIndex,
) -> Result<Complex64, String> {
    let ur = match side {
        TransformerSide::HV => tr.ur_hv.unwrap() * 1e3,
        TransformerSide::MV => tr.ur_mv.unwrap() * 1e3,
        TransformerSide::LV => tr.ur_lv.unwrap() * 1e3,
    };
    let (sr, ukr, mut urr, pkr) = match sides {
        TransformerSides::HvMv => (
            tr.sr_hv_mv.unwrap() * 1e3,
            tr.ukr_hv_mv.unwrap(),
            tr.urr_hv_mv.unwrap(),
            tr.pkr_hv_mv.unwrap() * 1e3,
        ),
        TransformerSides::HvLv => (
            tr.sr_hv_lv.unwrap() * 1e3,
            tr.ukr_hv_lv.unwrap(),
            tr.urr_hv_lv.unwrap(),
            tr.pkr_hv_lv.unwrap() * 1e3,
        ),
        TransformerSides::MvLv => (
            tr.sr_mv_lv.unwrap() * 1e3,
            tr.ukr_mv_lv.unwrap(),
            tr.urr_mv_lv.unwrap(),
            tr.pkr_mv_lv.unwrap() * 1e3,
        ),
    };
    if urr == 0.0 {
        urr = (pkr / sr) * 100.0;
    }
    if urr > ukr {
        return Err(format!("uRr ({}) must be < ukr ({})", urr, ukr).to_string());
    }
    let uxr = (ukr.sq() - urr.sq()).sqrt(); // (10d)

    let z = Complex64::new(urr / 100.0, uxr / 100.0) * Complex64::new(ur.sq() / sr, 0.0); // (10)

    let mut cmax = voltage_correction_factor(tr.ur_hv.unwrap(), false, true);
    if let Some(busbar) = busbar_index.busbar(&tr.node_hv.as_ref().unwrap()) {
        if busbar.cmax.unwrap() != 0.0 {
            cmax = busbar.cmax.unwrap();
        }
    }

    let xr = z.im / (ur.sq() / sr); // Relative reactance of the transformer.

    let k = 0.95 * (cmax / (1.0 + (0.6 * xr))); // (12a)

    let zk = Complex64::new(k, 0.0) * z;

    Ok(zk)
}

pub fn three_winding_transformer_impedance(
    tr: &ThreeWindingTransformer,
    side: TransformerSide,
    busbar_index: &BusbarIndex,
) -> Result<(Complex64, Complex64, Complex64), String> {
    let zk_hv_mv =
        three_winding_transformer_side_impedance(tr, side, TransformerSides::HvMv, busbar_index)?;

    let zk_hv_lv =
        three_winding_transformer_side_impedance(tr, side, TransformerSides::HvLv, busbar_index)?;

    let zk_mv_lv =
        three_winding_transformer_side_impedance(tr, side, TransformerSides::MvLv, busbar_index)?;

    let z_hv = 0.5 * (zk_hv_mv + zk_hv_lv - zk_mv_lv);
    let z_mv = 0.5 * (zk_mv_lv + zk_hv_mv - zk_hv_lv);
    let z_lv = 0.5 * (zk_hv_lv + zk_mv_lv - zk_hv_mv);

    Ok((z_hv, z_mv, z_lv))
}
