use crate::facade::MilestoneThreeHostileCertificationStatus;

use super::certify_hostile_suite_report;

#[test]
fn hostile_suite_category_posture_is_certified_and_gapless() {
    let report = certify_hostile_suite_report("m3.hostile_suite.categories");

    assert_eq!(
        report
            .hostile_certification_category_rows
            .iter()
            .map(|row| row.category.as_str())
            .collect::<Vec<_>>(),
        vec![
            "mutation_pipeline_integrity",
            "primitive_topology_family_closure",
            "operator_brutality",
            "query_traversal_brutality",
            "non_manifold_radial_brutality",
            "degeneracy_corruption_localization",
            "determinism_order_assault",
            "diagnostics_failure_taxonomy",
            "scale_depth_sustained_pressure",
        ]
    );
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .all(|row| {
            row.status == MilestoneThreeHostileCertificationStatus::Certified
                && row.evidence_count > 0
                && row.evidence_count == row.evidence_labels.len()
                && row.scenario_count > 0
                && row.replay_verified_count > 0
                && row.gap_labels.is_empty()
                && row
                    .row_digest
                    .starts_with(&format!("category={};", row.category.as_str()))
        }));
    assert_category_has_evidence(
        &report,
        "primitive_topology_family_closure",
        "primitive_family_edit_closure=WireClosed(n)",
    );
    assert_category_has_evidence(
        &report,
        "operator_brutality",
        "operator_family_closure=RewireLoopEndpoint",
    );
    assert_category_has_evidence(
        &report,
        "query_traversal_brutality",
        "edited_topology_query_traversal=post_edit_loop_cycle_view",
    );
    assert_category_has_evidence(
        &report,
        "scale_depth_sustained_pressure",
        "scale_pressure=large_branch_local_histories",
    );
    assert!(report.operator_family_closure_rows.iter().all(|row| {
        row.legal_execution_count() > 0
            && row.hostile_workload_count() > 0
            && row.replay_evidence_count() > 0
            && row.rejection_evidence_count() > 0
            && row.derived_breadth_evidence_count() > 0
            && row.row_digest().contains("legal_executions=")
            && row.row_digest().contains("derived_breadth=")
    }));
    assert!(report
        .operator_family_closure_rows
        .iter()
        .any(|row| row.branch_local_evidence_count() > 0));
    assert!(report
        .operator_family_closure_rows
        .iter()
        .any(|row| row.localized_rejection_evidence_count() > 0));
}

fn assert_category_has_evidence(
    report: &crate::facade::MilestoneThreeHostileSuiteReport,
    category_name: &str,
    evidence_label: &str,
) {
    assert!(report
        .hostile_certification_category_rows
        .iter()
        .any(|row| row.category.as_str() == category_name
            && row
                .evidence_labels
                .iter()
                .any(|evidence| evidence == evidence_label)));
}




