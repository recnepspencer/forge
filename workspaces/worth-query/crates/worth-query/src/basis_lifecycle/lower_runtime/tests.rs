use super::{
    readmit_lower_runtime_evidence, LowerRuntimeBasisEvidence, LowerRuntimeEvidenceAuthority,
};
use crate::basis_lifecycle::{
    admit_basis_capability, evaluate_basis_inspection_eligibility,
    evaluate_basis_mutation_preparation_eligibility, evaluate_basis_observation_eligibility,
    normalize_raw_basis_intent, scope_basis_for_inspection, scope_basis_for_mutation_preparation,
    scope_basis_for_observation, BasisOperationLane, DeniedBasisCapabilityKind,
    InspectionLaneWitness, MutationPreparationLaneWitness, ObservationLaneWitness, RawBasisIntent,
};

#[test]
fn runtime_evidence_readmission_binds_matching_current_head_basis() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::CurrentHead,
        <ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("current head should normalize");
    let eligibility =
        evaluate_basis_observation_eligibility(normalized).expect("observation should admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_observation(capability);
    let evidence = LowerRuntimeBasisEvidence::from_runtime_basis(
        "runtime-current-head",
        "runtime-evidence-a",
        3,
    );
    let bound =
        readmit_lower_runtime_evidence(scoped, evidence).expect("runtime evidence should bind");

    assert_eq!(bound.authority(), LowerRuntimeEvidenceAuthority::Runtime);
    assert_eq!(bound.basis_digest(), "runtime-current-head");
    assert!(!bound.lower_runtime_binding_digest().is_empty());
    assert_eq!(bound.counters().lower_runtime_binding_attempt_count(), 1);
    assert_eq!(bound.counters().lower_runtime_readmission_check_count(), 1);
    assert_eq!(bound.counters().retained_evidence_lookup_width(), 3);
}

#[test]
fn bridge_evidence_readmission_binds_matching_preview_basis() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::Preview {
            preview_identity: "preview-a".to_string(),
            stale: false,
        },
        <InspectionLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("preview should normalize");
    let eligibility =
        evaluate_basis_inspection_eligibility(normalized).expect("inspection should admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_inspection(capability);
    let evidence = LowerRuntimeBasisEvidence::from_runtime_bridge_facade(
        "bridge-preview:preview-a",
        "bridge-preview-evidence-a",
        2,
    );
    let bound =
        readmit_lower_runtime_evidence(scoped, evidence).expect("bridge evidence should bind");

    assert_eq!(
        bound.authority(),
        LowerRuntimeEvidenceAuthority::RuntimeBridgeFacade
    );
    assert_eq!(bound.counters().retained_evidence_lookup_width(), 2);
}

#[test]
fn relational_digest_mismatch_denies_before_lower_runtime_bound_basis_exists() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-a".to_string(),
            accessible: true,
        },
        <MutationPreparationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("branch head should normalize");
    let eligibility = evaluate_basis_mutation_preparation_eligibility(normalized)
        .expect("mutation preparation should admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_mutation_preparation(capability);
    let evidence = LowerRuntimeBasisEvidence::from_relational_facade(
        "relational-branch:foreign-branch",
        "relational-evidence-a",
        4,
    );
    let denial = readmit_lower_runtime_evidence(scoped, evidence)
        .expect_err("foreign relational evidence must deny");

    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::RelationalAuthorityMismatch
    );
    assert_eq!(denial.counters().lower_runtime_mismatch_denial_count(), 1);
    assert_eq!(denial.counters().retained_evidence_lookup_width(), 4);
}

#[test]
fn missing_signal_observation_denies_before_readmission_binding() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::CurrentHead,
        <ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("current head should normalize");
    let eligibility =
        evaluate_basis_observation_eligibility(normalized).expect("observation should admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_observation(capability);
    let evidence = LowerRuntimeBasisEvidence::missing_signal_observation("signal-evidence-a");
    let denial = readmit_lower_runtime_evidence(scoped, evidence)
        .expect_err("missing signal basis must deny");

    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::SignalObservationMissing
    );
    assert_eq!(denial.counters().lower_runtime_binding_attempt_count(), 1);
}

#[test]
fn stale_runtime_snapshot_evidence_denies_at_readmission_boundary() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::RuntimeSnapshot {
            snapshot_identity: "snapshot-a".to_string(),
            lower_runtime_binding_digest: Some("bridge-runtime-snapshot:snapshot-a".to_string()),
        },
        <ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("runtime snapshot should normalize");
    let eligibility =
        evaluate_basis_observation_eligibility(normalized).expect("observation should admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_observation(capability);
    let evidence = LowerRuntimeBasisEvidence::stale_runtime_snapshot(
        "bridge-runtime-snapshot:snapshot-a",
        "bridge-runtime-snapshot-evidence-a",
        1,
    );
    let denial =
        readmit_lower_runtime_evidence(scoped, evidence).expect_err("stale snapshot must deny");

    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::RuntimeSnapshotStale
    );
    assert_eq!(denial.counters().retained_evidence_lookup_width(), 1);
}
