use hadwiger_research::facade::{
    admit_hadwiger_research_handle, test_g27_dual_unit_anchor_pair_checked,
    G27DualUnitAnchorPosture, HadwigerCanonicalArtifact, HadwigerResearchOperatingContext,
};

#[test]
fn round1_falsifies_8_18_outside_moser_anchor() {
    let report = dual_anchor_report("8", "18", ["13", "6"]);

    assert_eq!(report.left_vertex(), "8");
    assert_eq!(report.right_vertex(), "18");
    assert_eq!(report.comparison_vertices(), &["13", "6"]);
    assert_moser_capped(&report);
    assert_eq!(report.anchors()[0].coefficients(), [0, 3, 3, 1]);
    assert_eq!(report.anchors()[1].coefficients(), [1, 2, 3, 2]);
    assert!(report.conclusion().contains("Moser-basis"));
}

#[test]
fn round2_falsifies_13_6_image_side_anchor() {
    let report = dual_anchor_report("13", "6", ["8", "18"]);

    assert_eq!(report.left_vertex(), "13");
    assert_eq!(report.right_vertex(), "6");
    assert_eq!(report.comparison_vertices(), &["8", "18"]);
    assert_moser_capped(&report);
    assert_eq!(report.anchors()[0].coefficients(), [1, 3, 2, 1]);
    assert_eq!(report.anchors()[1].coefficients(), [2, 3, 1, 2]);
    assert!(report
        .anchors()
        .iter()
        .all(|anchor| anchor.comparison_profile()
            == [("8".to_string(), false), ("18".to_string(), false)]));
}

#[test]
fn round3_falsifies_21_26_second_tight_pair_anchor() {
    let report = dual_anchor_report("21", "26", ["8", "18"]);

    assert_eq!(report.left_vertex(), "21");
    assert_eq!(report.right_vertex(), "26");
    assert_eq!(report.comparison_vertices(), &["8", "18"]);
    assert_moser_capped(&report);
    assert_eq!(report.anchors()[0].coefficients(), [2, 1, 2, 3]);
    assert_eq!(report.anchors()[1].coefficients(), [2, 2, 1, 3]);
    assert_eq!(
        report.anchors()[1].comparison_profile(),
        &[("8".to_string(), false), ("18".to_string(), true)]
    );
}

#[test]
fn round4_falsifies_5_20_next_tight_pair_anchor() {
    let report = dual_anchor_report("5", "20", ["8", "18"]);

    assert_eq!(report.left_vertex(), "5");
    assert_eq!(report.right_vertex(), "20");
    assert_eq!(report.comparison_vertices(), &["8", "18"]);
    assert_moser_capped(&report);
    assert_eq!(report.anchors()[0].coefficients(), [2, 1, 1, 3]);
    assert_eq!(report.anchors()[1].coefficients(), [2, 2, 2, 1]);
    assert!(report
        .anchors()
        .iter()
        .all(|anchor| anchor.comparison_profile()
            == [("8".to_string(), false), ("18".to_string(), false)]));
}

#[test]
fn round5_falsifies_8_16_self_anchor_variant() {
    let report = dual_anchor_report("8", "16", ["8", "18"]);

    assert_eq!(report.left_vertex(), "8");
    assert_eq!(report.right_vertex(), "16");
    assert_eq!(report.comparison_vertices(), &["8", "18"]);
    assert_moser_capped(&report);
    assert_eq!(report.anchors()[0].coefficients(), [1, 2, 2, 2]);
    assert_eq!(report.anchors()[1].coefficients(), [1, 3, 3, 1]);
    assert!(report
        .anchors()
        .iter()
        .all(|anchor| anchor.comparison_profile()
            == [("8".to_string(), true), ("18".to_string(), false)]));
}

fn dual_anchor_report(
    left: &str,
    right: &str,
    comparison: [&str; 2],
) -> hadwiger_research::facade::G27DualUnitAnchorTestReport {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");
    test_g27_dual_unit_anchor_pair_checked(&handle, left, right, comparison)
        .expect("dual-unit anchor test runs")
}

fn assert_moser_capped(report: &hadwiger_research::facade::G27DualUnitAnchorTestReport) {
    assert_eq!(
        report.posture(),
        G27DualUnitAnchorPosture::MoserCappedExhaustive
    );
    assert!(report.falsifies_outside_moser_dual_anchor());
    assert_eq!(report.anchors().len(), 2);
    assert_eq!(report.parent_artifacts().len(), 1);
    assert!(!report.admits_theorem_authority());
    assert!(!report.registers_query_invariant_authority());
}
