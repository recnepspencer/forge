use super::{
    certify_topology_historical_materialization_closeout, TopologyHistoricalMaterializationArea,
    TopologyHistoricalMaterializationStatus,
};

#[test]
fn historical_materialization_closeout_certifies_all_phase_seven_areas() {
    let report = certify_topology_historical_materialization_closeout()
        .expect("worth-topo historical materialization closeout should certify");

    assert!(report.phase_seven_ready());
    assert_eq!(
        report.rows().len(),
        TopologyHistoricalMaterializationArea::ALL.len()
    );
    for area in TopologyHistoricalMaterializationArea::ALL {
        assert_eq!(
            report.status(area),
            TopologyHistoricalMaterializationStatus::Closed
        );
    }
    assert!(report.closeout_digest().row_count > 0);
}

#[test]
fn historical_materialization_closeout_names_designated_survivors_for_every_area() {
    let report = certify_topology_historical_materialization_closeout()
        .expect("worth-topo historical materialization closeout should certify");

    for row in report.rows() {
        assert!(!row.reason().is_empty());
        assert!(!row.designated_survivor().is_empty());
        assert!(!row.row_digest().digest_hex.is_empty());
    }
}

#[test]
fn historical_materialization_closeout_proves_phase_seven_completion_rule() {
    let report = certify_topology_historical_materialization_closeout()
        .expect("worth-topo historical materialization closeout should certify");

    let retained_truth_row = report
        .rows()
        .iter()
        .find(|row| row.area() == TopologyHistoricalMaterializationArea::RetainedTruthProjection)
        .expect("historical materialization closeout should cover retained truth projection");

    assert_eq!(
        retained_truth_row.status(),
        TopologyHistoricalMaterializationStatus::Closed
    );
    assert_eq!(
        retained_truth_row.designated_survivor(),
        "src/projection/runtime_boundary/declared_query_surfaces/retained_artifacts.rs"
    );
    assert!(
        retained_truth_row
            .reason()
            .contains("thin topology projection over Query-owned retained artifact floors"),
        "phase seven closeout should state that the surviving topo seam is a thin projection rather than local historical reconstruction",
    );
}
