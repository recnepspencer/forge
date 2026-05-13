use super::{
    basis_lifecycle, basis_lifecycle_dx_certification_digest, BasisLifecycleIntentBuilder,
};
use crate::basis_lifecycle::{
    BasisFamily, BasisOperationLane, BasisSupportPosture, DeniedBasisCapabilityKind,
    InspectionLaneWitness, LowerRuntimeBasisEvidence,
};

#[test]
fn dx_transcript_admits_current_head_observation_and_materializes_envelope() {
    let envelope = basis_lifecycle()
        .current_head()
        .for_observation()
        .expect("current-head intent should normalize")
        .admit()
        .expect("current-head observation should admit")
        .bind_lower_runtime(LowerRuntimeBasisEvidence::from_runtime_basis(
            "runtime-current-head",
            "dx-runtime-evidence",
            1,
        ))
        .expect("runtime evidence should bind")
        .envelope();

    assert_eq!(envelope.lifecycle().as_str(), "current");
    assert!(!envelope.envelope_digest().is_empty());
    assert!(!envelope.receipt().receipt_digest().is_empty());
}

#[test]
fn dx_transcript_admits_branch_head_mutation_preparation_without_manual_proof_threading() {
    let capability = basis_lifecycle()
        .branch_head("branch-dx", true)
        .for_mutation_preparation()
        .expect("branch-head intent should normalize")
        .admit()
        .expect("branch mutation preparation should admit");

    assert!(!capability.capability_digest().is_empty());
}

#[test]
fn dx_transcript_supports_inspection_advisory_and_support_discovery() {
    let query = BasisLifecycleIntentBuilder;
    let advisory = query
        .preview_derived("preview-dx", "branch-dx")
        .for_inspection_advisory()
        .expect("preview-derived intent should normalize")
        .inspect_advisory()
        .expect("preview-derived inspection should be advisory");
    let support = query.support(
        BasisFamily::PreviewDerived,
        InspectionLaneWitness::lane_name(),
    );

    assert!(!advisory.decision_trace().trace_digest().is_empty());
    assert_eq!(support.posture(), BasisSupportPosture::Advisory);
    assert!(!support.discovery_digest().is_empty());
}

#[test]
fn dx_transcript_keeps_typed_denial_handling_on_the_legal_path() {
    let denial = basis_lifecycle()
        .policy_scoped("policy-dx", "tenant-dx", "branch-dx", "schema-dx")
        .policy_masks_operation()
        .for_observation()
        .expect("policy-scoped intent should normalize")
        .admit()
        .expect_err("policy mask should deny before a capability exists");

    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::PolicyMasked
    );
}

#[test]
fn dx_certification_digest_names_the_transcript_surface() {
    assert!(!basis_lifecycle_dx_certification_digest().is_empty());
}
