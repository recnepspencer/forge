use crate::s6_access_policy_support::{
    base_request, page_cache_policy, pinned_lifecycle, test_reference, test_security_scope,
    ExpectedExecutionRequest,
};
use worth_store_physical_backend::{
    AccessPolicyRequest, BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis,
    BackendCapabilityKind, BackendCapabilitySupportPosture, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile, MixedAccessTransition,
    PhysicalBackendCapabilityAdmissionAuthority, StoreAccessMode, StoreAccessPolicyProofAuthority,
};
use worth_store_physical_format::PhysicalAlignmentClass;

pub(crate) fn direct_io_request(
    backend: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
) -> AccessPolicyRequest {
    base_request(
        backend,
        AccessPolicyRequest::direct_io_read().with_alignment_requirement(
            StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
                .direct_io_page_and_sector_aligned(
                    test_reference(),
                    pinned_lifecycle(),
                    4096,
                    PhysicalAlignmentClass::page_start_4k(),
                    PhysicalAlignmentClass::extent_start_4k(),
                )
                .expect("test backend carries direct I/O alignment assumptions"),
        ),
    )
}

pub(crate) fn mixed_request(
    backend: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
    transition: MixedAccessTransition,
) -> AccessPolicyRequest {
    let reference = test_reference();
    let scope = test_security_scope();
    AccessPolicyRequest::mixed_read(transition)
        .for_physical_reference(reference)
        .with_security_scope(scope)
        .with_buffer_lifecycle(pinned_lifecycle())
        .with_page_cache_policy(page_cache_policy(backend))
        .with_alignment_requirement(
            StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
                .direct_io_page_and_sector_aligned(
                    reference,
                    pinned_lifecycle(),
                    4096,
                    PhysicalAlignmentClass::page_start_4k(),
                    PhysicalAlignmentClass::extent_start_4k(),
                )
                .expect("test backend carries direct I/O alignment assumptions"),
        )
        .with_coherence_basis(
            StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
                .mixed_coherence(transition, reference, scope)
                .expect("test backend carries mixed access coherence assumptions"),
        )
}

pub(crate) fn backend_with_access(
    direct_io: BackendCapabilitySupportPosture,
) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    let support = BackendCapabilitySupportSet::all_supported()
        .with_posture(BackendCapabilityKind::DirectIo, direct_io);
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
        .expect("backend admits")
}

impl ExpectedExecutionRequest {
    pub(crate) fn direct_io() -> Self {
        Self::new(StoreAccessMode::DirectIo, None, true, false)
    }

    pub(crate) fn mixed(transition: MixedAccessTransition) -> Self {
        Self::new(StoreAccessMode::Mixed, Some(transition), true, false)
    }
}
