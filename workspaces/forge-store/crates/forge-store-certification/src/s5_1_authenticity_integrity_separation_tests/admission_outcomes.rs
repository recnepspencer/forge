use super::support::{admitted_witness, authenticity_physical_identity};
use crate::courtroom::harness::test_support::pre_decode_physical_admission_test_support::{
    crc32c, with_pre_decode_admission, CountingSemanticDecoder,
};
use crate::courtroom::harness::test_support::authenticity_integrity_test_support::{
    admitted_scope, admitted_scope_with_requirement, policy_lane_observation, PolicyWitnessPosture,
};
use forge_store_physical_format::PhysicalFrameKind;
use forge_store_physical_integrity::{
    AuthenticityRequiredPhysicalDecodeGate, DeclaredPhysicalChecksum,
    PhysicalIntegrityAdmissionRequest, S3LogicalDecoder,
};
use forge_store_security::{
    StoreAuthenticityCheck, StoreAuthenticityCheckDenialKind, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreAuthenticityWitnessObservationDeclaration,
};

#[test]
fn admitted_authenticity_result_preserves_separate_integrity_and_authenticity_counters() {
    let admitted = admitted_scope("store.s51.authenticity.integrity.success", "page-0003");
    let authenticity_scope = admitted.witnesses().authenticity_scope();

    with_pre_decode_admission(
        b"checksum-valid-auth-ok",
        |admission, validation, witness| {
            let checked = admission
                .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                    validation,
                    witness,
                    PhysicalFrameKind::RecordFrame,
                    DeclaredPhysicalChecksum::new(crc32c(b"checksum-valid-auth-ok")),
                ))
                .unwrap();
            let physical_identity =
                authenticity_physical_identity(checked.gate_evidence().identity());
            let result = StoreAuthenticityCheck::for_requirement(authenticity_scope.requirement())
                .with_security_scope(authenticity_scope)
                .with_physical_identity(physical_identity)
                .with_witness(admitted_witness(
                    authenticity_scope,
                    physical_identity,
                    StoreAuthenticityWitnessObservationDeclaration::verified(),
                ))
                .admit();
            let gate =
                AuthenticityRequiredPhysicalDecodeGate::admit_frame(checked, result).unwrap();
            let mut decoder = CountingSemanticDecoder::default();

            decoder.decode(gate.logical_decode_gate());

            assert_eq!(gate.counters().integrity().checksum_execution_count(), 1);
            assert_eq!(gate.counters().authenticity().verified_results(), 1);
            assert_eq!(decoder.invocations, 1);
        },
    );
}

#[test]
fn policy_switch_changes_authenticity_outcome_without_changing_physical_decode_result() {
    let required = admitted_scope_with_requirement(
        "store.s51.authenticity.integrity.policy.required",
        "page-policy",
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
    );
    let not_required = admitted_scope_with_requirement(
        "store.s51.authenticity.integrity.policy.not-required",
        "page-policy",
        StoreAuthenticityRequirement::not_required(),
    );

    let required_observation = policy_lane_observation(
        required.witnesses().authenticity_scope(),
        PolicyWitnessPosture::Verified,
    );
    let not_required_observation = policy_lane_observation(
        not_required.witnesses().authenticity_scope(),
        PolicyWitnessPosture::Absent,
    );

    assert_eq!(
        required_observation.identity,
        not_required_observation.identity
    );
    assert!(required_observation.authenticity_result_present);
    assert!(!not_required_observation.authenticity_result_present);
    assert_eq!(
        not_required_observation.authenticity_denial_kind,
        Some(StoreAuthenticityCheckDenialKind::ResultNotRequired)
    );
    assert_eq!(required_observation.decode_invocations, 1);
    assert_eq!(not_required_observation.decode_invocations, 1);
    assert_eq!(
        required_observation
            .counters
            .integrity()
            .checksum_execution_count(),
        1
    );
    assert_eq!(
        not_required_observation
            .counters
            .integrity()
            .checksum_execution_count(),
        1
    );
    assert_eq!(
        required_observation
            .counters
            .authenticity()
            .witness_observations(),
        1
    );
    assert_eq!(
        required_observation
            .counters
            .authenticity()
            .verified_results(),
        1
    );
    assert_eq!(
        not_required_observation
            .counters
            .authenticity()
            .requirement_checks(),
        1
    );
    assert_eq!(
        not_required_observation
            .counters
            .authenticity()
            .witness_observations(),
        0
    );
}
