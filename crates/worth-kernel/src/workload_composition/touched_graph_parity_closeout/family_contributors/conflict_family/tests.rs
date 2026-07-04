use schema::facade::platform::authority::touched_graph_conflict::{
    BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
};
use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};

use super::contributor_catalog::{
    current_conflict_family_contributor_catalog, ConflictFamilyContributorCatalog,
};
use super::parity::{
    conflict_family_parity_claim_from_catalog, current_conflict_family_parity_claim,
    ConflictFamilyParityErrorKind,
};
use super::row::{ConflictFamilyContributorRowKind, ConflictFamilyDenialWitnessKind};

#[test]
fn conflict_independence_batch_families_share_one_semantic_graph_language() {
    let catalog = current_conflict_family_contributor_catalog().expect("conflict-family catalog");
    let claim = current_conflict_family_parity_claim().expect("conflict-family parity claim");

    assert_eq!(
        claim.kind(),
        TouchedGraphParityClaimKind::SelectedRouteParity
    );
    assert_eq!(catalog.rows().len(), 3);
    assert_eq!(claim.rows().len(), 3);
    assert!(catalog.rows().iter().all(|row| {
        row.family_kind() == TouchedGraphParityFamilyKind::ConflictIndependenceBatchAdmission
    }));

    let batch = catalog
        .rows()
        .iter()
        .find(|row| row.kind() == ConflictFamilyContributorRowKind::BatchAdmission)
        .expect("batch row");
    assert_eq!(
        batch.current_packet_or_identity_source(),
        "current_worth_touched_graph_conflict_selected_route_packet::conflict_family_batch_pre_execution_identity"
    );
    assert!(batch
        .carried_overlap_or_plan_source()
        .contains("selected_batch_plan_digest"));
    assert_eq!(
        batch.denial_witness_fields_produced(),
        &[
            "batch_admission_denial_witness_identity",
            "batch_admission_denial_witness_kind",
        ]
    );
}

#[test]
fn conflict_independence_batch_parity_rejects_result_only_equivalence() {
    let mut hostile_rows = current_conflict_family_contributor_catalog()
        .expect("conflict-family catalog")
        .rows()
        .to_vec();
    hostile_rows[0] = hostile_rows[0].clone().with_test_identity_override(
        hostile_rows[0].current_packet_identity(),
        &["foreign-overlap"],
        &["foreign-conflict-plan"],
        &[],
        "",
        hostile_rows[0].denial_witness_identity(),
        hostile_rows[0].denial_witness_kind(),
    );

    assert_eq!(
        conflict_family_parity_claim_from_catalog(
            &ConflictFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("result-only equivalence must not pass parity")
        .kind(),
        ConflictFamilyParityErrorKind::MismatchedConflictIdentity
    );
}

#[test]
fn conflict_family_parity_rejects_independence_identity_drift() {
    let mut hostile_rows = current_conflict_family_contributor_catalog()
        .expect("conflict-family catalog")
        .rows()
        .to_vec();
    hostile_rows[1] = hostile_rows[1].clone().with_test_identity_override(
        hostile_rows[1].current_packet_identity(),
        &hostile_rows[1]
            .overlap_identity_digests()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &[],
        &["foreign-independence-proof"],
        "",
        hostile_rows[1].denial_witness_identity(),
        hostile_rows[1].denial_witness_kind(),
    );

    assert_eq!(
        conflict_family_parity_claim_from_catalog(
            &ConflictFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("independence drift must be rejected")
        .kind(),
        ConflictFamilyParityErrorKind::MismatchedIndependenceIdentity
    );
}

#[test]
fn conflict_family_parity_rejects_batch_plan_drift() {
    let mut hostile_rows = current_conflict_family_contributor_catalog()
        .expect("conflict-family catalog")
        .rows()
        .to_vec();
    hostile_rows[2] = hostile_rows[2].clone().with_test_identity_override(
        hostile_rows[2].current_packet_identity(),
        &hostile_rows[2]
            .overlap_identity_digests()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &hostile_rows[2]
            .selected_conflict_plan_digests()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &hostile_rows[2]
            .independence_proof_digests()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "foreign-selected-batch-plan",
        hostile_rows[2].denial_witness_identity(),
        hostile_rows[2].denial_witness_kind(),
    );

    assert_eq!(
        conflict_family_parity_claim_from_catalog(
            &ConflictFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("batch plan drift must be rejected")
        .kind(),
        ConflictFamilyParityErrorKind::MismatchedBatchAdmissionIdentity
    );
}

#[test]
fn conflict_family_parity_rejects_conflict_witness_kind_drift() {
    let mut hostile_rows = current_conflict_family_contributor_catalog()
        .expect("conflict-family catalog")
        .rows()
        .to_vec();
    hostile_rows[0] = hostile_rows[0].clone().with_test_identity_override(
        hostile_rows[0].current_packet_identity(),
        &hostile_rows[0]
            .overlap_identity_digests()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &hostile_rows[0]
            .selected_conflict_plan_digests()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &[],
        "",
        hostile_rows[0].denial_witness_identity(),
        Some(ConflictFamilyDenialWitnessKind::ConflictIndependence(
            ConflictIndependencePlannerRouteWitnessKind::IndependenceDenial,
        )),
    );

    assert_eq!(
        conflict_family_parity_claim_from_catalog(
            &ConflictFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("witness-kind drift must be rejected")
        .kind(),
        ConflictFamilyParityErrorKind::MismatchedConflictIdentity
    );
}

#[test]
fn conflict_family_parity_rejects_batch_witness_contract_drift() {
    let mut hostile_rows = current_conflict_family_contributor_catalog()
        .expect("conflict-family catalog")
        .rows()
        .to_vec();
    hostile_rows[2] = hostile_rows[2].clone().with_test_identity_override(
        hostile_rows[2].current_packet_identity(),
        &hostile_rows[2]
            .overlap_identity_digests()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &hostile_rows[2]
            .selected_conflict_plan_digests()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &hostile_rows[2]
            .independence_proof_digests()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        hostile_rows[2].selected_batch_plan_digest(),
        Some("foreign-batch-witness"),
        Some(ConflictFamilyDenialWitnessKind::BatchAdmission(
            BatchAdmissionPlannerRouteWitnessKind::BatchAdmissionDenial,
        )),
    );

    assert_eq!(
        conflict_family_parity_claim_from_catalog(
            &ConflictFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("missing batch witness contract must be rejected")
        .kind(),
        ConflictFamilyParityErrorKind::MismatchedBatchAdmissionIdentity
    );
}
