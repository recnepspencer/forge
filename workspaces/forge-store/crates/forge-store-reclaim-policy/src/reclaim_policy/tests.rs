use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReclaimRegion,
    PhysicalRecordSlot, PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
    ReclaimedByteInterpretation,
};
use forge_store_security::admitted_store_internal_security_scope_for_s6_test;

use super::*;
use crate::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};

#[test]
fn distinct_reclaim_postures_preserve_byte_interpretation() {
    let backend = backend_with_reclaim_posture();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);

    assert_eq!(
        authority
            .trim_posture(ReclaimedByteInterpretation::PhysicalZeros)
            .unwrap()
            .interpretation(),
        ReclaimedByteInterpretation::PhysicalZeros
    );
    assert_eq!(
        authority
            .punch_hole_posture(ReclaimedByteInterpretation::LogicalHole)
            .unwrap()
            .interpretation(),
        ReclaimedByteInterpretation::LogicalHole
    );
    assert_eq!(
        authority
            .sparse_posture(ReclaimedByteInterpretation::UnavailableBytes)
            .unwrap()
            .interpretation(),
        ReclaimedByteInterpretation::UnavailableBytes
    );
    assert_eq!(
        authority
            .cold_tier_io_posture(ReclaimedByteInterpretation::NonObservableReclaimedStorage)
            .unwrap()
            .interpretation(),
        ReclaimedByteInterpretation::NonObservableReclaimedStorage
    );
}

#[test]
fn admission_denies_missing_or_blocked_reachability_before_execution() {
    let backend = backend_with_reclaim_posture();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let request = base_request(&backend);
    let denial = ReclaimPolicyAdmission::admit(authority, request).unwrap_err();
    assert_eq!(
        denial.kind(),
        &ReclaimPolicyDenialKind::MissingProtectedReachability
    );

    let blocked = base_request(&backend).with_reachability(
        ReclaimPolicyReachabilityProof::blocked_for_certification_test_authority(test_region()),
    );
    let denial = ReclaimPolicyAdmission::admit(authority, blocked).unwrap_err();
    assert_eq!(
        denial.kind(),
        &ReclaimPolicyDenialKind::ProtectedReachabilityBlocked
    );
}

#[test]
fn admission_denies_later_lifecycle_claims() {
    let backend = backend_with_reclaim_posture();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let request = base_request(&backend)
        .with_reachability(
            ReclaimPolicyReachabilityProof::for_certification_test_authority(test_region()),
        )
        .with_later_handoff_policy(ReclaimLaterHandoffPolicy::claims_later_lifecycle_for_denial());

    let denial = ReclaimPolicyAdmission::admit(authority, request).unwrap_err();
    assert_eq!(
        denial.kind(),
        &ReclaimPolicyDenialKind::LaterLifecycleClaimAttempted
    );
}

#[test]
fn execution_observes_success_and_typed_byte_contradiction() {
    let backend = backend_with_reclaim_posture();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let policy = ReclaimPolicyAdmission::admit(
        authority,
        base_request(&backend).with_reachability(
            ReclaimPolicyReachabilityProof::for_certification_test_authority(test_region()),
        ),
    )
    .unwrap();
    let security_scope = ReclaimPolicySecurityScope::from_admitted_scope(
        &admitted_store_internal_security_scope_for_s6_test(),
    );

    let receipt = policy
        .clone()
        .complete_execution_with_store_authority(ReclaimPolicyExecutionObservation::new(
            test_region(),
            ReclaimedByteInterpretation::LogicalHole,
            security_scope,
            true,
        ))
        .unwrap();
    assert_eq!(
        receipt.observed_interpretation(),
        ReclaimedByteInterpretation::LogicalHole
    );
    assert_eq!(receipt.counters().executed(), 1);

    let violation = policy
        .complete_execution_with_store_authority(ReclaimPolicyExecutionObservation::new(
            test_region(),
            ReclaimedByteInterpretation::PhysicalZeros,
            security_scope,
            true,
        ))
        .unwrap_err();
    assert!(matches!(
        violation.kind(),
        ReclaimPolicyViolationKind::ByteInterpretationContradicted { .. }
    ));
}

#[test]
fn admission_denies_posture_from_different_backend_authority() {
    let supported = backend_with_reclaim_posture();
    let unsupported = backend_without_reclaim_posture();
    let supported_authority = ReclaimPolicyProofAuthority::for_admitted_backend(&supported);
    let unsupported_authority = ReclaimPolicyProofAuthority::for_admitted_backend(&unsupported);
    let request = ReclaimPolicyRequest::new()
        .for_region(test_region())
        .with_posture(
            supported_authority
                .punch_hole_posture(ReclaimedByteInterpretation::LogicalHole)
                .unwrap(),
        )
        .with_reachability(
            ReclaimPolicyReachabilityProof::for_certification_test_authority(test_region()),
        )
        .with_security_scope(ReclaimPolicySecurityScope::from_admitted_scope(
            &admitted_store_internal_security_scope_for_s6_test(),
        ))
        .with_reclaim_permit(ReclaimPermit::new(1).unwrap())
        .with_later_handoff_policy(unsupported_authority.non_claim_later_handoff());

    let denial = ReclaimPolicyAdmission::admit(unsupported_authority, request).unwrap_err();
    assert_eq!(
        denial.kind(),
        &ReclaimPolicyDenialKind::UnsupportedBackendPosture
    );
}

#[test]
fn execution_observes_region_and_security_scope_violations() {
    let backend = backend_with_reclaim_posture();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let policy = ReclaimPolicyAdmission::admit(
        authority,
        base_request(&backend).with_reachability(
            ReclaimPolicyReachabilityProof::for_certification_test_authority(test_region()),
        ),
    )
    .unwrap();
    let security_scope = ReclaimPolicySecurityScope::from_admitted_scope(
        &admitted_store_internal_security_scope_for_s6_test(),
    );

    let wrong_region =
        PhysicalReclaimRegion::new(test_reference_with_generation(99), 4096).unwrap();
    let violation = policy
        .clone()
        .complete_execution_with_store_authority(ReclaimPolicyExecutionObservation::new(
            wrong_region,
            ReclaimedByteInterpretation::LogicalHole,
            security_scope,
            true,
        ))
        .unwrap_err();
    assert_eq!(
        violation.kind(),
        ReclaimPolicyViolationKind::ProtectedReachabilityLost
    );

    let wrong_scope = ReclaimPolicySecurityScope::from_admitted_scope(
        &forge_store_security::admitted_wrong_s6_io_qos_security_scope_for_test(),
    );
    let violation = policy
        .complete_execution_with_store_authority(ReclaimPolicyExecutionObservation::new(
            test_region(),
            ReclaimedByteInterpretation::LogicalHole,
            wrong_scope,
            true,
        ))
        .unwrap_err();
    assert_eq!(
        violation.kind(),
        ReclaimPolicyViolationKind::SecurityScopeLost
    );
}

fn backend_with_reclaim_posture() -> crate::AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_trim_posture()
                .with_punch_hole_posture()
                .with_sparse_posture()
                .with_cold_tier_io_posture(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}

fn backend_without_reclaim_posture() -> crate::AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}

fn base_request(backend: &crate::AdmittedBackendCapabilityWitness) -> ReclaimPolicyRequest {
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(backend);
    ReclaimPolicyRequest::new()
        .for_region(test_region())
        .with_posture(
            authority
                .punch_hole_posture(ReclaimedByteInterpretation::LogicalHole)
                .unwrap(),
        )
        .with_security_scope(ReclaimPolicySecurityScope::from_admitted_scope(
            &admitted_store_internal_security_scope_for_s6_test(),
        ))
        .with_reclaim_permit(ReclaimPermit::new(1).unwrap())
        .with_later_handoff_policy(authority.non_claim_later_handoff())
}

fn test_reference() -> PhysicalReference {
    test_reference_with_generation(1)
}

fn test_region() -> PhysicalReclaimRegion {
    PhysicalReclaimRegion::new(test_reference(), 4096).unwrap()
}

fn test_reference_with_generation(generation: u64) -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(generation).unwrap());
    PhysicalReferenceAuthority::s1()
        .admit_page_slot(cell)
        .reference()
}
