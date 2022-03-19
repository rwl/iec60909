use gridjson::*;

/// IEC60909_4_5 returns the a.c. system from Section 5 of IEC60909-4.
pub fn iec60909_4_5() -> AcSystem {
    let q = Busbar {
        node: Some("Q".to_string()),
        un: Some(220.0),
        cmax: Some(1.1),
        ..Default::default()
    };
    let b = Busbar {
        nodes: Some(vec![
            "M7".to_string(),
            "M6".to_string(),
            "M5".to_string(),
            "M4".to_string(),
            "M3".to_string(),
            "M2".to_string(),
            "M1".to_string(),
            "T20".to_string(),
            "T15".to_string(),
            "T16".to_string(),
            "T17".to_string(),
            "T18".to_string(),
            "T19".to_string(),
            "B".to_string(),
        ]),
        un: Some(10.0),
        ..Default::default()
    };
    let c = Busbar {
        nodes: Some(vec![
            "C".to_string(),
            "T21".to_string(),
            "T22".to_string(),
            "T23".to_string(),
            "T24".to_string(),
            "T25".to_string(),
            "T26".to_string(),
            "M8".to_string(),
            "M9".to_string(),
            "M10".to_string(),
            "M11".to_string(),
            "M12".to_string(),
            "M13".to_string(),
            "M14".to_string(),
        ]),
        un: Some(10.0),
        ..Default::default()
    };

    let network = NetworkFeeder {
        node: Some("Q".to_string()),
        ur: q.un,
        ikss: Some(21.0),
        rx: Some(0.12), // IkssQmax = 52.5 kA
        ..Default::default()
    };

    let t = NetworkTransformer {
        node_hv: Some("Q".to_string()),
        node_lv: Some("A".to_string()),
        sr: Some(250.0 * 1e3), // 250MVA
        ur_hv: Some(240.0),    //250,
        ur_lv: Some(21.0),
        //p:          12, // %
        ukr: Some(15.0),  // %
        pkr: Some(520.0), // 520kW
        ..Default::default()
    };

    let g = SynchronousGenerator {
        node: Some("A".to_string()),
        sr: Some(250.0 * 1e3), // 250MVA
        ur: Some(21.0),
        p: Some(5.0),             // 5%
        r: Some(0.0025),          // Ohms
        xdpp: Some(17.0 / 100.0), // 17%
        xdsat: Some(200.0 / 100.0),
        cos_phi: Some(0.78),
    };

    let at = ThreeWindingTransformer {
        node_hv: Some("A".to_string()),
        node_mv: Some("B".to_string()),
        node_lv: Some("C".to_string()),
        sr_hv_mv: Some(25.0 * 1e3), // 25MVA
        sr_hv_lv: Some(25.0 * 1e3), // 25MVA
        sr_mv_lv: Some(25.0 * 1e3), // 25MVA
        ur_hv: Some(21.0),
        ur_mv: Some(10.5),
        ur_lv: Some(10.5),
        ukr_hv_mv: Some(7.0),
        ukr_hv_lv: Some(7.0),
        ukr_mv_lv: Some(13.0),
        pkr_hv_mv: Some(59.0),
        pkr_hv_lv: Some(59.0),
        pkr_mv_lv: Some(114.0),
        ..Default::default()
    };
    let (m1, m2, m3, m4, m5, m6, m7) = {
        let m1 = AsynchronousMotor {
            node: Some("M1".to_string()),
            pr: Some(6.8),
            n: Some(2),
            ur: Some(10.0),
            cos_phi: Some(0.89),
            eta: Some(97.6),
            ilr_ir: Some(4.0),
            p: Some(2),
            ..Default::default()
        };
        let m2 = AsynchronousMotor {
            node: Some("M2".to_string()),
            pr: Some(3.1),
            n: Some(1),
            ur: Some(10.0),
            cos_phi: Some(0.85),
            eta: Some(95.9),
            ilr_ir: Some(4.0),
            p: Some(2),
            ..Default::default()
        };
        let m3 = AsynchronousMotor {
            node: Some("M3".to_string()),
            pr: Some(1.5),
            n: Some(2),
            ur: Some(10.0),
            cos_phi: Some(0.88),
            eta: Some(96.2),
            ilr_ir: Some(4.0),
            p: Some(1),
            ..Default::default()
        };
        let m4 = AsynchronousMotor {
            node: Some("M4".to_string()),
            pr: Some(0.7),
            n: Some(1),
            ur: Some(10.0),
            cos_phi: Some(0.85),
            eta: Some(95.2),
            ilr_ir: Some(4.0),
            p: Some(3),
            ..Default::default()
        };
        let m5 = AsynchronousMotor {
            node: Some("M5".to_string()),
            pr: Some(0.53),
            n: Some(2),
            ur: Some(10.0),
            cos_phi: Some(0.75),
            eta: Some(94.8),
            ilr_ir: Some(4.0),
            p: Some(5),
            ..Default::default()
        };
        let m6 = AsynchronousMotor {
            node: Some("M6".to_string()),
            pr: Some(2.0),
            n: Some(1),
            ur: Some(10.0),
            cos_phi: Some(0.85),
            eta: Some(96.0),
            ilr_ir: Some(4.0),
            p: Some(3),
            ..Default::default()
        };
        let m7 = AsynchronousMotor {
            node: Some("M7".to_string()),
            pr: Some(1.71),
            n: Some(2),
            ur: Some(10.0),
            cos_phi: Some(0.85),
            eta: Some(96.0),
            ilr_ir: Some(4.0),
            p: Some(3),
            ..Default::default()
        };
        (m1, m2, m3, m4, m5, m6, m7)
    };
    let (m8, m9, m10, m11, m12, m13, m14) = {
        let m8 = AsynchronousMotor {
            node: Some("M8".to_string()),
            pr: Some(5.1),
            n: Some(1),
            ur: Some(10.0),
            cos_phi: Some(0.87),
            eta: Some(97.3),
            ilr_ir: Some(4.0),
            p: Some(3),
            ..Default::default()
        };
        let m9 = AsynchronousMotor {
            node: Some("M9".to_string()),
            pr: Some(3.1),
            n: Some(1),
            ur: Some(10.0),
            cos_phi: Some(0.85),
            eta: Some(95.9),
            ilr_ir: Some(4.0),
            p: Some(2),
            ..Default::default()
        };
        let m10 = AsynchronousMotor {
            node: Some("M10".to_string()),
            pr: Some(1.5),
            n: Some(2),
            ur: Some(10.0),
            cos_phi: Some(0.88),
            eta: Some(96.2),
            ilr_ir: Some(4.0),
            p: Some(1),
            ..Default::default()
        };
        let m11 = AsynchronousMotor {
            node: Some("M11".to_string()),
            pr: Some(1.85),
            n: Some(1),
            ur: Some(10.0),
            cos_phi: Some(0.85),
            eta: Some(95.9),
            ilr_ir: Some(4.0),
            p: Some(3),
            ..Default::default()
        };
        let m12 = AsynchronousMotor {
            node: Some("M12".to_string()),
            pr: Some(0.7),
            n: Some(2),
            ur: Some(10.0),
            cos_phi: Some(0.85),
            eta: Some(95.2),
            ilr_ir: Some(4.0),
            p: Some(3),
            ..Default::default()
        };
        let m13 = AsynchronousMotor {
            node: Some("M13".to_string()),
            pr: Some(0.53),
            n: Some(2),
            ur: Some(10.0),
            cos_phi: Some(0.75),
            eta: Some(94.8),
            ilr_ir: Some(4.0),
            p: Some(5),
            ..Default::default()
        };
        let m14 = AsynchronousMotor {
            node: Some("M14".to_string()),
            pr: Some(2.0),
            n: Some(1),
            ur: Some(10.0),
            cos_phi: Some(0.85),
            eta: Some(96.0),
            ilr_ir: Some(4.0),
            p: Some(3),
            ..Default::default()
        };
        (m8, m9, m10, m11, m12, m13, m14)
    };
    let t15_19 = |hv: &str, lv: &str| -> NetworkTransformer {
        NetworkTransformer {
            node_hv: Some(hv.to_string()),
            node_lv: Some(lv.to_string()),
            sr: Some(2.5 * 1e3),
            ur_hv: Some(10.0),
            ur_lv: Some(0.73),
            ukr: Some(6.0),
            pkr: Some(23.5),
            ..Default::default()
        }
    };
    let m15_19 = |t: &str| -> AsynchronousMotor {
        AsynchronousMotor {
            node: Some(t.to_string()),
            pr: Some(900.0), // 0.9MW
            ur: Some(0.69),
            cos_phi: Some(0.72), // cosPhi*eta
            ilr_ir: Some(5.0),
            rx: Some(0.42),
            ..Default::default()
        }
    };
    let t20 = |hv: &str, lv: &str| -> NetworkTransformer {
        NetworkTransformer {
            node_hv: Some(hv.to_string()),
            node_lv: Some(lv.to_string()),
            sr: Some(1.6 * 1e3),
            ur_hv: Some(10.0),
            ur_lv: Some(0.42), // 0.73
            ukr: Some(6.0),
            pkr: Some(16.5),
            ..Default::default()
        }
    };
    let m20 = |t: &str| -> AsynchronousMotor {
        AsynchronousMotor {
            node: Some(t.to_string()),
            pr: Some(1_000.0), // 1MW
            ur: Some(0.40),
            cos_phi: Some(0.72),
            ilr_ir: Some(5.0),
            rx: Some(0.42),
            ..Default::default()
        }
    };

    AcSystem {
        frequency: Some(50.0),
        busbars: Some(vec![q, b, c]),
        feeders: Some(vec![network]),
        power_stations: Some(vec![PowerStationUnit {
            generator: Some(g),
            transformer: Some(t),
        }]),
        transformers: Some(vec![
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
        ]),
        three_winding_transformers: Some(vec![at]),
        motors: Some(vec![
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
        ]),
        ..Default::default()
    }
}
