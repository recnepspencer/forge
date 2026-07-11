use crate::courtroom::harness::test_support::pre_decode_physical_admission_test_support::{
    crc32c, with_pre_decode_admission,
};
use crate::courtroom::harness::test_support::authenticity_integrity_test_support::{
    admitted_scope, admitted_scope_with_requirement,
};
use forge_store_physical_format::{PhysicalAuthenticityIdentity, PhysicalFrameKind};
use forge_store_physical_integrity::{
    AuthenticityRequiredPhysicalDecodeGate, DeclaredPhysicalChecksum, LogicalDecodeGateIdentity,
    PhysicalIntegrityAdmissionRequest,
};
use forge_store_security::{
    admit_store_authenticity_witness_observation, StoreAuthenticityCheck,
    StoreAuthenticityCheckDenialKind, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreAuthenticityWitnessInput,
    StoreAuthenticityWitnessObservationDeclaration,
};

pub(super) fn assert_authenticity_denial_counter(
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

pub(super) fn assert_checksum_valid_authenticity_gate_denial(
    expected_kind: StoreAuthenticityCheckDenialKind,
) {
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
                forge_store_physical_integrity::PreDecodePhysicalDenialKind::AuthenticityRequiredPhysicalDenial
            );
            assert_eq!(
                physical_denial.authenticity_denial().unwrap().kind(),
                expected_kind
            );
            assert_eq!(physical_denial.counters().checksum_execution_count(), 1);
        },
    );
}

pub(super) fn admitted_witness(
    scope: &forge_store_security::StoreCurrentAuthenticityScopeWitness,
    physical_identity: PhysicalAuthenticityIdentity,
    declaration: StoreAuthenticityWitnessObservationDeclaration,
) -> StoreAuthenticityWitnessInput<PhysicalAuthenticityIdentity> {
    admit_store_authenticity_witness_observation(scope, physical_identity, declaration)
}

pub(super) fn authenticity_physical_identity(
    identity: &LogicalDecodeGateIdentity,
) -> PhysicalAuthenticityIdentity {
    PhysicalAuthenticityIdentity::new(
        identity.header_kind(),
        identity.locality(),
        identity.checked_byte_count(),
        identity.checksum_value(),
        identity.checksum_algorithm(),
    )
}

fn witness_for_expected_denial(
    expected_kind: StoreAuthenticityCheckDenialKind,
    scope: &forge_store_security::StoreCurrentAuthenticityScopeWitness,
    physical_identity: PhysicalAuthenticityIdentity,
) -> StoreAuthenticityWitnessInput<PhysicalAuthenticityIdentity> {
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
