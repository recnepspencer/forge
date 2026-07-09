use worth_store_certification::S6AccessPolicyEvidenceRow;
use worth_store_physical_backend::{
    AccessPolicyAdmission, AccessPolicyBufferLifecycle, AccessPolicyExecutionObservation,
    AccessPolicyExecutionRequest, AccessPolicyExecutionSession, AccessPolicyRequest,
    AccessPolicySecurityScope, AdmittedAccessPolicy, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
    BackendTargetProfile, MixedAccessTransition, PageCachePolicyProof,
    PhysicalBackendCapabilityAdmissionAuthority, PhysicalStoreAccessPolicyExecutor,
    StoreAccessMode, StoreAccessPolicyProofAuthority, StoreOwnedAccessPolicyExecution,
};
use worth_store_physical_format::{
    PhysicalAlignmentClass, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
};
use worth_store_security::admitted_store_internal_security_scope_for_s6_test;

pub fn base_request(
    backend: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
    request: AccessPolicyRequest,
) -> AccessPolicyRequest {
    request
        .for_physical_reference(test_reference())
        .with_security_scope(test_security_scope())
        .with_buffer_lifecycle(pinned_lifecycle())
        .with_page_cache_policy(page_cache_policy(backend))
}

pub fn admit(
    backend: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
    request: AccessPolicyRequest,
) -> AdmittedAccessPolicy {
    AccessPolicyAdmission::for_backend(backend)
        .admit(request)
        .expect("access policy admits")
}

pub fn executed_row(
    admitted: AdmittedAccessPolicy,
    expected: ExpectedExecutionRequest,
    observation: AccessPolicyExecutionObservation,
) -> S6AccessPolicyEvidenceRow {
    let mut executor = AssertingAccessBackend::new(expected, observation);
    let receipt = AccessPolicyExecutionSession::for_store_backend(
        &mut executor,
        StoreOwnedAccessPolicyExecution::for_certification_test_authority(),
    )
    .execute(admitted)
    .expect("backend executes")
    .expect("execution succeeds");
    S6AccessPolicyEvidenceRow::from_execution_receipt(receipt)
}

pub fn violation_row(
    admitted: AdmittedAccessPolicy,
    expected: ExpectedExecutionRequest,
    observation: AccessPolicyExecutionObservation,
) -> S6AccessPolicyEvidenceRow {
    let mode = admitted.mode();
    let mut executor = AssertingAccessBackend::new(expected, observation);
    let violation = AccessPolicyExecutionSession::for_store_backend(
        &mut executor,
        StoreOwnedAccessPolicyExecution::for_certification_test_authority(),
    )
    .execute(admitted)
    .expect("backend executes")
    .expect_err("execution violates");
    S6AccessPolicyEvidenceRow::from_violation(mode, violation)
}

pub fn direct_io_request(
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

pub fn mmap_request(
    backend: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
) -> AccessPolicyRequest {
    base_request(
        backend,
        AccessPolicyRequest::mmap_read().with_mmap_fault_posture(admitted_mmap_posture(backend)),
    )
}

pub fn mixed_request(
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

pub fn pinned_lifecycle() -> AccessPolicyBufferLifecycle {
    AccessPolicyBufferLifecycle::for_certification_pinned_s2_lease()
}

pub fn backend_with_access(
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

pub fn test_reference() -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    PhysicalReferenceAuthority::s1()
        .admit_page_slot(cell)
        .reference()
}

fn page_cache_policy(
    backend: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
) -> PageCachePolicyProof {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .page_cache_policy()
        .expect("test backend carries Store-admitted page-cache posture")
}

fn admitted_mmap_posture(
    backend: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
) -> worth_store_physical_backend::MmapFaultPosture {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .mmap_posture()
        .expect("test backend carries mmap capability posture")
}

fn test_security_scope() -> AccessPolicySecurityScope {
    let admitted = admitted_store_internal_security_scope_for_s6_test();
    AccessPolicySecurityScope::from_current_store_scope(admitted.witnesses())
}

#[derive(Clone, Copy)]
pub struct ExpectedExecutionRequest {
    mode: StoreAccessMode,
    reference: PhysicalReference,
    transition: Option<MixedAccessTransition>,
    requires_direct_io: bool,
    requires_mmap: bool,
}

impl ExpectedExecutionRequest {
    pub fn buffered() -> Self {
        Self::new(StoreAccessMode::Buffered, None, false, false)
    }

    pub fn mmap() -> Self {
        Self::new(StoreAccessMode::Mmap, None, false, true)
    }

    pub fn direct_io() -> Self {
        Self::new(StoreAccessMode::DirectIo, None, true, false)
    }

    pub fn mixed(transition: MixedAccessTransition) -> Self {
        Self::new(StoreAccessMode::Mixed, Some(transition), true, false)
    }

    fn new(
        mode: StoreAccessMode,
        transition: Option<MixedAccessTransition>,
        requires_direct_io: bool,
        requires_mmap: bool,
    ) -> Self {
        Self {
            mode,
            reference: test_reference(),
            transition,
            requires_direct_io,
            requires_mmap,
        }
    }
}

struct AssertingAccessBackend {
    expected: ExpectedExecutionRequest,
    observation: AccessPolicyExecutionObservation,
}

impl AssertingAccessBackend {
    const fn new(
        expected: ExpectedExecutionRequest,
        observation: AccessPolicyExecutionObservation,
    ) -> Self {
        Self {
            expected,
            observation,
        }
    }
}

impl PhysicalStoreAccessPolicyExecutor for AssertingAccessBackend {
    type Error = core::convert::Infallible;

    fn execute_access_policy(
        &mut self,
        request: AccessPolicyExecutionRequest,
    ) -> Result<AccessPolicyExecutionObservation, Self::Error> {
        let access = request.access_request();
        assert_eq!(access.mode(), self.expected.mode);
        assert_eq!(access.reference(), Some(self.expected.reference));
        assert!(access.security_scope().is_some());
        assert!(access.page_cache_policy().is_some());
        assert_eq!(access.mixed_transition(), self.expected.transition);
        assert_eq!(
            access.alignment().is_some(),
            self.expected.requires_direct_io
        );
        assert!(!self.expected.requires_mmap || access.mmap_fault_posture().admits_mmap());
        if self.expected.transition.is_some() {
            assert!(access.coherence_basis().is_some());
        }
        Ok(self.observation)
    }
}
