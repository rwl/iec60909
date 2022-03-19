use crate::busbar;
use gridjson::*;

// IEC60909_4_6 returns the a.c. system from Section 6 of IEC60909-4.
pub fn iec60909_4_6() -> AcSystem {
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

    let G1 = SynchronousGenerator {
        node: Some("G1".to_string()),
        ur: Some(21.0),
        sr: Some(150e3),
        xdpp: Some(0.14),
        xdsat: Some(1.8),
        cos_phi: Some(0.85),
        r: Some(0.002), // operated only in the overexcited region
        ..Default::default()
    };
    let T1 = NetworkTransformer {
        node_lv: Some("G1".to_string()),
        node_hv: Some("4-T1".to_string()),
        ur_hv: Some(115.0),
        ur_lv: Some(21.0),
        sr: Some(150e3),
        ukr: Some(16.0),
        urr: Some(0.5),
        p: Some(12.0), // YNd5 with on-load tap-changer
        x_0x: Some(0.95),
        r_0r: Some(1.0),
        ..Default::default()
    };

    let G2 = SynchronousGenerator {
        node: Some("G2".to_string()),
        ur: Some(10.5),
        sr: Some(100e3),
        p: Some(7.5),
        xdpp: Some(0.16),
        xdsat: Some(2.0),
        cos_phi: Some(0.9),
        r: Some(0.005),
        ..Default::default()
    };
    let T2 = NetworkTransformer {
        node_lv: Some("G2".to_string()),
        node_hv: Some("3-T2".to_string()),
        ur_hv: Some(120.0),
        ur_lv: Some(10.5),
        sr: Some(100e3),
        ukr: Some(12.0),
        urr: Some(0.5), // YNd5 without tap-changer or off-load taps
        x_0x: Some(1.0),
        r_0r: Some(1.0),
        ..Default::default()
    };

    let G3 = SynchronousGenerator {
        node: Some("6-G3".to_string()),
        ur: Some(10.5),
        sr: Some(10e3),
        p: Some(5.0), // For the calculation a constant value UG=UrG is assumed.
        xdpp: Some(0.1),
        xdsat: Some(1.8),
        cos_phi: Some(0.8),
        r: Some(0.018),
        ..Default::default()
    };

    // Three-winding network transformers YNyn,d5 with on-load tap-changer
    // at the high-voltage side, pT = ±16%. Starpoint earthing:
    // - T3 at the high-voltage side,
    // - T4 at the medium-voltage side.
    let T3 = ThreeWindingTransformer {
        node_hv: Some("1-T3".to_string()),
        node_mv: Some("2-T3".to_string()),
        node_lv: Some("8".to_string()),
        ur_hv: Some(400.0),
        ur_mv: Some(120.0),
        ur_lv: Some(30.0),
        sr_hv_mv: Some(350e3),
        sr_mv_lv: Some(50e3),
        sr_hv_lv: Some(50e3),
        ukr_hv_mv: Some(21.0),
        ukr_hv_lv: Some(10.0),
        ukr_mv_lv: Some(7.0),
        urr_hv_mv: Some(0.26),
        urr_hv_lv: Some(0.16),
        urr_mv_lv: Some(0.16),
        //X0MvXMvHv: 2.1, R0MvRMvLv: 1.0,
        ..Default::default()
    };
    let T4 = ThreeWindingTransformer {
        node_hv: Some("1-T4".to_string()),
        node_mv: Some("2-T4".to_string()),
        node_lv: Some("9".to_string()),
        ur_hv: Some(400.0),
        ur_mv: Some(120.0),
        ur_lv: Some(30.0),
        sr_hv_mv: Some(350e3),
        sr_mv_lv: Some(50e3),
        sr_hv_lv: Some(50e3),
        ukr_hv_mv: Some(21.0),
        ukr_hv_lv: Some(10.0),
        ukr_mv_lv: Some(7.0),
        urr_hv_mv: Some(0.26),
        urr_hv_lv: Some(0.16),
        urr_mv_lv: Some(0.16),
        //X0MvXMvHv: 2.1, R0MvRMvLv: 1.0,
        ..Default::default()
    };
    /*let T4_2 = NetworkTransformer{
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
    let T5 = NetworkTransformer {
        node_hv: Some("5-T5".to_string()),
        node_lv: Some("6-G3".to_string()),
        ur_hv: Some(115.0),
        ur_lv: Some(10.5),
        sr: Some(31.5e3),
        ukr: Some(12.0),
        urr: Some(0.5),
        ..Default::default()
    };
    let T6 = NetworkTransformer {
        node_hv: Some("5-T6".to_string()),
        node_lv: Some("6-L6".to_string()),
        ur_hv: Some(115.0),
        ur_lv: Some(10.5),
        sr: Some(31.5e3),
        ukr: Some(12.0),
        urr: Some(0.5),
        ..Default::default()
    };

    let Q1 = NetworkFeeder {
        // c_max=1.1
        node: Some("1-Q1".to_string()),
        ur: Some(380.0),
        ikss: Some(38.0),
        rx: Some(0.1),
        x_0x: Some(3.0),
        r_0x: Some(0.15),
        tr: Some(T3.ur_hv.unwrap() / T3.ur_mv.unwrap()),
        ..Default::default()
    };
    let Q2 = NetworkFeeder {
        node: Some("5-Q2".to_string()),
        ur: Some(110.0),
        ikss: Some(16.0),
        rx: Some(0.1),
        x_0x: Some(3.3),
        r_0x: Some(0.20),
        ..Default::default()
    };

    let M1 = AsynchronousMotor {
        node: Some("7-M1".to_string()),
        ur: Some(10.0),
        pr: Some(5e3),
        cos_phi: Some(0.88),
        eta: Some(97.5),
        ilr_ir: Some(5.0),
        p: Some(1),
        ..Default::default()
    };
    // Two parallel motors with PrM = 2MW each.
    let M2 = AsynchronousMotor {
        node: Some("7-M2".to_string()),
        ur: Some(10.0),
        pr: Some(2e3),
        cos_phi: Some(0.89),
        eta: Some(96.8),
        ilr_ir: Some(5.2),
        p: Some(2),
        n: Some(2),
        ..Default::default()
    };

    let R1 = Reactor {
        xr: Some(22.0),
        ..Default::default()
    }; // R << X (short-circuit limiting reactor)

    // Arc-suppression coil for the 10 kV network with resonance neutral earthing.
    let R6 = Reactor {
        ukr: Some(10.0),
        ..Default::default()
    };

    let L1 = OverheadLine {
        node_i: Some("2-T3".to_string()),
        node_j: Some("3-L1".to_string()),
        l: Some(20.0),
        rl: Some(0.12),
        xl: Some(0.39),
        r_0: Some(0.32),
        x_0: Some(1.26),
        ..Default::default()
    };
    let L2 = OverheadLine {
        node_i: Some("3-L1".to_string()),
        node_j: Some("4-L2".to_string()),
        l: Some(10.0),
        rl: Some(0.12),
        xl: Some(0.39),
        r_0: Some(0.32),
        x_0: Some(1.26),
        ..Default::default()
    };
    let L3 = OverheadLine {
        node_i: Some("2-L3".to_string()),
        node_j: Some("5-L3".to_string()),
        l: Some(5.0),
        rl: Some(0.12),
        xl: Some(0.39),
        r_0: Some(0.52),
        x_0: Some(1.86),
        parallel: Some(2), // double line
        ..Default::default()
    };
    let L4 = OverheadLine {
        node_i: Some("5-L4".to_string()),
        node_j: Some("3-L4".to_string()),
        l: Some(10.0),
        rl: Some(0.096),
        xl: Some(0.388),
        r_0: Some(0.22),
        x_0: Some(1.10),
        ..Default::default()
    };
    let L5 = OverheadLine {
        node_i: Some("5-L5".to_string()),
        node_j: Some("4-L5".to_string()),
        l: Some(15.0),
        rl: Some(0.12),
        xl: Some(0.386),
        r_0: Some(0.22),
        x_0: Some(1.10),
        ..Default::default()
    };

    let L6 = Cable {
        node_i: Some("6-L6".to_string()),
        node_j: Some("7-L6".to_string()),
        ur: Some(10.0),
        l: Some(1.0),
        rl: Some(0.082),
        xl: Some(0.086),
        ..Default::default()
    };

    AcSystem {
        frequency: Some(50.0),
        busbars: Some(vec![b1, b2, b8, b5, b6, b7, b3, b4]),
        feeders: Some(vec![Q1, Q2]),
        power_stations: Some(vec![
            PowerStationUnit {
                generator: Some(G1),
                transformer: Some(T1),
            },
            PowerStationUnit {
                generator: Some(G2),
                transformer: Some(T2),
            },
        ]),
        generators: Some(vec![G3]),
        three_winding_transformers: Some(vec![T3, T4]),
        transformers: Some(vec![T5, T6]),
        motors: Some(vec![M1, M2]),
        reactors: Some(vec![R1, R6]),
        lines: Some(vec![L1, L2, L3, L4, L5]),
        cables: Some(vec![L6]),
        ..Default::default()
    }
}
