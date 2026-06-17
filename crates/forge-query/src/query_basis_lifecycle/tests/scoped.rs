use super::{
    basis_compatibility_debt_registry, scope_certification_basis_intent,
    scope_inspection_basis_intent, scope_mutation_preparation_basis_intent,
    scope_observation_basis_intent, BasisCapabilityAdmission, BasisCompatibilityDebtPosture,
    BasisOperationLaneRequest, BasisScopedAdmissionDenial, RawBasisIntent,
};

#[test]
fn scoped_observation_common_path_preserves_advisory_preview_posture() {
    let scoped = scope_observation_basis_intent(RawBasisIntent::preview(
        super::test_preview_identity("preview:session-1"),
        BasisOperationLaneRequest::Observation,
    ))
    .expect("preview observation should scope through advisory observation");

    match scoped.admission() {
        BasisCapabilityAdmission::Advisory(advisory) => {
            assert_eq!(
                advisory.operation_lane(),
                &BasisOperationLaneRequest::Observation
            );
        }
        other => panic!("unexpected scoped observation admission: {other:?}"),
    }
    assert_eq!(scoped.counters().lane_witness_width(), 1);
}

#[test]
fn scoped_inspection_common_path_accepts_advisory_preview_lane() {
    let scoped = scope_inspection_basis_intent(RawBasisIntent::preview(
        super::test_preview_identity("preview:session-2"),
        BasisOperationLaneRequest::Inspection,
    ))
    .expect("preview inspection should scope through advisory inspection");

    match scoped.admission() {
        BasisCapabilityAdmission::Advisory(_) => {}
        other => panic!("unexpected scoped inspection admission: {other:?}"),
    }
}

#[test]
fn scoped_mutation_preparation_common_path_requires_admitted_capability() {
    let scoped = scope_mutation_preparation_basis_intent(RawBasisIntent::branch_head(
        super::test_branch_identity("branch:main"),
        BasisOperationLaneRequest::MutationPreparation,
    ))
    .expect("branch-head mutation preparation should scope from admitted capability");

    assert_eq!(
        scoped.capability().operation_lane(),
        &BasisOperationLaneRequest::MutationPreparation
    );
    assert_eq!(scoped.counters().admitted_evidence_width(), 1);
}

#[test]
fn scoped_certification_common_path_denies_preview_advisory_capability() {
    let denial = scope_certification_basis_intent(RawBasisIntent::preview_derived_historical(
        super::test_preview_identity("preview:session-3"),
        BasisOperationLaneRequest::Certification,
    ))
    .expect_err("advisory certification capability should not scope into certification use");

    match denial {
        BasisScopedAdmissionDenial::Eligibility(denied) => {
            assert_eq!(
                denied.trace().rule_label(),
                "scoped_use_requires_admitted_capability"
            );
        }
        other => panic!("unexpected scoped certification denial: {other:?}"),
    }
}

#[test]
fn compatibility_debt_registry_names_phase_three_unmigrated_surfaces() {
    let registry = basis_compatibility_debt_registry();

    assert!(registry.rows().iter().any(|row| {
        row.surface_label()
            == "query_context::{bind_query_basis_context,admit_query_basis_context,execute_query_basis_context}"
            && row.posture() == BasisCompatibilityDebtPosture::ScopedMigrationPending
    }));
    assert!(registry.rows().iter().any(|row| {
        row.surface_label()
            == "preview::{assess_preview_live_drift,PreviewLiveExecutionEnvelope::preview_live}"
            && row.posture() == BasisCompatibilityDebtPosture::CompatibilityAdapterPending
    }));
    assert!(registry.rows().iter().any(|row| {
        row.surface_label() == "runtime::inspection::causal::*"
            && row.posture() == BasisCompatibilityDebtPosture::CompatibilityAdapterPending
    }));
}
