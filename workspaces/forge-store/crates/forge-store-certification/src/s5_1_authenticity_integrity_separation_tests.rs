use crate::courtroom::harness::test_support::pre_decode_physical_admission_test_support::{
    crc32c, with_pre_decode_admission, CountingSemanticDecoder,
};
use crate::courtroom::harness::test_support::s5_1_authenticity_integrity_test_support::{
    admitted_scope, admitted_scope_with_requirement, policy_lane_observation, PolicyWitnessPosture,
};
use forge_store_physical_format::PhysicalFrameKind;
use forge_store_physical_integrity::{
    AuthenticityRequiredPhysicalDecodeGate, DeclaredPhysicalChecksum, LogicalDecodeGateIdentity,
    PhysicalIntegrityAdmissionRequest, PreDecodePhysicalDenialKind, S3LogicalDecoder,
};
use forge_store_security::{
    admit_store_authenticity_witness_observation, StoreAuthenticityCheck,
    StoreAuthenticityCheckDenialKind, StoreAuthenticityPhysicalIdentity,
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreAuthenticityWitnessInput,
    StoreAuthenticityWitnessObservationDeclaration,
};

#[test]
fn checksum_valid_bytes_cannot_reach_decode_without_authenticity_result() {
    let admitted = admitted_scope("store.s51.authenticity.integrity.absent", "page-0001");
    let authenticity_scope = admitted.witnesses().authenticity_scope();
    let decoder = CountingSemanticDecoder::default();

    with_pre_decode_admission(
        b"checksum-valid-auth-absent",
        |admission, validation, witness| {
            let checked = admission
                .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                    validation,
                    witness,
                    PhysicalFrameKind::RecordFrame,
                    DeclaredPhysicalChecksum::new(crc32c(b"checksum-valid-auth-absent")),
                ))
                .unwrap();
            let physical_identity =
                authenticity_physical_identity(checked.gate_evidence().identity());
            let denial = StoreAuthenticityCheck::for_requirement(authenticity_scope.requirement())
                .with_security_scope(authenticity_scope)
                .with_physical_identity(physical_identity)
                .with_witness(StoreAuthenticityWitnessInput::absent())
                .admit()
                .unwrap_err();

            let physical_denial =
                AuthenticityRequiredPhysicalDecodeGate::admit_frame(checked, Err(denial))
                    .unwrap_err();

            assert_eq!(
                physical_denial.kind(),
                PreDecodePhysicalDenialKind::AuthenticityRequiredPhysicalDenial
            );
            assert_eq!(
                physical_denial.authenticity_denial().unwrap().kind(),
                StoreAuthenticityCheckDenialKind::MissingWitness
            );
            assert_eq!(physical_denial.counters().checksum_execution_count(), 1);
        },
    );

    assert_eq!(decoder.invocations, 0);
    assert_eq!(decoder.semantic_index_lookups, 0);
    assert_eq!(decoder.domain_constructors, 0);
}

#[test]
fn checksum_valid_authenticity_failed_unavailable_and_unsupported_are_counted_separately() {
    assert_authenticity_denial_counter(StoreAuthenticityCheckDenialKind::Failed, (1, 0, 0));
    assert_authenticity_denial_counter(StoreAuthenticityCheckDenialKind::Unavailable, (0, 1, 0));
    assert_authenticity_denial_counter(StoreAuthenticityCheckDenialKind::Unsupported, (0, 0, 1));
}

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

#[test]
fn checksum_valid_stale_and_wrong_scope_authenticity_are_gate_denials() {
    assert_checksum_valid_authenticity_gate_denial(StoreAuthenticityCheckDenialKind::StaleWitness);
    assert_checksum_valid_authenticity_gate_denial(StoreAuthenticityCheckDenialKind::WrongScope);
}

#[test]
fn verified_authenticity_result_cannot_be_replayed_to_another_checked_frame() {
    let admitted = admitted_scope("store.s51.authenticity.integrity.replay", "page-0004");
    let authenticity_scope = admitted.witnesses().authenticity_scope();

    with_pre_decode_admission(
        b"checksum-valid-auth-source",
        |admission, validation, witness| {
            let checked = admission
                .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                    validation,
                    witness,
                    PhysicalFrameKind::RecordFrame,
                    DeclaredPhysicalChecksum::new(crc32c(b"checksum-valid-auth-source")),
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
                .admit()
                .unwrap();

            let gate = AuthenticityRequiredPhysicalDecodeGate::admit_frame(checked, Ok(result))
                .expect("source identity admits");
            assert_eq!(
                gate.authenticity_result().physical_identity(),
                physical_identity
            );

            with_pre_decode_admission(
                b"checksum-valid-auth-target",
                |target_admission, target_validation, target_witness| {
                    let target_checked = target_admission
                        .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                            target_validation,
                            target_witness,
                            PhysicalFrameKind::RecordFrame,
                            DeclaredPhysicalChecksum::new(crc32c(b"checksum-valid-auth-target")),
                        ))
                        .unwrap();
                    let physical_denial = AuthenticityRequiredPhysicalDecodeGate::admit_frame(
                        target_checked,
                        Ok(result),
                    )
                    .unwrap_err();

                    assert_eq!(
                        physical_denial.kind(),
                        PreDecodePhysicalDenialKind::AuthenticityResultPhysicalIdentityMismatch
                    );
                    assert!(physical_denial.authenticity_denial().is_none());
                },
            );
        },
    );
}

fn assert_authenticity_denial_counter(
    expected_kind: StoreAuthenticityCheckDenialKind,
    expected: (u32, u32, u32),
) {
    let admitted = admitted_scope("store.s51.authenticity.integrity.counter", "page-counter");
    let authenticity_scope = admitted.witnesses().authenticity_scope();
    with_pre_decode_admission(
        b"checksum-valid-auth-denied",
        |admission, validation, witness_head| {
            let checked = admission
                .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                    validation,
                    witness_head,
                    PhysicalFrameKind::RecordFrame,
                    DeclaredPhysicalChecksum::new(crc32c(b"checksum-valid-auth-denied")),
                ))
                .unwrap();
            let physical_identity =
                authenticity_physical_identity(checked.gate_evidence().identity());
            let denial = StoreAuthenticityCheck::for_requirement(authenticity_scope.requirement())
                .with_security_scope(authenticity_scope)
                .with_physical_identity(physical_identity)
                .with_witness(witness_for_expected_denial(
                    expected_kind,
                    authenticity_scope,
                    physical_identity,
                ))
                .admit()
                .unwrap_err();
            let physical_denial =
                AuthenticityRequiredPhysicalDecodeGate::admit_frame(checked, Err(denial))
                    .unwrap_err();
            let counters = physical_denial.authenticity_required_counters().unwrap();

            assert_eq!(
                physical_denial.authenticity_denial().unwrap().kind(),
                expected_kind
            );
            assert_eq!(counters.checksum_valid_authenticity_failed(), expected.0);
            assert_eq!(
                counters.checksum_valid_authenticity_unavailable(),
                expected.1
            );
            assert_eq!(
                counters.checksum_valid_authenticity_unsupported(),
                expected.2
            );
        },
    );
}

fn witness_for_expected_denial(
    expected_kind: StoreAuthenticityCheckDenialKind,
    scope: &forge_store_security::StoreCurrentAuthenticityScopeWitness,
    physical_identity: StoreAuthenticityPhysicalIdentity,
) -> StoreAuthenticityWitnessInput {
    match expected_kind {
        StoreAuthenticityCheckDenialKind::Failed => admitted_witness(
            scope,
            physical_identity,
            StoreAuthenticityWitnessObservationDeclaration::failed(),
        ),
        StoreAuthenticityCheckDenialKind::Unavailable => {
            StoreAuthenticityWitnessInput::unavailable()
        }
        StoreAuthenticityCheckDenialKind::Unsupported => {
            StoreAuthenticityWitnessInput::unsupported()
        }
        _ => panic!("test helper covers only failed, unavailable, and unsupported"),
    }
}

fn assert_checksum_valid_authenticity_gate_denial(expected_kind: StoreAuthenticityCheckDenialKind) {
    let admitted = admitted_scope("store.s51.authenticity.integrity.drift", "page-drift");
    let wrong_scope = admitted_scope_with_requirement(
        "store.s51.authenticity.integrity.other",
        "page-other",
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedManifest,
        ),
    );
    let authenticity_scope = admitted.witnesses().authenticity_scope();
    let wrong_authenticity_scope = wrong_scope.witnesses().authenticity_scope();

    with_pre_decode_admission(
        b"checksum-valid-auth-stale-or-wrong",
        |admission, validation, witness| {
            let checked = admission
                .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                    validation,
                    witness,
                    PhysicalFrameKind::RecordFrame,
                    DeclaredPhysicalChecksum::new(crc32c(b"checksum-valid-auth-stale-or-wrong")),
                ))
                .unwrap();
            let physical_identity =
                authenticity_physical_identity(checked.gate_evidence().identity());
            let witness = match expected_kind {
                StoreAuthenticityCheckDenialKind::StaleWitness => admitted_witness(
                    authenticity_scope,
                    physical_identity,
                    StoreAuthenticityWitnessObservationDeclaration::stale(),
                ),
                StoreAuthenticityCheckDenialKind::WrongScope => admitted_witness(
                    wrong_authenticity_scope,
                    physical_identity,
                    StoreAuthenticityWitnessObservationDeclaration::verified(),
                ),
                _ => panic!("test helper covers stale and wrong-scope denials"),
            };
            let denial = StoreAuthenticityCheck::for_requirement(authenticity_scope.requirement())
                .with_security_scope(authenticity_scope)
                .with_physical_identity(physical_identity)
                .with_witness(witness)
                .admit()
                .unwrap_err();
            let physical_denial =
                AuthenticityRequiredPhysicalDecodeGate::admit_frame(checked, Err(denial))
                    .unwrap_err();

            assert_eq!(
                physical_denial.kind(),
                PreDecodePhysicalDenialKind::AuthenticityRequiredPhysicalDenial
            );
            assert_eq!(
                physical_denial.authenticity_denial().unwrap().kind(),
                expected_kind
            );
            assert_eq!(physical_denial.counters().checksum_execution_count(), 1);
        },
    );
}

fn admitted_witness(
    scope: &forge_store_security::StoreCurrentAuthenticityScopeWitness,
    physical_identity: StoreAuthenticityPhysicalIdentity,
    declaration: StoreAuthenticityWitnessObservationDeclaration,
) -> StoreAuthenticityWitnessInput {
    admit_store_authenticity_witness_observation(scope, physical_identity, declaration)
}

fn authenticity_physical_identity(
    identity: &LogicalDecodeGateIdentity,
) -> StoreAuthenticityPhysicalIdentity {
    StoreAuthenticityPhysicalIdentity::new(
        identity.header_kind(),
        identity.locality(),
        identity.checked_byte_count(),
        identity.checksum_value(),
        identity.checksum_algorithm(),
    )
}
