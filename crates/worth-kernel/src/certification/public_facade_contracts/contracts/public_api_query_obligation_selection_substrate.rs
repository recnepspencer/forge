#[path = "public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
#[allow(dead_code, unused_imports)]
mod metaboss_support;

use super::public_api_query_obligation_selection_support::primitive_construction_birth_declared_touched_basis;
use metaboss_support::MetabossEventExtractionSubject;
use worth_kernel::query_obligation_selection::selection_substrate::{
    deny_copied_query_obligation_selection_parts,
    deny_in_memory_query_obligation_selection_authority,
    deny_local_query_obligation_selector_authority,
    deny_local_support_row_query_obligation_authority, QueryObligationSelectionAuthorityKind,
    QueryObligationSelectionError, QueryObligationSelectionErrorKind,
    QueryObligationSelectionInput, QueryObligationSelectionSubstrate,
};
use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, WorthWorkload, WorthWorkloadParts,
};
use worth_primitives::PrimitiveWitnessDescriptor;
use worth_spatial::facade::workload_vocabulary::{
    lower_spatial_touch_authority_to_query_descriptor, BooleanEvidenceRowAuthority,
    BooleanEvidenceStageKind, SpatialGeometryEvidenceTouchRequest,
};

#[test]
fn phase2_spatial_descriptor_selects_execution_backed_obligations_through_kernel_substrate() {
    run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase2 query obligation selection substrate");
        let event_ledger_receipt = subject.ledger();
        let completed_workload = completed_workload_with_boolean_receipt(
            subject.pair(),
            event_ledger_receipt,
            BooleanEvidenceStageKind::EventLedger,
        );
        let authority =
            SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(event_ledger_receipt)
                .with_complete_ledger(completed_workload.evidence_ledger())
                .admit()
                .expect("event ledger receipt must admit through the complete workload ledger");
        let lookup = authority
            .spatial_evidence_lookup(completed_workload.evidence_ledger())
            .expect("spatial authority must produce a real lookup product");
        let descriptor = lower_spatial_touch_authority_to_query_descriptor(&authority, &lookup)
            .expect("spatial authority must lower to a Query descriptor");
        let input = QueryObligationSelectionInput::from_spatial_query_descriptor(&descriptor)
            .expect("spatial descriptor must become selection input");
        let selected =
            QueryObligationSelectionSubstrate::select_execution_backed_obligations(input)
                .expect("kernel substrate must select execution-backed obligations");

        assert_eq!(
            selected.authority_kind(),
            QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor
        );
        assert_eq!(
            selected.touch_descriptor_digest(),
            descriptor.touch_descriptor().descriptor_digest()
        );
        assert_eq!(
            selected.operating_world_digest(),
            descriptor.operating_world().descriptor_digest()
        );
        assert_eq!(
            selected.authority_digest(),
            descriptor.product_digest().as_str()
        );
        assert_eq!(selected.selected_obligation_count(), 1);
        assert_eq!(selected.execution_row_count(), 1);
        assert!(selected.execution_proof().has_real_executor_rows());
        assert_selected_closeout_matches_spatial_authority(&selected, &descriptor);
        assert_eq!(
            selected.manifest().execution_proof_digest(),
            Some(selected.execution_proof_digest())
        );
        assert!(!selected.adoption_manifest_digest().is_empty());
        assert_eq!(descriptor.counters().broad_ledger_scan_count(), 0);
    });
}

#[test]
fn phase2_topology_touched_basis_selects_execution_backed_obligations_through_kernel_substrate() {
    let declared_touched_basis = primitive_construction_birth_declared_touched_basis(
        &PrimitiveWitnessDescriptor::SimplexSolid,
        "phase2-simplex",
    );
    let input =
        QueryObligationSelectionInput::from_topology_touched_basis(declared_touched_basis.proof())
            .expect("real topology touched basis proof must become selection input");
    let selected = QueryObligationSelectionSubstrate::select_execution_backed_obligations(input)
        .expect("kernel substrate must select topology execution-backed obligations");
    let closeout = selected.closeout();
    let counters = closeout.selection_counters();

    assert_eq!(
        closeout.authority_kind(),
        QueryObligationSelectionAuthorityKind::TopologyTouchedBasis
    );
    assert_eq!(
        closeout.authority_digest(),
        declared_touched_basis.proof().basis_digest()
    );
    assert_eq!(
        closeout.touch_descriptor_digest(),
        declared_touched_basis
            .proof()
            .touch_descriptor()
            .descriptor_digest()
    );
    assert_eq!(closeout.selected_obligation_count(), 1);
    assert_eq!(closeout.execution_row_count(), 1);
    assert_eq!(closeout.selected_registration_digests().len(), 1);
    assert_eq!(counters.matched_obligation_count(), 1);
    assert_eq!(counters.registration_full_scan_count(), 0);
    assert!(!closeout.execution_proof_digest().is_empty());
    assert!(!closeout.adoption_manifest_digest().is_empty());
    assert!(!closeout.residue_manifest_digest().is_empty());
}

#[test]
fn phase2_selection_substrate_exposes_explicit_denials_for_forbidden_substitutes() {
    assert_denial_kind(
        deny_copied_query_obligation_selection_parts("copied descriptor fields"),
        QueryObligationSelectionErrorKind::CopiedSelectionPartsDenied,
    );
    assert_denial_kind(
        deny_local_query_obligation_selector_authority("worth-local selected row"),
        QueryObligationSelectionErrorKind::LocalSelectorAuthorityDenied,
    );
    assert_denial_kind(
        deny_local_support_row_query_obligation_authority("local support pin row"),
        QueryObligationSelectionErrorKind::LocalSupportRowAuthorityDenied,
    );
    assert_denial_kind(
        deny_in_memory_query_obligation_selection_authority("in-memory adoption proof"),
        QueryObligationSelectionErrorKind::InMemorySelectionAuthorityDenied,
    );
}

fn completed_workload_with_boolean_receipt<T>(
    pair: &BuiltBooleanOperandPairRecipe,
    receipt: &T,
    expected_stage: BooleanEvidenceStageKind,
) -> WorthWorkload
where
    T: BooleanEvidenceRowAuthority + 'static,
{
    assert_eq!(receipt.boolean_stage(), expected_stage);
    let left = pair.left().workload();
    let evidence_ledger = left
        .evidence_ledger()
        .with_boolean_evidence_receipt(receipt)
        .expect("real boolean receipt should extend the complete workload ledger");

    WorthWorkload::compose(WorthWorkloadParts {
        topology: left.topology().clone(),
        geometry_binding: left.geometry_binding().clone(),
        surface_support: left.surface_support().clone(),
        projection: left.projection().clone(),
        transform: left.transform().clone(),
        retained_replay: left.retained_replay().clone(),
        batch_admission_execution: left.batch_admission_execution().cloned(),
        diagnostics: left.diagnostics().clone(),
        response: left.response().clone(),
        evidence_ledger,
    })
    .expect("completed workload should recompose with real boolean evidence")
}

fn assert_selected_closeout_matches_spatial_authority(
    selected: &worth_kernel::query_obligation_selection::selection_substrate::QuerySelectedGraphObligations,
    descriptor: &worth_spatial::facade::workload_vocabulary::SpatialEvidenceQueryTouchDescriptor,
) {
    let closeout = selected.closeout();
    let counters = closeout.selection_counters();

    assert_eq!(
        closeout.authority_kind(),
        QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor
    );
    assert_eq!(
        closeout.touch_descriptor_digest(),
        descriptor.touch_descriptor().descriptor_digest()
    );
    assert_eq!(
        closeout.operating_world_digest(),
        descriptor.operating_world().descriptor_digest()
    );
    assert_eq!(
        closeout.authority_digest(),
        descriptor.product_digest().as_str()
    );
    assert_eq!(closeout.selected_obligation_count(), 1);
    assert_eq!(closeout.execution_row_count(), 1);
    assert_eq!(closeout.selected_registration_digests().len(), 1);
    assert_eq!(counters.matched_obligation_count(), 1);
    assert_eq!(counters.registration_full_scan_count(), 0);
    assert!(!closeout.residue_manifest_digest().is_empty());
}

fn assert_denial_kind(
    error: QueryObligationSelectionError,
    expected: QueryObligationSelectionErrorKind,
) {
    assert_eq!(error.kind(), expected);
    assert!(!error.detail().is_empty());
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("query-obligation-selection-substrate-contract".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("query obligation selection contract thread should spawn")
        .join()
        .expect("query obligation selection contract thread should finish");
}
