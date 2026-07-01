use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;
use crate::workload_platform::evidence_lookup_input_admission::{
    current_projection_consumption_receipt, EvidenceLookupQueryAdmissionEvidenceSet,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanTopologyPosture, EvidenceLookupPlanTopologyPostureState,
};

use super::{
    current_path::{
        admit_current_family_stage_cutover_path,
        admit_current_family_stage_cutover_path_with_query_evidence,
    },
    EvidenceLookupCoveredStageCutoverProof, EvidenceLookupStageCutoverErrorKind,
    EvidenceLookupTopologyDerivedReceiptState,
};

#[test]
fn covered_stage_closeout_requires_lookup_receipt() {
    let catalog = current_evidence_lookup_family_catalog().expect("family catalog");
    let family = catalog
        .families_for_stage(
            WorkloadEvidenceStage::BooleanEventLedger,
            &crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        )
        .family_identities()
        .first()
        .and_then(|identity| catalog.family_by_identity(identity))
        .expect("covered family declaration")
        .clone();
    let path = admit_current_family_stage_cutover_path(
        &catalog,
        &family,
        WorkloadEvidenceStage::BooleanEventLedger,
    )
    .expect("current cutover path");
    let proof = path
        .prove_for_family(family.identity().as_str())
        .expect("covered stage proof");

    assert_eq!(proof.family_identity(), family.identity().as_str());
    assert_eq!(proof.stage(), WorkloadEvidenceStage::BooleanEventLedger);
    assert_eq!(
        proof.stage_receipt_identity(),
        path.stage_receipt_identity()
    );
    assert_eq!(
        proof.selected_lookup_plan_digest(),
        path.selected_plan().selected_plan_digest()
    );
    assert_eq!(
        proof.lookup_execution_receipt_digest(),
        path.execution_receipt().execution_receipt_digest()
    );
    assert_eq!(
        proof.lookup_product_output_digest(),
        path.execution_receipt().lookup_product_output_digest()
    );
    assert_eq!(
        proof.selected_equivalence_family_identity(),
        path.execution_receipt()
            .selected_equivalence_family_identity()
    );
    assert_eq!(
        proof.selected_reuse_basis_identity_digest(),
        path.execution_receipt()
            .selected_reuse_basis_identity_digest()
    );
    assert_eq!(proof.counters().raw_row_scan_count(), 0);
    assert_eq!(proof.counters().broad_receipt_scan_count(), 0);
    assert_eq!(proof.counters().caller_owned_scan_count(), 0);
}

#[test]
fn stage_consumption_does_not_expand_lookup_scope() {
    let catalog = current_evidence_lookup_family_catalog().expect("family catalog");
    let family = catalog
        .families_for_stage(
            WorkloadEvidenceStage::BooleanEventLedger,
            &crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        )
        .family_identities()
        .first()
        .and_then(|identity| catalog.family_by_identity(identity))
        .expect("covered family declaration")
        .clone();
    let path = admit_current_family_stage_cutover_path(
        &catalog,
        &family,
        WorkloadEvidenceStage::BooleanEventLedger,
    )
    .expect("current cutover path");

    let denial = EvidenceLookupCoveredStageCutoverProof::prove(
        WorkloadEvidenceStage::BooleanEventLedger,
        path.spatial_touch_authority(),
        path.stage_receipt_identity(),
        "not-in-selected-scope",
        path.selected_plan(),
        path.execution_receipt(),
    )
    .expect_err("foreign family must not expand selected scope");

    assert_eq!(
        denial.kind(),
        EvidenceLookupStageCutoverErrorKind::MissingCoveredFamily
    );
}

#[test]
fn covered_stage_cutover_requires_exact_query_surface_for_projection_families() {
    let catalog = current_evidence_lookup_family_catalog().expect("family catalog");
    let family = catalog
        .family_by_identity("spatial-touch.boolean.projection-consumption-evidence.v1")
        .expect("projection family");
    let denial = admit_current_family_stage_cutover_path_with_query_evidence(
        &catalog,
        family,
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
        None,
        Some(&current_projection_consumption_receipt()),
    )
    .expect_err("projection families must not build cutover path without exact query evidence");
    assert!(
        denial.detail().contains("MissingQueryImportEvidence"),
        "expected query-evidence denial, got {}",
        denial.detail()
    );
}

#[test]
fn covered_stage_cutover_admits_topology_required_support_pin_families_through_query_backed_cutover(
) {
    let catalog = current_evidence_lookup_family_catalog().expect("family catalog");
    let family = catalog
        .family_by_identity("spatial-touch.boolean.overlap-evidence.v1")
        .expect("support-pin family");
    let path = admit_current_family_stage_cutover_path_with_query_evidence(
        &catalog,
        family,
        WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
        Some(
            &EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(
                family
                    .query_posture()
                    .imported_evidence()
                    .expect("support-pin family imports query support"),
            ),
        ),
        None,
    )
    .expect("support-pin families should admit through current topology query-backed cutover");
    let proof = path
        .prove_for_family(family.identity().as_str())
        .expect("topology-required covered family proof");
    assert_eq!(
        proof.stage(),
        WorkloadEvidenceStage::BooleanSharedPlaneIdentity
    );
}

#[test]
fn all_covered_stage_lookups_have_current_cutover_paths() {
    let catalog = current_evidence_lookup_family_catalog().expect("family catalog");

    for family in catalog.declarations() {
        for stage in family.stage_applicability().stages() {
            let path = admit_current_family_stage_cutover_path(&catalog, family, *stage)
                .unwrap_or_else(|error| {
                    panic!(
                        "family `{}` at stage `{:?}` lacks a current cutover path: {}",
                        family.identity().as_str(),
                        stage,
                        error.detail()
                    )
                });
            let proof = path
                .prove_for_family(family.identity().as_str())
                .expect("covered family must prove through current cutover path");
            assert_eq!(proof.stage(), *stage);
            assert!(proof
                .covered_family_identities()
                .contains(&family.identity().as_str().to_string()));
        }
    }
}

#[test]
fn covered_stage_records_topology_state_as_not_required_or_typed_ref() {
    let not_required = EvidenceLookupTopologyDerivedReceiptState::from_plan_topology_posture(
        &EvidenceLookupPlanTopologyPosture::from_state_for_tests(
            EvidenceLookupPlanTopologyPostureState::NotRequired,
        ),
        "phase11-not-required",
    )
    .expect("not required topology state");
    let receipt_ref = EvidenceLookupTopologyDerivedReceiptState::from_plan_topology_posture(
        &EvidenceLookupPlanTopologyPosture::from_state_for_tests(
            EvidenceLookupPlanTopologyPostureState::Satisfied {
                seed_digest: "seed-digest".to_string(),
                receipt_ref_digest: "receipt-ref-digest".to_string(),
                family_identity: "phase11-topology-family",
            },
        ),
        "phase11-topology-family",
    )
    .expect("typed receipt ref topology state");

    assert_eq!(
        not_required,
        EvidenceLookupTopologyDerivedReceiptState::NotRequired
    );
    match receipt_ref {
        EvidenceLookupTopologyDerivedReceiptState::ReceiptRef(receipt_ref) => {
            assert_eq!(receipt_ref.seed_digest(), "seed-digest");
            assert_eq!(receipt_ref.receipt_ref_digest(), "receipt-ref-digest");
            assert_eq!(receipt_ref.family_identity(), "phase11-topology-family");
        }
        other => panic!("expected typed receipt ref, got {other:?}"),
    }
}
