extern crate self as iec60909;

use anyhow::Result;

use iec60909::ac_system::ACSystem;
use iec60909::busbar::Busbar;
use iec60909::feeder::NetworkFeeder;
use iec60909::generator::SynchronousGenerator;
use iec60909::motor::AsynchronousMotor;
use iec60909::transformer::NetworkTransformer;
use iec60909::transformer3::ThreeWindingTransformer;

use crate::station::PowerStationUnit;

/// IEC60909_4_5 returns the a.c. system from Section 5 of IEC60909-4.
pub fn iec60909_4_5() -> Result<ACSystem<&'static str>> {
    let q = Busbar::new().node("Q").un(220).cmax(1.1).build()?;
    let b = Busbar::new()
        .nodes([
            "M7", "M6", "M5", "M4", "M3", "M2", "M1", "T20", "T15", "T16", "T17", "T18", "T19", "B",
        ])
        .un(10)
        .build()?;
    let c = Busbar::new()
        .nodes([
            "C", "T21", "T22", "T23", "T24", "T25", "T26", "M8", "M9", "M10", "M11", "M12", "M13",
            "M14",
        ])
        .un(10)
        .build()?;

    let network = NetworkFeeder::new()
        .node("Q")
        .ur(q.un)
        .ikss(21)
        .rx(0.12) // IkssQmax = 52.5 kA
        .build()?;

    let t = NetworkTransformer::new()
        .node_hv("Q")
        .node_lv("A")
        .sr(250.0 * 1e3) // 250MVA
        .ur_hv(240) //250,
        .ur_lv(21)
        //.p(12) // %
        .ukr(15) // %
        .pkr(520) // 520kW
        .build()?;

    let g = SynchronousGenerator::new()
        .node("A")
        .sr(250.0 * 1e3) // 250MVA
        .ur(21)
        .p(5) // 5%
        .r(0.0025) // Ohms
        .xdpp(17.0 / 100.0) // 17%
        .xdsat(200.0 / 100.0)
        .cos_phi(0.78)
        .build()?;

    let at = ThreeWindingTransformer::new()
        .node_hv("A")
        .node_mv("B")
        .node_lv("C")
        .sr_hv_mv(25.0 * 1e3) // 25MVA
        .sr_hv_lv(25.0 * 1e3) // 25MVA
        .sr_mv_lv(25.0 * 1e3) // 25MVA
        .ur_hv(21)
        .ur_mv(10.5)
        .ur_lv(10.5)
        .ukr_hv_mv(7)
        .ukr_hv_lv(7)
        .ukr_mv_lv(13)
        .pkr_hv_mv(59)
        .pkr_hv_lv(59)
        .pkr_mv_lv(114)
        .build()?;
    let (m1, m2, m3, m4, m5, m6, m7) = {
        let m1 = AsynchronousMotor::new()
            .node("M1")
            .pr(6.8)
            .n(2)
            .ur(10)
            .cos_phi(0.89)
            .eta(97.6)
            .ilr_ir(4)
            .p(2)
            .build()?;
        let m2 = AsynchronousMotor::new()
            .node("M2")
            .pr(3.1)
            .n(1)
            .ur(10)
            .cos_phi(0.85)
            .eta(95.9)
            .ilr_ir(4)
            .p(2)
            .build()?;
        let m3 = AsynchronousMotor::new()
            .node("M3")
            .pr(1.5)
            .n(2)
            .ur(10)
            .cos_phi(0.88)
            .eta(96.2)
            .ilr_ir(4)
            .p(1)
            .build()?;
        let m4 = AsynchronousMotor::new()
            .node("M4")
            .pr(0.7)
            .n(1)
            .ur(10)
            .cos_phi(0.85)
            .eta(95.2)
            .ilr_ir(4)
            .p(3)
            .build()?;
        let m5 = AsynchronousMotor::new()
            .node("M5")
            .pr(0.53)
            .n(2)
            .ur(10)
            .cos_phi(0.75)
            .eta(94.8)
            .ilr_ir(4)
            .p(5)
            .build()?;
        let m6 = AsynchronousMotor::new()
            .node("M6")
            .pr(2)
            .n(1)
            .ur(10)
            .cos_phi(0.85)
            .eta(96)
            .ilr_ir(4)
            .p(3)
            .build()?;
        let m7 = AsynchronousMotor::new()
            .node("M7")
            .pr(1.71)
            .n(2)
            .ur(10)
            .cos_phi(0.85)
            .eta(96)
            .ilr_ir(4)
            .p(3)
            .build()?;
        (m1, m2, m3, m4, m5, m6, m7)
    };
    let (m8, m9, m10, m11, m12, m13, m14) = {
        let m8 = AsynchronousMotor::new()
            .node("M8")
            .pr(5.1)
            .n(1)
            .ur(10)
            .cos_phi(0.87)
            .eta(97.3)
            .ilr_ir(4)
            .p(3)
            .build()?;
        let m9 = AsynchronousMotor::new()
            .node("M9")
            .pr(3.1)
            .n(1)
            .ur(10)
            .cos_phi(0.85)
            .eta(95.9)
            .ilr_ir(4)
            .p(2)
            .build()?;
        let m10 = AsynchronousMotor::new()
            .node("M10")
            .pr(1.5)
            .n(2)
            .ur(10.0)
            .cos_phi(0.88)
            .eta(96.2)
            .ilr_ir(4)
            .p(1)
            .build()?;
        let m11 = AsynchronousMotor::new()
            .node("M11")
            .pr(1.85)
            .n(1)
            .ur(10)
            .cos_phi(0.85)
            .eta(95.9)
            .ilr_ir(4)
            .p(3)
            .build()?;
        let m12 = AsynchronousMotor::new()
            .node("M12")
            .pr(0.7)
            .n(2)
            .ur(10)
            .cos_phi(0.85)
            .eta(95.2)
            .ilr_ir(4)
            .p(3)
            .build()?;
        let m13 = AsynchronousMotor::new()
            .node("M13")
            .pr(0.53)
            .n(2)
            .ur(10)
            .cos_phi(0.75)
            .eta(94.8)
            .ilr_ir(4)
            .p(5)
            .build()?;
        let m14 = AsynchronousMotor::new()
            .node("M14")
            .pr(2)
            .n(1)
            .ur(10)
            .cos_phi(0.85)
            .eta(96)
            .ilr_ir(4)
            .p(3)
            .build()?;
        (m8, m9, m10, m11, m12, m13, m14)
    };
    fn t15_19<N: Clone + Default + Eq + core::hash::Hash>(hv: N, lv: N) -> NetworkTransformer<N> {
        NetworkTransformer::new()
            .node_hv(hv)
            .node_lv(lv)
            .sr(2.5 * 1e3)
            .ur_hv(10)
            .ur_lv(0.73)
            .ukr(6)
            .pkr(23.5)
            .build()
            .unwrap()
    }
    fn m15_19<N: Clone + Default>(t: N) -> AsynchronousMotor<N> {
        AsynchronousMotor::new()
            .node(t)
            .pr(900) // 0.9MW
            .ur(0.69)
            .cos_phi(0.72) // cosPhi*eta
            .ilr_ir(5)
            .rx(0.42)
            .build()
            .unwrap()
    }
    fn t20<N: Clone + Default + Eq + core::hash::Hash>(hv: N, lv: N) -> NetworkTransformer<N> {
        NetworkTransformer::new()
            .node_hv(hv)
            .node_lv(lv)
            .sr(1.6 * 1e3)
            .ur_hv(10)
            .ur_lv(0.42) // 0.73
            .ukr(6)
            .pkr(16.5)
            .build()
            .unwrap()
    }
    fn m20<N: Clone + Default>(t: N) -> AsynchronousMotor<N> {
        AsynchronousMotor::new()
            .node(t)
            .pr(1_000) // 1MW
            .ur(0.40)
            .cos_phi(0.72)
            .ilr_ir(5)
            .rx(0.42)
            .build()
            .unwrap()
    }

    let ac_system = ACSystem::new()
        .frequency(50)
        .busbars([q, b, c])
        .feeders([network])
        .power_station(PowerStationUnit {
            generator: g,
            transformer: t,
        })
        .transformers([
            t15_19("T15", "M15"),
            t15_19("T16", "M16"),
            t15_19("T17", "M17"),
            t15_19("T18", "M18"),
            t15_19("T19", "M19"),
            t20("T20", "M20"),
            t15_19("T21", "M21"),
            t15_19("T22", "M22"),
            t15_19("T23", "M23"),
            t15_19("T24", "M24"),
            t15_19("T25", "M25"),
            t20("T26", "M26"),
        ])
        .three_winding_transformer(at)
        .motors([
            m1,
            m2,
            m3,
            m4,
            m5,
            m6,
            m7,
            m8,
            m9,
            m10,
            m11,
            m12,
            m13,
            m14,
            m15_19("M15"),
            m15_19("M16"),
            m15_19("M17"),
            m15_19("M18"),
            m15_19("M19"),
            m20("M20"),
            m15_19("M21"),
            m15_19("M22"),
            m15_19("M23"),
            m15_19("M24"),
            m15_19("M25"),
            m20("M26"),
        ])
        .build()?;

    Ok(ac_system)
}
