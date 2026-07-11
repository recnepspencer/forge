use super::support::{
    admitted_witness, assert_authenticity_denial_counter,
    assert_checksum_valid_authenticity_gate_denial, authenticity_physical_identity,
};
use crate::courtroom::harness::test_support::pre_decode_physical_admission_test_support::{
    crc32c, with_pre_decode_admission, CountingSemanticDecoder,
};
use crate::courtroom::harness::test_support::s5_1_authenticity_integrity_test_support::admitted_scope;
use forge_store_physical_format::PhysicalFrameKind;
use forge_store_physical_integrity::{
    AuthenticityRequiredPhysicalDecodeGate, DeclaredPhysicalChecksum,
    PhysicalIntegrityAdmissionRequest, PreDecodePhysicalDenialKind,
};
use forge_store_security::{
    StoreAuthenticityCheck, StoreAuthenticityCheckDenialKind, StoreAuthenticityWitnessInput,
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
