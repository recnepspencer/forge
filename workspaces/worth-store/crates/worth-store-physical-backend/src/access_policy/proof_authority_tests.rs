use super::*;
use crate::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilityKind,
    BackendCapabilitySupportPosture, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_physical_format::{
    PhysicalAlignmentClass, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
};
use worth_store_security::admitted_store_internal_security_scope_for_io_qos_test;

use super::test_support::backend_with_assumptions;

#[test]
fn direct_io_alignment_proof_requires_physical_format_page_and_extent_alignment() {
    let backend = backend_with_all_access_modes();
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);
    let reference = test_reference();
    let lifecycle = pinned_lifecycle();

    assert!(proof
        .direct_io_page_and_sector_aligned(
            reference,
            lifecycle,
            4096,
            PhysicalAlignmentClass::frame_start_8(),
            PhysicalAlignmentClass::extent_start_4k(),
        )
        .is_none());
    assert!(proof
        .direct_io_page_and_sector_aligned(
            reference,
            lifecycle,
            4096,
            PhysicalAlignmentClass::page_start_4k(),
            PhysicalAlignmentClass::manifest_record_8(),
        )
        .is_none());
}

#[test]
fn direct_io_alignment_proof_must_match_request_reference_and_lifecycle() {
    let backend = backend_with_all_access_modes();
    let mismatched = direct_io_alignment(&backend, other_reference(), pinned_lifecycle(), 4096);
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(base_request(
            &backend,
            AccessPolicyRequest::direct_io_read().with_alignment_requirement(mismatched),
        ))
        .expect_err("alignment proof is bound to the request reference");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::DirectIoAlignmentRequired
    );
}

#[test]
fn mmap_posture_requires_exact_admitted_media_axes() {
    let backend = admitted_backend(false, true);
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);

    assert!(proof.mmap_posture().is_none());
}

#[test]
fn mmap_posture_rejects_each_missing_media_axis() {
    for assumptions in mmap_assumptions_missing_one_axis() {
        let backend = backend_with_assumptions(assumptions);
        let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);

        assert!(proof.mmap_posture().is_none());
    }
}

#[test]
fn mixed_coherence_requires_exact_admitted_media_axes() {
    let backend = admitted_backend(true, false);
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);
    let transition =
        MixedAccessTransition::new(StoreAccessMode::DirectIo, StoreAccessMode::Buffered);

    assert!(proof
        .mixed_coherence(transition, test_reference(), test_security_scope())
        .is_none());
}

#[test]
fn mixed_mmap_to_buffered_requires_mmap_fault_posture() {
    let backend = backend_with_all_access_modes();
    let transition = MixedAccessTransition::new(StoreAccessMode::Mmap, StoreAccessMode::Buffered);
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(
            base_request(&backend, AccessPolicyRequest::mixed_read(transition))
                .with_coherence_basis(admitted_mixed_basis(&backend, transition)),
        )
        .expect_err("mmap-origin mixed access requires mmap fault/writeback posture");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::MmapFaultPostureUnsupported
    );
}

fn mmap_assumptions_missing_one_axis() -> [BackendMediaAssumptionSet; 5] {
    let base = BackendMediaAssumptionSet::platform_file_defaults()
        .with_page_cache_policy()
        .with_mmap_mapping_coherence();
    [
        base.with_mmap_store_tracked_writeback()
            .with_mmap_shared_visibility()
            .with_mmap_typed_truncate()
            .with_mmap_typed_punch_hole(),
        base.with_mmap_typed_faults()
            .with_mmap_shared_visibility()
            .with_mmap_typed_truncate()
            .with_mmap_typed_punch_hole(),
        base.with_mmap_typed_faults()
            .with_mmap_store_tracked_writeback()
            .with_mmap_typed_truncate()
            .with_mmap_typed_punch_hole(),
        base.with_mmap_typed_faults()
            .with_mmap_store_tracked_writeback()
            .with_mmap_shared_visibility()
            .with_mmap_typed_punch_hole(),
        base.with_mmap_typed_faults()
            .with_mmap_store_tracked_writeback()
            .with_mmap_shared_visibility()
            .with_mmap_typed_truncate(),
    ]
}

#[test]
fn mixed_mmap_to_buffered_requires_mmap_backend_capability() {
    let backend = admitted_backend_with_mmap_posture(BackendCapabilitySupportPosture::Unknown);
    let transition = MixedAccessTransition::new(StoreAccessMode::Mmap, StoreAccessMode::Buffered);
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(
            base_request(&backend, AccessPolicyRequest::mixed_read(transition))
                .with_mmap_fault_posture(admitted_mmap_posture(&backend))
                .with_coherence_basis(admitted_mixed_basis(&backend, transition)),
        )
        .expect_err("mmap-origin mixed access requires admitted mmap capability");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::BackendCapabilityDenied
    );
    assert!(denial.backend_denial().is_some());
}

#[test]
fn mixed_direct_to_buffered_requires_direct_io_alignment() {
    let backend = backend_with_all_access_modes();
    let transition =
        MixedAccessTransition::new(StoreAccessMode::DirectIo, StoreAccessMode::Buffered);
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(
            base_request(&backend, AccessPolicyRequest::mixed_read(transition))
                .with_coherence_basis(admitted_mixed_basis(&backend, transition)),
        )
        .expect_err("direct-origin mixed access requires direct-I/O alignment proof");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::DirectIoAlignmentRequired
    );
}

#[test]
fn mixed_transition_rejects_mixed_meta_mode_as_participant() {
    let backend = backend_with_all_access_modes();
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);
    let transition = MixedAccessTransition::new(StoreAccessMode::Mixed, StoreAccessMode::Buffered);

    assert!(proof
        .mixed_coherence(transition, test_reference(), test_security_scope())
        .is_none());

    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(base_request(
            &backend,
            AccessPolicyRequest::mixed_read(transition),
        ))
        .expect_err("mixed meta-mode is not a physical transition participant");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::InvalidMixedAccessTransition
    );
}

#[test]
fn mixed_direct_participant_dirty_mmap_page_gets_typed_denial() {
    let backend = backend_with_all_access_modes();
    let transition =
        MixedAccessTransition::new(StoreAccessMode::Buffered, StoreAccessMode::DirectIo);
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(
            base_request(&backend, AccessPolicyRequest::mixed_read(transition))
                .with_buffer_lifecycle(
                    AccessPolicyBufferLifecycle::for_certification_dirty_mmap_page(),
                ),
        )
        .expect_err("dirty mmap state blocks direct-I/O even inside mixed access");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::DirtyMmapPageBlocksDirectIo
    );
}

fn base_request(
    backend: &crate::AdmittedBackendCapabilityWitness,
    request: AccessPolicyRequest,
) -> AccessPolicyRequest {
    request
        .for_physical_reference(test_reference())
        .with_security_scope(test_security_scope())
        .with_buffer_lifecycle(pinned_lifecycle())
        .with_page_cache_policy(page_cache_policy(backend))
}

fn direct_io_alignment(
    backend: &crate::AdmittedBackendCapabilityWitness,
    reference: PhysicalReference,
    lifecycle: AccessPolicyBufferLifecycle,
    byte_length: u32,
) -> DirectIoAlignmentRequirement {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .direct_io_page_and_sector_aligned(
            reference,
            lifecycle,
            byte_length,
            PhysicalAlignmentClass::page_start_4k(),
            PhysicalAlignmentClass::extent_start_4k(),
        )
        .expect("test backend carries direct I/O alignment assumptions")
}

fn pinned_lifecycle() -> AccessPolicyBufferLifecycle {
    AccessPolicyBufferLifecycle::for_certification_pinned_physical_substrate_lease()
}

fn page_cache_policy(backend: &crate::AdmittedBackendCapabilityWitness) -> PageCachePolicyProof {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .page_cache_policy()
        .expect("test backend carries Store-admitted page-cache posture")
}

fn admitted_mmap_posture(backend: &crate::AdmittedBackendCapabilityWitness) -> MmapFaultPosture {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .mmap_posture()
        .expect("test backend carries mmap capability posture")
}

fn admitted_mixed_basis(
    backend: &crate::AdmittedBackendCapabilityWitness,
    transition: MixedAccessTransition,
) -> MixedAccessCoherenceBasis {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .mixed_coherence(transition, test_reference(), test_security_scope())
        .expect("test backend carries mixed-mode coherence posture")
}

fn backend_with_all_access_modes() -> crate::AdmittedBackendCapabilityWitness {
    admitted_backend_with_mmap_posture(BackendCapabilitySupportPosture::Supported)
}

fn admitted_backend(
    include_mmap_access_policy: bool,
    include_mixed_coherence: bool,
) -> crate::AdmittedBackendCapabilityWitness {
    let support = BackendCapabilitySupportSet::all_supported().with_posture(
        BackendCapabilityKind::DirectIo,
        BackendCapabilitySupportPosture::Supported,
    );
    let mut assumptions = BackendMediaAssumptionSet::platform_file_defaults()
        .with_direct_io_alignment()
        .with_sector_atomicity()
        .with_page_cache_policy();
    if include_mmap_access_policy {
        assumptions = assumptions.with_mmap_coherence();
    }
    if include_mixed_coherence {
        assumptions = assumptions.with_mixed_access_coherence();
    }
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            support,
            assumptions,
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
                .with_sector_alignment()
                .with_security_posture(),
        ))
        .expect("test backend capability admits")
}

fn admitted_backend_with_mmap_posture(
    posture: BackendCapabilitySupportPosture,
) -> crate::AdmittedBackendCapabilityWitness {
    let support = BackendCapabilitySupportSet::all_supported()
        .with_posture(
            BackendCapabilityKind::DirectIo,
            BackendCapabilitySupportPosture::Supported,
        )
        .with_posture(BackendCapabilityKind::Mmap, posture);
    let assumptions = BackendMediaAssumptionSet::platform_file_defaults()
        .with_direct_io_alignment()
        .with_sector_atomicity()
        .with_page_cache_policy()
        .with_mmap_coherence()
        .with_mixed_access_coherence();
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            support,
            assumptions,
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
                .with_sector_alignment()
                .with_security_posture(),
        ))
        .expect("test backend capability admits")
}

fn test_security_scope() -> AccessPolicySecurityScope {
    let admitted = admitted_store_internal_security_scope_for_io_qos_test();
    AccessPolicySecurityScope::from_current_store_scope(admitted.witnesses())
}

fn test_reference() -> PhysicalReference {
    test_reference_for_segment(1)
}

fn other_reference() -> PhysicalReference {
    test_reference_for_segment(2)
}

fn test_reference_for_segment(segment: u64) -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(segment).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(cell)
        .reference()
}
