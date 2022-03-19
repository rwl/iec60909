use gridjson::*;

/// Returns the a.c. system from Section 3 of IEC60909-4.
pub fn iec60909_4_3() -> AcSystem {
    let q = Busbar {
        nodes: Some(vec!["Q1".to_string(), "Q".to_string(), "Q2".to_string()]),
        un: Some(20.0), // kV
        cmax: Some(1.1),
        ..Default::default()
    };
    let a = Busbar {
        nodes: Some(vec!["L1".to_string(), "L3".to_string(), "L2".to_string()]),
        un: Some(0.400), // 400V
        cmax: Some(1.05),
        ..Default::default()
    };

    let t1 = NetworkTransformer {
        node_hv: Some("Q1".to_string()),
        node_lv: Some("T1".to_string()),
        sr: Some(630.0),   // kVA
        ur_hv: Some(20.0), // kV
        ur_lv: Some(0.410),
        ukr: Some(4.0), // %
        pkr: Some(6.5), // kW
        //r0r:  1,
        //x0x:  0.95,
        ..Default::default()
    };
    let t2 = NetworkTransformer {
        node_hv: Some("Q2".to_string()),
        node_lv: Some("T2".to_string()),
        sr: Some(400.0),   // kVA
        ur_hv: Some(20.0), // kV
        ur_lv: Some(0.410),
        ukr: Some(4.0), // %
        pkr: Some(4.6), // kW
        //r0r:  1,
        //x0x:  0.95,
        ..Default::default()
    };

    let network = NetworkFeeder {
        node: Some("Q".to_string()),
        ur: q.un,
        ikss: Some(10.0), // kA
        tr: Some(20.0 / t1.ur_lv.unwrap()),
        ..Default::default()
    };

    // Two parallel four-core cables (4 x 240 mm^2 Cu).
    let mut l1 = Cable {
        node_i: Some("T1".to_string()),
        node_j: Some("L1".to_string()),
        l: Some(10.0 / 1000.0), // km
        rl: Some(0.077),        // Ohms/km
        xl: Some(0.079),        // Ohms/km
        parallel: Some(2),
        ..Default::default()
    };
    l1.r_0 = Some(3.7 * l1.rl.unwrap());
    l1.x_0 = Some(1.81 * l1.xl.unwrap());

    // Two parallel three-core cables (3 x 185 mm^2 Al).
    let mut l2 = Cable {
        node_i: Some("T2".to_string()),
        node_j: Some("L2".to_string()),
        l: Some(4.0 / 1000.0),
        rl: Some(0.208),
        xl: Some(0.068),
        parallel: Some(2),
        ..Default::default()
    };
    l2.r_0 = Some(4.23 * l2.rl.unwrap());
    l2.x_0 = Some(1.21 * l2.xl.unwrap());

    // Four-core cable (4 x 70 mm^2 Cu).
    let mut l3 = Cable {
        node_i: Some("L3".to_string()),
        node_j: Some("L4".to_string()),
        l: Some(20.0 / 1000.0),
        rl: Some(0.271),
        xl: Some(0.087),
        ..Default::default()
    };
    l3.r_0 = Some(3.0 * l3.rl.unwrap());
    l3.x_0 = Some(4.46 * l3.xl.unwrap());

    // Overhead line (qn = 50 mm^2 Cu, d = 0.4m).
    let mut l4 = OverheadLine {
        node_i: Some("L4".to_string()),
        node_j: Some("F3".to_string()),

        l: Some(50.0 / 1000.0),
        rl: Some(0.3704),
        xl: Some(0.297),

        qn: Some(50.0),
        rho: Some(1.0 / 54.0),
        d: Some(0.4),

        ..Default::default()
    };
    l4.r_0 = Some(2.0 * l4.rl.unwrap());
    l4.x_0 = Some(3.0 * l4.xl.unwrap());

    AcSystem {
        frequency: Some(50.0),
        busbars: Some(vec![q, a]),
        feeders: Some(vec![network]),
        transformers: Some(vec![t1, t2]),
        cables: Some(vec![l1, l2, l3]),
        lines: Some(vec![l4]),

        ..Default::default()
    }
}
