use gridjson::*;

/// Returns the a.c. system from Section 4 of IEC60909-4.
///
/// The system consists of a network feeding two 4850m cables at 33kV
/// with transformers and 6kV motors at the end.
pub fn iec60909_4_4() -> AcSystem {
    let q = Busbar {
        nodes: Some(vec!["Q1".to_string(), "Q".to_string(), "Q2".to_string()]),
        un: Some(33.0),
        cmax: Some(1.1),
        cmin: Some(1.0),
        ..Default::default()
    };
    let a = Busbar {
        nodes: Some(vec![
            "AT1".to_string(),
            "M1".to_string(),
            "AT2".to_string(),
            "M2".to_string(),
        ]),
        un: Some(6.0),
        ..Default::default()
    };

    let tr = q.un.unwrap() / (a.un.unwrap() * 1.05);
    let network = NetworkFeeder {
        node: Some("Q".to_string()),
        ur: q.un,
        ikss: Some(13.12), // kA (750 MVA)
        //rx: 0.1, // breaks test
        tr: Some(tr),
        ..Default::default()
    };

    let cable1 = Cable {
        node_i: Some("Q1".to_string()),
        node_j: Some("T1".to_string()),
        rl: Some(0.1),
        xl: Some(0.1),
        l: Some(4.85),
        tr: Some(tr),
        ..Default::default()
    };
    let cable2 = Cable {
        node_i: Some("Q2".to_string()),
        node_j: Some("T2".to_string()),
        rl: Some(0.1),
        xl: Some(0.1),
        l: Some(4.85),
        tr: Some(tr),
        ..Default::default()
    };

    let t1 = NetworkTransformer {
        node_hv: Some("T1".to_string()),
        node_lv: Some("AT1".to_string()),
        sr: Some(15.0 * 1e3), // kVA
        ur_hv: q.un,
        ur_lv: Some(a.un.unwrap() * 1.05),
        urr: Some(0.6),  // %
        ukr: Some(15.0), // %
        ..Default::default()
    };
    let t2 = NetworkTransformer {
        node_hv: Some("T2".to_string()),
        node_lv: Some("AT2".to_string()),
        sr: Some(15.0 * 1e3),
        ur_hv: q.un,
        ur_lv: Some(a.un.unwrap() * 1.05),
        urr: Some(0.6),  // %
        ukr: Some(15.0), // %
        ..Default::default()
    };

    let m1 = AsynchronousMotor {
        node: Some("M1".to_string()),
        ur: Some(6.0),
        pr: Some(5.0 * 1e3), // kW
        cos_phi: Some(0.86),
        eta: Some(97.0),
        ilr_ir: Some(4.0),
        p: Some(2),
        ..Default::default()
    };
    let m2 = AsynchronousMotor {
        node: Some("M2".to_string()),
        ur: Some(6.0),
        pr: Some(1000.0), // kW
        cos_phi: Some(0.83),
        eta: Some(94.0),
        ilr_ir: Some(5.5),
        p: Some(1),
        n: Some(3),
        ..Default::default()
    };

    AcSystem {
        frequency: Some(50.0),

        busbars: Some(vec![q, a]),
        feeders: Some(vec![network]),
        cables: Some(vec![cable1, cable2]),
        transformers: Some(vec![t1, t2]),
        motors: Some(vec![m1, m2]),

        ..Default::default()
    }
}
