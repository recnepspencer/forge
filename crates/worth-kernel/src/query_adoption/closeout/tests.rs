use super::*;

#[test]
fn closeout_report_proves_gate_counts_and_doc_agreement() {
    let report = current_worth_query_native_hardening_closeout_report()
        .expect("query-native hardening closeout");

    assert_eq!(report.audited_source_set_count(), 17);
    assert_eq!(report.admitted_source_set_count(), 9);
    assert_eq!(report.denied_source_set_count(), 5);
    assert_eq!(report.explicit_residue_source_set_count(), 3);
    assert_eq!(report.support_requirement_count(), 7);
    assert_eq!(report.support_observed_row_count(), 8);
    assert_eq!(report.support_matched_required_count(), 7);
    assert_eq!(report.support_snapshot_row_count(), 68);
    assert_eq!(report.boundary_audit_source_count(), 5);
    assert_eq!(report.synthetic_denial_localization_row_count(), 5);
    assert_eq!(report.kernel_receipt_breadth_count(), 8);
    assert_eq!(report.lower_crate_receipt_family_count(), 2);
    assert_eq!(report.topology_read_touched_scope_count(), 4);
    assert_eq!(report.spatial_witness_resolution_request_count(), 8);
    assert_eq!(report.spatial_witness_denial_count(), 4);
    assert_eq!(report.spatial_witness_catalog_lookup_count(), 2);
    assert!(report.closeout_doc_agrees());
    assert!(report.ai_readme_agrees());
    assert!(report.roadmap_sequencing_agrees());
    assert!(report.gate_closed());
}

#[test]
fn closeout_keeps_known_synthetic_source_families_out_of_production_proof() {
    let synthetic = WorthQuerySyntheticProofDispositionReport::current()
        .expect("synthetic proof disposition report");

    for source_set in [
        "crates/worth-kernel/src/certification/public_facade_contracts",
        "crates/worth-spatial/src/certification/public_facade_contracts",
        "crates/worth-spatial/src/test_support",
        "crates/worth-topo/src/test_support",
        "crates/worth-topo/tests/ui",
    ] {
        assert_source_set_has_disposition(
            &synthetic,
            source_set,
            WorthQuerySyntheticProofDisposition::DeniedByBoundary,
        );
    }

    for source_set in [
        "crates/worth-kernel/src/binding/tests",
        "crates/worth-spatial/src/workload_platform/vocabulary",
        "crates/worth-topo/src/projection/runtime_boundary/query_support",
    ] {
        assert_source_set_has_disposition(
            &synthetic,
            source_set,
            WorthQuerySyntheticProofDisposition::ExplicitResidue,
        );
    }
}

fn assert_source_set_has_disposition(
    synthetic: &WorthQuerySyntheticProofDispositionReport,
    source_set: &str,
    expected: WorthQuerySyntheticProofDisposition,
) {
    let row = synthetic
        .require_source_set(source_set)
        .unwrap_or_else(|| panic!("missing synthetic source set {source_set}"));
    assert_eq!(row.disposition(), expected);
}
