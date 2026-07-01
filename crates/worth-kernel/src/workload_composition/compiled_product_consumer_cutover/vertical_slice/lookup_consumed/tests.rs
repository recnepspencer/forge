use super::{
    admitted_slice::LookupConsumedVerticalSlice,
    displaced_surface::{
        current_lookup_consumed_vertical_slice_displaced_surfaces,
        LookupConsumedVerticalSliceDisplacedSurfaceDisposition,
    },
};
use crate::workload_composition::{
    admit_spatial_conflict_input, lower_selected_spatial_conflict_plan,
    AdmittedSpatialConflictRoute, LookupConsumedWorkloadDenial, SpatialConflictInputRequest,
    WorkloadCompositionError,
};
use worth_spatial::facade::replay_undo_semantic_graph::current_boolean_event_ledger_spatial_boundary;
use worth_spatial::touched_graph_conflict::current_spatial_conflict_family_catalog_closeout;

#[test]
fn vertical_slice_matches_or_strengthens_old_reuse_posture() {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");
    let closeout = current_spatial_conflict_family_catalog_closeout().expect("catalog closes");
    let legacy_input = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(boundary.authority())
            .with_evidence_lookup(boundary.workload_handoff(), boundary.execution_receipt()),
    )
    .expect("legacy direct route still defines the displaced posture");
    let slice = LookupConsumedVerticalSlice::admit(&boundary).expect("vertical slice admit");
    let resolved = slice
        .resolve_prior_lookup_product(boundary.index_product())
        .expect("vertical slice resolves current product reuse");
    let migrated_input = resolved
        .admit_spatial_conflict_input()
        .expect("vertical slice lowers ordinary conflict input");
    let legacy_plan = lower_selected_spatial_conflict_plan(&closeout, &legacy_input);
    let migrated_plan = lower_selected_spatial_conflict_plan(&closeout, &migrated_input);
    let resolved_digest = match resolved.reuse_product() {
        super::reuse_resolution::LookupConsumedVerticalSliceReuseProduct::Reused(product)
        | super::reuse_resolution::LookupConsumedVerticalSliceReuseProduct::Rebuilt(product) => {
            product.index_product_digest()
        }
    };

    match migrated_input.route() {
        AdmittedSpatialConflictRoute::LookupCompiledProduct { product, .. } => {
            assert_eq!(
                product.index_product_digest(),
                resolved_digest,
                "migrated vertical slice must lower through the resolved typed compiled product at runtime",
            );
        }
        AdmittedSpatialConflictRoute::EvidenceLookup { .. } => {
            panic!(
                "migrated vertical slice must not fall back to the displaced receipt-backed route"
            )
        }
        AdmittedSpatialConflictRoute::ReplayBoundary(_) => {
            panic!(
                "migrated vertical slice must stay on the lookup-consumed compiled-product route"
            )
        }
    }

    assert_eq!(
        migrated_input.routing_contract().contract_digest(),
        legacy_input.routing_contract().contract_digest(),
        "compiled-product-backed lowering must preserve the admitted routing contract even though it no longer reuses the legacy route digest",
    );
    assert_eq!(
        migrated_plan.overlap_category(),
        legacy_plan.overlap_category()
    );
    assert_eq!(
        migrated_plan.locality_footprint_digest(),
        legacy_plan.locality_footprint_digest()
    );
    assert_eq!(
        migrated_plan.prior_proof_posture(),
        legacy_plan.prior_proof_posture()
    );
    assert_eq!(
        migrated_plan.downstream_proof_category(),
        legacy_plan.downstream_proof_category()
    );
    assert_eq!(migrated_plan.counters(), legacy_plan.counters());
    assert_eq!(
        migrated_plan.execution_admission(),
        legacy_plan.execution_admission()
    );
    assert_eq!(migrated_plan.denial(), legacy_plan.denial());
    assert_eq!(
        migrated_plan
            .selected_families()
            .iter()
            .map(|row| row.identity().as_str().to_string())
            .collect::<Vec<_>>(),
        legacy_plan
            .selected_families()
            .iter()
            .map(|row| row.identity().as_str().to_string())
            .collect::<Vec<_>>(),
        "vertical slice execution must preserve the selected spatial conflict families produced by the displaced direct route",
    );
}

#[test]
fn vertical_slice_rebuilt_posture_owns_runtime_lookup_compiled_product_route() {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");
    let slice = LookupConsumedVerticalSlice::admit(&boundary).expect("vertical slice admit");
    let hostile_prior_product = boundary
        .index_product()
        .clone()
        .with_test_selected_reuse_basis_identity_digest("phase-10-rebuilt-selected-reuse-basis");
    let resolved = slice
        .resolve_prior_lookup_product(&hostile_prior_product)
        .expect("vertical slice should rebuild from hostile prior product");

    let rebuilt_product = match resolved.reuse_product() {
        super::reuse_resolution::LookupConsumedVerticalSliceReuseProduct::Rebuilt(product) => {
            product
        }
        super::reuse_resolution::LookupConsumedVerticalSliceReuseProduct::Reused(_) => {
            panic!("hostile prior product must yield rebuilt posture rather than reused posture")
        }
    };

    let admitted = resolved
        .admit_spatial_conflict_input()
        .expect("rebuilt vertical slice product should still lower ordinary conflict input");

    match admitted.route() {
        AdmittedSpatialConflictRoute::LookupCompiledProduct { product, .. } => {
            assert_eq!(
                product.index_product_digest(),
                rebuilt_product.index_product_digest(),
                "rebuilt vertical slice must own runtime lowering through the rebuilt compiled product",
            );
            assert!(
                std::ptr::eq(product, rebuilt_product),
                "rebuilt vertical slice must lower the exact resolved rebuilt product rather than a substituted compiled product",
            );
            assert!(
                !std::ptr::eq(product, boundary.index_product()),
                "rebuilt vertical slice must not silently fall back to the boundary current product",
            );
        }
        AdmittedSpatialConflictRoute::EvidenceLookup { .. } => {
            panic!(
                "rebuilt vertical slice must not fall back to the displaced receipt-backed route"
            )
        }
        AdmittedSpatialConflictRoute::ReplayBoundary(_) => {
            panic!("rebuilt vertical slice must stay on the lookup-consumed compiled-product route")
        }
    }
}

#[test]
fn old_reuse_helper_cannot_satisfy_migrated_slice() {
    let displaced = current_lookup_consumed_vertical_slice_displaced_surfaces();
    assert_eq!(displaced.len(), 1);
    assert_eq!(
        displaced[0].current_surface(),
        "current_worth_workload_ordinary_consumer_batch_execution_receipt direct evidence-lookup conflict-input lowering"
    );
    assert_eq!(
        displaced[0].current_path(),
        "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/current_cutover_proof.rs"
    );
    assert_eq!(
        displaced[0].disposition(),
        LookupConsumedVerticalSliceDisplacedSurfaceDisposition::DeletedNow
    );
    assert!(
        !include_str!("../../../worth_workload/ordinary_consumer_sweep/current_cutover_proof.rs")
            .contains(".with_evidence_lookup("),
        "migrated caller must not keep the displaced direct evidence-lookup helper on the ordinary route",
    );
    assert!(
        !include_str!("execution_posture.rs").contains(".with_evidence_lookup("),
        "vertical slice execution must lower from the typed reuse product rather than re-entering the displaced evidence-lookup helper",
    );
}

#[test]
fn slice_preserves_authority_vs_derived_distinction() {
    let boundary = current_boolean_event_ledger_spatial_boundary().expect("current boundary");
    let slice = LookupConsumedVerticalSlice::admit(&boundary).expect("vertical slice admit");
    let hostile_prior_product = boundary
        .index_product()
        .clone()
        .with_test_selected_equivalence_family_identity(
            "spatial.selected-equivalence.retained-replay-semantic-parity",
        );

    let error = slice
        .resolve_prior_lookup_product(&hostile_prior_product)
        .expect_err("vertical slice must reject prior product that forges derived family identity");

    assert!(matches!(
        error,
        WorkloadCompositionError::LookupConsumedWorkload(
            LookupConsumedWorkloadDenial::ReuseResolutionDenied(_)
        ) | WorkloadCompositionError::LookupConsumedWorkload(
            LookupConsumedWorkloadDenial::ReuseResolutionSelectedFamilyMismatch
        )
    ));
}
