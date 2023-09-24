extern crate self as iec60909;

use anyhow::Result;

use iec60909::*;

// IEC60909_4_6 returns the a.c. system from Section 6 of IEC60909-4.
pub fn iec60909_4_6() -> Result<ACSystem<&'static str>> {
    let b1 = busbar!(380, "1-T3", "1-Q1", "1-T4"); // CMax: 1.1
    let b2 = busbar!(110, "2-T3", "2-T4", "2-L3");
    let b3 = busbar!(110, "3-T2", "3-L1", "3-L4");
    let b4 = busbar!(110, "4-T1", "4-L2", "4-L5");
    let b5 = busbar!(110, "5-L4", "5-L3", "5-L5", "5-T5", "5-Q2", "5-T6");
    let b6 = busbar!(10, "6-G3", "6-L6");
    let b7 = busbar!(10, "7-M1", "7-L6", "7-M2");
    let b8 = busbar!(30, "8");
    //busbar!(30);
    //busbar!(21);
    //busbar!(10);

    let g1 = SynchronousGenerator::new()
        .node("G1")
        .ur(21)
        .sr(150_000)
        .xdpp(0.14)
        .xdsat(1.8)
        .cos_phi(0.85)
        .r(0.002) // operated only in the overexcited region
        .build()?;
    let t1 = NetworkTransformer::new()
        .node_lv("G1")
        .node_hv("4-T1")
        .ur_hv(115)
        .ur_lv(21)
        .sr(150_000)
        .ukr(16)
        .urr(0.5)
        .p(12) // YNd5 with on-load tap-changer
        .x0x(0.95)
        .r0r(1.0)
        .build()?;

    let g2 = SynchronousGenerator::new()
        .node("G2")
        .ur(10.5)
        .sr(100_000)
        .p(7.5)
        .xdpp(0.16)
        .xdsat(2.0)
        .cos_phi(0.9)
        .r(0.005)
        .build()?;
    let t2 = NetworkTransformer::new()
        .node_lv("G2")
        .node_hv("3-T2")
        .ur_hv(120.0)
        .ur_lv(10.5)
        .sr(100e3)
        .ukr(12.0)
        .urr(0.5) // YNd5 without tap-changer or off-load taps
        .x0x(1.0)
        .r0r(1.0)
        .build()?;

    let g3 = SynchronousGenerator::new()
        .node("6-G3")
        .ur(10.5)
        .sr(10_000)
        .p(5.0) // For the calculation a constant value UG=UrG is assumed.
        .xdpp(0.1)
        .xdsat(1.8)
        .cos_phi(0.8)
        .r(0.018)
        .build()?;

    // Three-winding network transformers YNyn,d5 with on-load tap-changer
    // at the high-voltage side, pT = ±16%. Starpoint earthing:
    // - T3 at the high-voltage side,
    // - T4 at the medium-voltage side.
    let t3 = ThreeWindingTransformer::new()
        .node_hv("1-T3")
        .node_mv("2-T3")
        .node_lv("8")
        .ur_hv(400)
        .ur_mv(120)
        .ur_lv(30)
        .sr_hv_mv(350_000)
        .sr_mv_lv(50_000)
        .sr_hv_lv(50_000)
        .ukr_hv_mv(21)
        .ukr_hv_lv(10)
        .ukr_mv_lv(7)
        .urr_hv_mv(0.26)
        .urr_hv_lv(0.16)
        .urr_mv_lv(0.16)
        //X0MvXMvHv: 2.1, R0MvRMvLv: 1.0,
        .build()?;
    let t4 = ThreeWindingTransformer::new()
        .node_hv("1-T4")
        .node_mv("2-T4")
        .node_lv("9")
        .ur_hv(400)
        .ur_mv(120)
        .ur_lv(30)
        .sr_hv_mv(350_000)
        .sr_mv_lv(50_000)
        .sr_hv_lv(50_000)
        .ukr_hv_mv(21)
        .ukr_hv_lv(10)
        .ukr_mv_lv(7)
        .urr_hv_mv(0.26)
        .urr_hv_lv(0.16)
        .urr_mv_lv(0.16)
        //X0MvXMvHv: 2.1, R0MvRMvLv: 1.0,
        .build()?;
    /*let t4_2 = NetworkTransformer{
        node_hv: "1-T4",
        node_lv: "2-T4",
        ur_hv:   400,
        ur_lv:   120,
        sr:     350e3,
        ukr:    21,
        urr:    0.26,
    }*/

    // Three-winding network transformer YNyn,d5, treated here as a two-winding
    // transformer, i.e. ukr = ukr_HvMv
    let t5 = NetworkTransformer::new()
        .node_hv("5-T5")
        .node_lv("6-G3")
        .ur_hv(115)
        .ur_lv(10.5)
        .sr(31_500)
        .ukr(12.0)
        .urr(0.5)
        .build()?;
    let t6 = NetworkTransformer::new()
        .node_hv("5-T6")
        .node_lv("6-L6")
        .ur_hv(115)
        .ur_lv(10.5)
        .sr(31_500)
        .ukr(12.0)
        .urr(0.5)
        .build()?;

    let q1 = NetworkFeeder::new()
        //.c_max(1.1)
        .node("1-Q1")
        .ur(380)
        .ikss(38)
        .rx(0.1)
        .x0x(3.0)
        .r0x(0.15)
        .tr(t3.ur_hv / t3.ur_mv)
        .build()?;
    let q2 = NetworkFeeder::new()
        .node("5-Q2")
        .ur(110)
        .ikss(16)
        .rx(0.1)
        .x0x(3.3)
        .r0x(0.20)
        .build()?;

    let m1 = AsynchronousMotor::new()
        .node("7-M1")
        .ur(10)
        .pr(5e3)
        .cos_phi(0.88)
        .eta(97.5)
        .ilr_ir(5.0)
        .p(1)
        .build()?;
    // Two parallel motors with PrM = 2MW each.
    let m2 = AsynchronousMotor::new()
        .node("7-M2")
        .ur(10)
        .pr(2_000)
        .cos_phi(0.89)
        .eta(96.8)
        .ilr_ir(5.2)
        .p(2)
        .n(2)
        .build()?;

    let r1 = Reactor::new().xr(22.0).build()?; // R << X (short-circuit limiting reactor)

    // Arc-suppression coil for the 10 kV network with resonance neutral earthing.
    let r6 = Reactor::new().ukr(10.0).build()?;

    let l1 = OverheadLine::new()
        .node_i("2-T3")
        .node_j("3-L1")
        .l(20)
        .rl(0.12)
        .xl(0.39)
        .r0(0.32)
        .x0(1.26)
        .build()?;
    let l2 = OverheadLine::new()
        .node_i("3-L1")
        .node_j("4-L2")
        .l(10)
        .rl(0.12)
        .xl(0.39)
        .r0(0.32)
        .x0(1.26)
        .build()?;
    let l3 = OverheadLine::new()
        .node_i("2-L3")
        .node_j("5-L3")
        .l(5)
        .rl(0.12)
        .xl(0.39)
        .r0(0.52)
        .x0(1.86)
        .parallel(2) // double line
        .build()?;
    let l4 = OverheadLine::new()
        .node_i("5-L4")
        .node_j("3-L4")
        .l(10)
        .rl(0.096)
        .xl(0.388)
        .r0(0.22)
        .x0(1.10)
        .build()?;
    let l5 = OverheadLine::new()
        .node_i("5-L5")
        .node_j("4-L5")
        .l(15)
        .rl(0.12)
        .xl(0.386)
        .r0(0.22)
        .x0(1.10)
        .build()?;

    let l6 = Cable::new()
        .node_i("6-L6")
        .node_j("7-L6")
        .ur(10)
        .l(1)
        .rl(0.082)
        .xl(0.086)
        .build()?;

    let ac_system = ACSystem::new()
        .frequency(50)
        .busbars([b1, b2, b8, b5, b6, b7, b3, b4])
        .feeders([q1, q2])
        .power_station(PowerStationUnit {
            generator: g1,
            transformer: t1,
        })
        .power_station(PowerStationUnit {
            generator: g2,
            transformer: t2,
        })
        .generator(g3)
        .three_winding_transformers([t3, t4])
        .transformers([t5, t6])
        .motors([m1, m2])
        .reactors([r1, r6])
        .lines([l1, l2, l3, l4, l5])
        .cable(l6)
        .build()?;

    Ok(ac_system)
}
