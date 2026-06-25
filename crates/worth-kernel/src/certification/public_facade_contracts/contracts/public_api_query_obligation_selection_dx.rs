use super::public_api_query_obligation_selection_real_spatial_authority_support::real_spatial_authority_case;
use super::public_api_query_obligation_selection_support::primitive_construction_birth_cases;
use worth_kernel::workload_composition::{
    QueryGraphObligationSelectionAuthorityKind, QueryGraphObligationSelectionFacadeErrorKind,
    WorkloadCatalog, WorthQuerySelectorPrecisionPosture, WorthWorkload,
};
use worth_spatial::facade::workload_vocabulary::lower_spatial_touch_authority_to_query_descriptor;

#[test]
fn public_dx_selects_obligations_from_topology_touched_authority_only() {
    let workload = public_selection_workload();
    for case in primitive_construction_birth_cases() {
        let touched_basis = case.declared_touched_basis("phase7-public-dx");
        let selected = workload
            .select_query_graph_obligations(&touched_basis)
            .expect("public workload facade must select from every topology touched basis family");
        let closeout = selected.closeout();

        assert!(selected.execution_proof().has_real_executor_rows());
        assert_eq!(
            selected.touch_descriptor_digest(),
            touched_basis.proof().touch_descriptor().descriptor_digest(),
            "family {:?} selected the wrong touch descriptor",
            case.family()
        );
        assert_eq!(
            selected.authority_digest(),
            touched_basis.proof().basis_digest(),
            "family {:?} selected the wrong authority digest",
            case.family()
        );
        assert_eq!(selected.selected_obligation_count(), 1);
        assert_eq!(selected.execution_row_count(), 1);
        assert_eq!(selected.selected_registration_digests().len(), 1);
        assert_eq!(
            closeout.authority_kind(),
            QueryGraphObligationSelectionAuthorityKind::TopologyTouchedBasis
        );
        assert_precision_is_touched_descriptor_bounded(&closeout.selector_precision_report());
        assert!(closeout.local_ceremony_is_clean());
        assert!(!closeout.graph_read_access_planning_claimed());
    }
}

#[test]
fn public_dx_selects_obligations_from_spatial_descriptor_without_graph_read_claim() {
    let authority_case = real_spatial_authority_case("phase7-public-spatial-dx");
    let descriptor = lower_spatial_touch_authority_to_query_descriptor(
        authority_case.authority(),
        authority_case.lookup(),
    )
    .expect("real spatial authority must lower to Query descriptor");

    let selected = authority_case
        .workload()
        .select_query_graph_obligations(&descriptor)
        .expect("public workload facade must select spatial Query obligations");
    let closeout = selected.closeout();

    assert!(selected.execution_proof().has_real_executor_rows());
    assert_eq!(
        selected.touch_descriptor_digest(),
        descriptor.touch_descriptor().descriptor_digest()
    );
    assert_eq!(
        selected.spatial_touch_digest(),
        Some(descriptor.spatial_touch_digest().as_str())
    );
    assert_eq!(
        selected.spatial_lookup_product_digest(),
        Some(descriptor.lookup_product_digest().as_str())
    );
    assert_eq!(selected.selected_obligation_count(), 1);
    assert_eq!(
        closeout.authority_kind(),
        QueryGraphObligationSelectionAuthorityKind::SpatialQueryDescriptor
    );
    assert_eq!(
        closeout.spatial_query_gap_rows(),
        descriptor.gap_rows().len()
    );
    assert_spatial_precision_is_counter_bounded_with_declared_gaps(
        &closeout.selector_precision_report(),
        descriptor.gap_rows().len(),
    );
    assert!(!closeout.graph_read_access_planning_claimed());
}

#[test]
fn public_dx_rejects_spatial_descriptor_from_a_different_workload() {
    let authority_case = real_spatial_authority_case("phase7-public-spatial-mismatch");
    let descriptor = lower_spatial_touch_authority_to_query_descriptor(
        authority_case.authority(),
        authority_case.lookup(),
    )
    .expect("real spatial authority must lower to Query descriptor");
    let unrelated_workload = public_selection_workload();

    let error = unrelated_workload
        .select_query_graph_obligations(&descriptor)
        .expect_err("spatial Query descriptor must remain bound to its minting workload");

    assert_eq!(
        error.kind(),
        QueryGraphObligationSelectionFacadeErrorKind::WorkloadAuthorityMismatch
    );
    assert!(
        error.detail().contains("stage index"),
        "mismatch detail should explain the workload authority boundary: {}",
        error.detail()
    );
}

fn public_selection_workload() -> WorthWorkload {
    WorkloadCatalog::cube()
        .with_retained_replay_artifacts()
        .build()
        .expect("catalog cube workload should build")
        .into_workload()
}

fn assert_precision_is_touched_descriptor_bounded(
    report: &worth_kernel::workload_composition::WorthQuerySelectorPrecisionReport,
) {
    assert_eq!(
        report.posture(),
        WorthQuerySelectorPrecisionPosture::TouchedDescriptorBounded
    );
    assert!(report.is_touched_descriptor_bounded());
    assert!(report.has_touched_descriptor_bounded_counters());
    assert!(report.has_clean_selector_closeout());
    assert_eq!(report.registration_full_scan_count(), 0);
    assert_eq!(report.broad_selector_residue_count(), 0);
    assert_eq!(report.query_selector_gap_count(), 0);
    assert_eq!(
        report.matched_obligation_count(),
        report.selected_obligation_count()
    );
    assert!(
        !report.counters_digest().is_empty() && !report.report_digest().is_empty(),
        "precision report must expose read-only counter provenance"
    );
}

fn assert_spatial_precision_is_counter_bounded_with_declared_gaps(
    report: &worth_kernel::workload_composition::WorthQuerySelectorPrecisionReport,
    expected_gap_rows: usize,
) {
    assert_eq!(
        report.posture(),
        WorthQuerySelectorPrecisionPosture::QueryExpressivenessGap
    );
    assert!(report.has_touched_descriptor_bounded_counters());
    assert_eq!(report.registration_full_scan_count(), 0);
    assert_eq!(report.broad_selector_residue_count(), 1);
    assert_eq!(report.query_selector_gap_count(), expected_gap_rows);
    assert_eq!(
        report.matched_obligation_count(),
        report.selected_obligation_count()
    );
    assert!(
        !report.counters_digest().is_empty() && !report.report_digest().is_empty(),
        "spatial precision report must expose read-only counter provenance"
    );
}
