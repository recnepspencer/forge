use super::{
    AccessPolicyBufferLifecycle, AccessPolicyRequest, AccessPolicySecurityScope,
    DirectIoAlignmentRequirement, MixedAccessCoherenceBasis, MixedAccessTransition,
    MmapFaultPosture, PageCachePolicyProof, StoreAccessPolicyProofAuthority,
};
use crate::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
    PhysicalReference,
};
use forge_store_physical_format::{
    PhysicalAlignmentClass, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId,
};
use forge_store_security::admitted_store_internal_security_scope_for_io_qos_test;

pub fn backend_with_all_access_modes() -> AdmittedBackendCapabilityWitness {
    backend_with_assumptions(
        BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_sector_atomicity()
            .with_page_cache_policy()
            .with_mmap_coherence()
            .with_mixed_access_coherence(),
    )
}

pub fn backend_with_assumptions(
    assumptions: BackendMediaAssumptionSet,
) -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            assumptions,
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
                .with_sector_alignment()
                .with_security_posture(),
        ))
        .expect("backend admits")
}

pub fn base_request(
    backend: &AdmittedBackendCapabilityWitness,
    request: AccessPolicyRequest,
) -> AccessPolicyRequest {
    request
        .for_physical_reference(test_reference())
        .with_security_scope(test_security_scope())
        .with_buffer_lifecycle(pinned_lifecycle())
        .with_page_cache_policy(page_cache_policy(backend))
}

pub fn direct_io_request(backend: &AdmittedBackendCapabilityWitness) -> AccessPolicyRequest {
    let reference = test_reference();
    base_request(
        backend,
        AccessPolicyRequest::direct_io_read().with_alignment_requirement(direct_io_alignment(
            backend,
            reference,
            pinned_lifecycle(),
            4096,
        )),
    )
}

pub fn mmap_request(backend: &AdmittedBackendCapabilityWitness) -> AccessPolicyRequest {
    base_request(
        backend,
        AccessPolicyRequest::mmap_read().with_mmap_fault_posture(admitted_mmap_posture(backend)),
    )
}

pub fn mixed_direct_buffered_request(
    backend: &AdmittedBackendCapabilityWitness,
    transition: MixedAccessTransition,
) -> AccessPolicyRequest {
    let reference = test_reference();
    let scope = test_security_scope();
    AccessPolicyRequest::mixed_read(transition)
        .for_physical_reference(reference)
        .with_security_scope(scope)
        .with_buffer_lifecycle(pinned_lifecycle())
        .with_page_cache_policy(page_cache_policy(backend))
        .with_alignment_requirement(direct_io_alignment(
            backend,
            reference,
            pinned_lifecycle(),
            4096,
        ))
        .with_coherence_basis(admitted_mixed_basis(backend, transition, reference, scope))
}

pub fn page_cache_policy(backend: &AdmittedBackendCapabilityWitness) -> PageCachePolicyProof {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .page_cache_policy()
        .expect("test backend carries Store-admitted page-cache posture")
}

pub fn admitted_mmap_posture(backend: &AdmittedBackendCapabilityWitness) -> MmapFaultPosture {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .mmap_posture()
        .expect("test backend carries mmap capability posture")
}

pub fn direct_io_alignment(
    backend: &AdmittedBackendCapabilityWitness,
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

pub fn admitted_mixed_basis(
    backend: &AdmittedBackendCapabilityWitness,
    transition: MixedAccessTransition,
    reference: PhysicalReference,
    scope: AccessPolicySecurityScope,
) -> MixedAccessCoherenceBasis {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .mixed_coherence(transition, reference, scope)
        .expect("test backend carries mixed access coherence assumptions")
}

pub const fn pinned_lifecycle() -> AccessPolicyBufferLifecycle {
    AccessPolicyBufferLifecycle::for_certification_pinned_physical_substrate_lease()
}

pub fn test_security_scope() -> AccessPolicySecurityScope {
    let admitted = admitted_store_internal_security_scope_for_io_qos_test();
    AccessPolicySecurityScope::from_current_store_scope(admitted.witnesses())
}

pub fn test_reference() -> PhysicalReference {
    reference_for_page(1)
}

fn reference_for_page(page_value: u64) -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(page_value).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(cell)
        .reference()
}
