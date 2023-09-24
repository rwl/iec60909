extern crate self as iec60909;

use anyhow::Result;

use iec60909::ac_system::ACSystem;
use iec60909::busbar::Busbar;
use iec60909::cable::Cable;
use iec60909::feeder::NetworkFeeder;
use iec60909::line::OverheadLine;
use iec60909::transformer::NetworkTransformer;

/// Returns the a.c. system from Section 3 of IEC60909-4.
pub fn iec60909_4_3() -> Result<ACSystem<&'static str>> {
    let q = Busbar::new()
        .node("Q1")
        .node("Q")
        .node("Q2")
        .un(20) // kV
        .cmax(1.1)
        .build()?;
    let a = Busbar::new()
        .node("L1")
        .node("L3")
        .node("L2")
        .un(0.400) // 400V
        .cmax(1.05)
        .build()?;

    let t1 = NetworkTransformer::new()
        .node_hv("Q1")
        .node_lv("T1")
        .sr(630) // kVA
        .ur_hv(20) // kV
        .ur_lv(0.410)
        .ukr(4) // %
        .pkr(6.5) // kW
        //.r0r(1)
        //.x0x(0.95)
        .build()?;
    let t2 = NetworkTransformer::new()
        .node_hv("Q2")
        .node_lv("T2")
        .sr(400) // kVA
        .ur_hv(20) // kV
        .ur_lv(0.410)
        .ukr(4) // %
        .pkr(4.6) // kW
        //.r0r(1)
        //.x0x(0.95)
        .build()?;

    let network = NetworkFeeder::new()
        .node("Q")
        .ur(q.un)
        .ikss(10) // kA
        .tr(20.0 / t1.ur_lv)
        .build()?;

    // Two parallel four-core cables (4 x 240 mm^2 Cu).
    let l1 = {
        let rl = 0.077;
        let xl = 0.079;
        Cable::new()
            .node_i("T1")
            .node_j("L1")
            .l(10.0 / 1000.0) // km
            .rl(rl) // Ohms/km
            .xl(xl) // Ohms/km
            .r0(3.7 * rl)
            .x0(1.81 * xl)
            .parallel(2)
            .build()?
    };

    // Two parallel three-core cables (3 x 185 mm^2 Al).
    let l2 = {
        let rl = 0.208;
        let xl = 0.068;
        Cable::new()
            .node_i("T2")
            .node_j("L2")
            .l(4.0 / 1000.0)
            .rl(rl)
            .xl(xl)
            .r0(4.23 * rl)
            .x0(1.21 * xl)
            .parallel(2)
            .build()?
    };

    // Four-core cable (4 x 70 mm^2 Cu).
    let l3 = {
        let rl = 0.271;
        let xl = 0.087;
        Cable::new()
            .node_i("L3")
            .node_j("L4")
            .l(20.0 / 1000.0)
            .rl(rl)
            .xl(xl)
            .r0(3.0 * rl)
            .x0(4.46 * xl)
            .build()?
    };

    // Overhead line (qn = 50 mm^2 Cu, d = 0.4m).
    let l4 = {
        let rl = 0.3704;
        let xl = 0.297;
        OverheadLine::new()
            .node_i("L4")
            .node_j("F3")
            .l(50.0 / 1000.0)
            .rl(rl)
            .xl(xl)
            .r0(2.0 * rl)
            .x0(3.0 * xl)
            .qn(50)
            .rho(1.0 / 54.0)
            .d(0.4)
            .build()?
    };

    let ac_system = ACSystem::new()
        .frequency(50)
        .busbars([q, a])
        .feeder(network)
        .transformers([t1, t2])
        .cables([l1, l2, l3])
        .line(l4)
        .build()?;

    Ok(ac_system)
}
