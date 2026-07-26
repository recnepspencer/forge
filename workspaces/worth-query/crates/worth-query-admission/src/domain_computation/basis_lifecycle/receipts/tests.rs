use super::{
    emit_inspection_basis_receipt, emit_mutation_preparation_basis_receipt,
    emit_observation_basis_receipt, envelope_basis_use, BasisNextTransition, BasisUseReceiptKind,
};
use crate::domain_computation::{
    admit_basis_capability, evaluate_basis_inspection_eligibility,
    evaluate_basis_mutation_preparation_eligibility, evaluate_basis_observation_eligibility,
    normalize_raw_basis_intent, readmit_lower_runtime_evidence, scope_basis_for_inspection,
    scope_basis_for_mutation_preparation, scope_basis_for_observation, BasisLifecyclePosture,
    BasisOperationLane, InspectionLaneWitness, LowerRuntimeBasisEvidence,
    MutationPreparationLaneWitness, ObservationLaneWitness, RawBasisIntent,
};

#[test]
fn observation_bound_basis_emits_receipt_and_self_describing_envelope() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::CurrentHead,
        <ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("current head should normalize");
    let eligibility =
        evaluate_basis_observation_eligibility(normalized).expect("observation should admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_observation(capability);
    let bound = readmit_lower_runtime_evidence(
        scoped,
        LowerRuntimeBasisEvidence::from_runtime_basis(
            "runtime-current-head",
            "runtime-evidence-a",
            3,
        ),
    )
    .expect("runtime evidence should bind");
    let receipt = emit_observation_basis_receipt(bound);

    assert_eq!(receipt.kind(), BasisUseReceiptKind::Observation);
    assert_eq!(receipt.lifecycle(), BasisLifecyclePosture::Current);
    assert_eq!(receipt.lower_runtime_basis_digest(), "runtime-current-head");
    assert!(!receipt.readmission_trace_digest().is_empty());
    assert!(!receipt.receipt_digest().is_empty());
    assert_eq!(receipt.counters().basis_receipt_emission_count(), 1);
    assert_eq!(receipt.counters().retained_evidence_lookup_width(), 3);
    assert!(receipt
        .permitted_next_transitions()
        .contains(&BasisNextTransition::LaterInspection));

    let envelope = envelope_basis_use(receipt);
    assert_eq!(envelope.lifecycle(), BasisLifecyclePosture::Current);
    assert!(!envelope.readmission_trace_digest().is_empty());
    assert!(!envelope.envelope_digest().is_empty());
    assert!(!envelope.integrity_digest().is_empty());
    assert!(!envelope.support_matrix_digest().is_empty());
    assert_eq!(
        envelope.counters().basis_envelope_materialization_count(),
        1
    );
    assert_eq!(envelope.structured_warnings().len(), 1);
}

#[test]
fn mutation_preparation_receipt_permits_effect_plan_without_projection_shortcut() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-a".to_string(),
            accessible: true,
        },
        <MutationPreparationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("branch mutation should normalize");
    let eligibility = evaluate_basis_mutation_preparation_eligibility(normalized)
        .expect("mutation preparation should admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_mutation_preparation(capability);
    let bound = readmit_lower_runtime_evidence(
        scoped,
        LowerRuntimeBasisEvidence::from_relational_facade(
            "relational-branch:branch-a",
            "relational-evidence-a",
            1,
        ),
    )
    .expect("relational evidence should bind");
    let receipt = emit_mutation_preparation_basis_receipt(bound);

    assert_eq!(receipt.kind(), BasisUseReceiptKind::MutationPreparation);
    assert!(receipt
        .permitted_next_transitions()
        .contains(&BasisNextTransition::EffectPlan));
    assert!(!receipt
        .permitted_next_transitions()
        .contains(&BasisNextTransition::ProjectionConsumption));
}

#[test]
fn inspection_envelope_has_no_deferred_warning_for_materialization_path() {
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
    let bound = readmit_lower_runtime_evidence(
        scoped,
        LowerRuntimeBasisEvidence::from_runtime_bridge_facade(
            "bridge-preview:preview-a",
            "bridge-preview-evidence-a",
            2,
        ),
    )
    .expect("bridge preview evidence should bind");
    let receipt = emit_inspection_basis_receipt(bound);
    let envelope = envelope_basis_use(receipt);

    assert!(envelope
        .receipt()
        .permitted_next_transitions()
        .contains(&BasisNextTransition::Materialization));
    assert!(envelope.structured_warnings().is_empty());
}
