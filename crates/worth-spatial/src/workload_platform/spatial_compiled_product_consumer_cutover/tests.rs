use crate::replay_family_catalog::current_spatial_replay_family_catalog;
use crate::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input,
    current_boolean_event_ledger_spatial_boundary,
    lower_spatial_replay_scope_product_from_admitted_input,
    prepare_spatial_replay_semantic_graph_request, SpatialReplaySemanticGraphPreparationRequest,
};
use crate::workload_platform::evidence_lookup_reuse_decision::{
    EvidenceLookupIndexReuseResolution, EvidenceLookupReuseMismatchLocus,
};
use crate::workload_platform::spatial_compiled_product_consumer_cutover::reuse_evidence_lookup_index_product;

mod residue_tests;

#[test]
fn spatial_consumers_route_through_reuse_decision_products() {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");
    let (
        replay_scope_handoff_identity,
        replay_scope_family_identity,
        replay_scope_reuse_basis_identity,
        replay_scope_product_identity,
    ) = {
        let request = prepare_spatial_replay_semantic_graph_request(
            SpatialReplaySemanticGraphPreparationRequest::new(
                boundary.replay_family_identity(),
                boundary.authority(),
                boundary.execution_receipt(),
                boundary.workload_handoff(),
            )
            .with_retained_replay_receipt(
                boundary
                    .retained_replay_receipt()
                    .expect("current boolean-event boundary should carry retained replay receipt"),
            ),
        )
        .expect("prepared replay request");
        let admitted = admit_prepared_spatial_replay_semantic_graph_input(
            &current_spatial_replay_family_catalog(),
            &request,
        )
        .expect("admitted replay request");
        let replay_scope = lower_spatial_replay_scope_product_from_admitted_input(&admitted)
            .expect("replay scope lowers through the shared consumer cutover lane");
        (
            replay_scope
                .lookup_consumed_workload_handoff_identity()
                .to_string(),
            replay_scope
                .lookup_consumed_workload_handoff()
                .selected_equivalence_family_identity()
                .to_string(),
            replay_scope
                .lookup_consumed_workload_handoff()
                .selected_reuse_basis_identity_digest()
                .to_string(),
            replay_scope.scope_product_identity().digest().to_string(),
        )
    };
    let reuse_resolution = reuse_evidence_lookup_index_product(
        boundary.selected_plan(),
        boundary.selected_lookup_slice(),
        boundary.index_product(),
    )
    .expect("evidence lookup reuse should lower through the shared consumer cutover lane");

    let EvidenceLookupIndexReuseResolution::Reused { product, .. } = reuse_resolution else {
        panic!("current boundary product must stay reusable through the shared cutover lane");
    };

    assert_eq!(
        boundary
            .workload_handoff()
            .selected_equivalence_family_identity(),
        boundary
            .execution_receipt()
            .selected_equivalence_family_identity()
    );
    assert_eq!(
        boundary
            .workload_handoff()
            .selected_reuse_basis_identity_digest(),
        boundary
            .execution_receipt()
            .selected_reuse_basis_identity_digest()
    );
    assert_eq!(
        product.selected_equivalence_family_identity().as_str(),
        boundary
            .workload_handoff()
            .selected_equivalence_family_identity()
    );
    assert_eq!(
        product.selected_reuse_basis_identity_digest(),
        boundary
            .workload_handoff()
            .selected_reuse_basis_identity_digest()
    );
    assert_eq!(
        replay_scope_family_identity,
        boundary
            .workload_handoff()
            .selected_equivalence_family_identity()
    );
    assert_eq!(
        replay_scope_reuse_basis_identity,
        boundary
            .workload_handoff()
            .selected_reuse_basis_identity_digest()
    );
    assert_eq!(
        replay_scope_handoff_identity,
        boundary.workload_handoff().semantic_graph_identity()
    );

    let hostile_prior_product = boundary
        .index_product()
        .clone()
        .with_test_selected_equivalence_family_identity(
            "spatial.selected-equivalence.retained-replay-semantic-parity",
        );
    let hostile_resolution = reuse_evidence_lookup_index_product(
        boundary.selected_plan(),
        boundary.selected_lookup_slice(),
        &hostile_prior_product,
    )
    .expect("hostile prior product should still lower as a typed denial");
    let EvidenceLookupIndexReuseResolution::Denied { denial, .. } = hostile_resolution else {
        panic!("forged selected-family prior product must deny reuse");
    };
    assert_eq!(
        denial.mismatch_loci(),
        &[EvidenceLookupReuseMismatchLocus::SelectedEquivalenceFamilyIdentity]
    );

    let hostile_handoff = boundary
        .workload_handoff()
        .clone()
        .with_test_selected_equivalence_family_identity(
            "spatial.selected-equivalence.retained-replay-semantic-parity",
        );
    let (
        hostile_scope_handoff_identity,
        hostile_scope_family_identity,
        hostile_scope_product_identity,
    ) = {
        let request = prepare_spatial_replay_semantic_graph_request(
            SpatialReplaySemanticGraphPreparationRequest::new(
                boundary.replay_family_identity(),
                boundary.authority(),
                boundary.execution_receipt(),
                &hostile_handoff,
            )
            .with_retained_replay_receipt(
                boundary
                    .retained_replay_receipt()
                    .expect("current boolean-event boundary should carry retained replay receipt"),
            ),
        )
        .expect("prepared hostile replay request");
        let admitted = admit_prepared_spatial_replay_semantic_graph_input(
            &current_spatial_replay_family_catalog(),
            &request,
        )
        .expect("admitted hostile replay request");
        let hostile_scope = lower_spatial_replay_scope_product_from_admitted_input(&admitted)
            .expect("hostile replay scope lowers through the shared lane");
        (
            hostile_scope
                .lookup_consumed_workload_handoff_identity()
                .to_string(),
            hostile_scope
                .lookup_consumed_workload_handoff()
                .selected_equivalence_family_identity()
                .to_string(),
            hostile_scope.scope_product_identity().digest().to_string(),
        )
    };

    assert_eq!(
        hostile_scope_family_identity,
        hostile_handoff.selected_equivalence_family_identity()
    );
    assert_ne!(
        hostile_scope_handoff_identity,
        replay_scope_handoff_identity
    );
    assert_ne!(
        hostile_scope_product_identity,
        replay_scope_product_identity
    );
}

#[test]
fn spatial_cutover_preserves_zero_broad_scan_fallback() {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");
    let evidence_resolution = reuse_evidence_lookup_index_product(
        boundary.selected_plan(),
        boundary.selected_lookup_slice(),
        boundary.index_product(),
    )
    .expect("evidence route lowers through the shared consumer cutover lane");
    let hostile_prior_product = boundary
        .index_product()
        .clone()
        .with_test_selected_equivalence_family_identity(
            "spatial.selected-equivalence.retained-replay-semantic-parity",
        );
    let hostile_resolution = reuse_evidence_lookup_index_product(
        boundary.selected_plan(),
        boundary.selected_lookup_slice(),
        &hostile_prior_product,
    )
    .expect("hostile prior product still lowers through the evidence route");
    let replay_scope_counters = {
        let request = prepare_spatial_replay_semantic_graph_request(
            SpatialReplaySemanticGraphPreparationRequest::new(
                boundary.replay_family_identity(),
                boundary.authority(),
                boundary.execution_receipt(),
                boundary.workload_handoff(),
            )
            .with_retained_replay_receipt(
                boundary
                    .retained_replay_receipt()
                    .expect("current boolean-event boundary should carry retained replay receipt"),
            ),
        )
        .expect("prepared replay request");
        let admitted = admit_prepared_spatial_replay_semantic_graph_input(
            &current_spatial_replay_family_catalog(),
            &request,
        )
        .expect("admitted replay request");
        lower_spatial_replay_scope_product_from_admitted_input(&admitted)
            .expect("replay scope lowers through the shared consumer cutover lane")
            .counters()
            .clone()
    };

    assert_eq!(
        boundary.workload_handoff().counters().raw_row_scan_count(),
        0
    );
    assert_eq!(
        boundary
            .workload_handoff()
            .counters()
            .broad_receipt_scan_count(),
        0
    );
    assert_eq!(
        boundary
            .workload_handoff()
            .counters()
            .caller_owned_scan_count(),
        0
    );
    assert_eq!(replay_scope_counters.raw_row_scan_count(), 0);
    assert_eq!(replay_scope_counters.broad_receipt_scan_count(), 0);
    assert_eq!(replay_scope_counters.caller_owned_scan_count(), 0);
    assert_eq!(replay_scope_counters.retained_replay_binding_count(), 1);
    assert_eq!(
        evidence_resolution
            .decision()
            .counters()
            .raw_evidence_row_scan_count(),
        0
    );
    assert_eq!(
        evidence_resolution
            .decision()
            .counters()
            .broad_receipt_scan_count(),
        0
    );
    assert_eq!(
        evidence_resolution
            .decision()
            .counters()
            .caller_owned_evidence_work_count(),
        0
    );
    assert_eq!(
        hostile_resolution
            .decision()
            .counters()
            .raw_evidence_row_scan_count(),
        0
    );
    assert_eq!(
        hostile_resolution
            .decision()
            .counters()
            .broad_receipt_scan_count(),
        0
    );
    assert_eq!(
        hostile_resolution
            .decision()
            .counters()
            .caller_owned_evidence_work_count(),
        0
    );
}
