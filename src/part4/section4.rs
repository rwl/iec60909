extern crate self as iec60909;

use anyhow::Result;

use iec60909::ac_system::ACSystem;
use iec60909::busbar::Busbar;
use iec60909::cable::Cable;
use iec60909::feeder::NetworkFeeder;
use iec60909::motor::AsynchronousMotor;
use iec60909::transformer::NetworkTransformer;

/// Returns the a.c. system from Section 4 of IEC60909-4.
///
/// The system consists of a network feeding two 4850m cables at 33kV
/// with transformers and 6kV motors at the end.
pub fn iec60909_4_4() -> Result<ACSystem<&'static str>> {
    let q = Busbar::new()
        .nodes(["Q1", "Q", "Q2"])
        .un(33)
        .cmax(1.1)
        .cmin(1.0)
        .build()?;
    let a = Busbar::new()
        .nodes(["AT1", "M1", "AT2", "M2"])
        .un(6)
        .build()?;

    let tr = q.un / (a.un * 1.05);

    let network = NetworkFeeder::new()
        .node("Q")
        .ur(q.un)
        .ikss(13.12) // kA (750 MVA)
        //.rx(0.1) // breaks test
        .tr(tr)
        .build()?;

    let cable1 = Cable::new()
        .node_i("Q1")
        .node_j("T1")
        .rl(0.1)
        .xl(0.1)
        .l(4.85)
        .tr(tr)
        .build()?;
    let cable2 = Cable::new()
        .node_i("Q2")
        .node_j("T2")
        .rl(0.1)
        .xl(0.1)
        .l(4.85)
        .tr(tr)
        .build()?;

    let t1 = NetworkTransformer::new()
        .node_hv("T1")
        .node_lv("AT1")
        .sr(15.0 * 1e3) // kVA
        .ur_hv(q.un)
        .ur_lv(a.un * 1.05)
        .urr(0.6) // %
        .ukr(15) // %
        .build()?;
    let t2 = NetworkTransformer::new()
        .node_hv("T2")
        .node_lv("AT2")
        .sr(15.0 * 1e3)
        .ur_hv(q.un)
        .ur_lv(a.un * 1.05)
        .urr(0.6) // %
        .ukr(15) // %
        .build()?;

    let m1 = AsynchronousMotor::new()
        .node("M1")
        .ur(6)
        .pr(5.0 * 1e3) // kW
        .cos_phi(0.86)
        .eta(97)
        .ilr_ir(4)
        .p(2)
        .build()?;
    let m2 = AsynchronousMotor::new()
        .node("M2")
        .ur(6)
        .pr(1000) // kW
        .cos_phi(0.83)
        .eta(94)
        .ilr_ir(5.5)
        .p(1)
        .n(3)
        .build()?;

    let ac_system = ACSystem::new()
        .frequency(50)
        .busbars([q, a])
        .feeder(network)
        .cables([cable1, cable2])
        .transformers([t1, t2])
        .motors([m1, m2])
        .build()?;

    Ok(ac_system)
}
