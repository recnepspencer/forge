use crate::construction::certification::corpus::replay_siege_report::prepare_primitive_construction_corpus_replay_siege_report;

#[test]
fn phase_five_corpus_closeout_report_proves_named_replay_siege_artifact_family() {
    let report =
        prepare_primitive_construction_corpus_replay_siege_report("phase-five.corpus-closeout");

    assert!(report.required_scenario_coverage_verified());
    assert!(report.row_digest_uniqueness_verified());
    assert!(report.authoring_order_lane_coverage_verified());
    assert!(report.authoring_order_parity_verified());
    assert!(report.authoring_order_digest_uniqueness_verified());
    assert_eq!(
        report.lane_names(),
        vec!["canonical", "reversed", "rejected_first", "role_clustered"]
    );
    assert!(report.accepted_count() > 0);
    assert!(report.rejected_count() > 0);
    assert!(report
        .rows()
        .iter()
        .all(|row| !row.current_head_lane().lane_digest().is_empty()));
    assert!(report.rows().iter().all(|row| {
        row.branch_local_lane().execution_gap().code() == "branch_local_execution_surface_missing"
            && !row
                .branch_local_lane()
                .preview_admission_digest()
                .is_empty()
            && !row.branch_local_lane().branch_admission_digest().is_empty()
    }));
    assert!(report.rows().iter().all(|row| {
        row.replay_lane().replay_gap().code() == "historical_replay_execution_surface_missing"
    }));
    assert!(!report.report_digest().is_empty());
}
