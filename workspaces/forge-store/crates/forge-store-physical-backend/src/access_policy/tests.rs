use super::*;
use crate::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilityKind,
    BackendCapabilitySupportPosture, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_format::{
    PhysicalAlignmentClass, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
};
use forge_store_security::admitted_store_internal_security_scope_for_io_qos_test;

#[test]
fn buffered_access_admits_page_cache_visibility_and_security_scope() {
    let backend = backend_with_all_access_modes();
    let request = base_request(&backend, AccessPolicyRequest::buffered_read());
    let admitted = AccessPolicyAdmission::for_backend(&backend)
        .admit(request)
        .expect("buffered policy admits with backend and security proof");
    assert_eq!(admitted.mode(), StoreAccessMode::Buffered);
    assert_eq!(admitted.counters().buffered_admissions(), 1);
    assert_eq!(admitted.counters().page_cache_visibility_checks(), 1);
    assert_eq!(admitted.counters().security_scope_preservations(), 1);
}

#[test]
fn direct_io_denies_without_alignment_before_execution() {
    let backend = backend_with_all_access_modes();
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);
    let reference = test_reference();
    let lifecycle = pinned_lifecycle();
    let request = base_request(
        &backend,
        AccessPolicyRequest::direct_io_read()
            .with_alignment_requirement(proof.direct_io_unaligned_for_denial(reference, lifecycle)),
    );
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(request)
        .expect_err("unaligned direct I/O denies before execution");
    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::DirectIoAlignmentRequired
    );
    assert_eq!(denial.counters().direct_io_alignment_checks(), 1);
    assert_eq!(denial.counters().denials(), 1);
}

#[test]
fn unknown_direct_io_capability_remains_typed_backend_denial() {
    let backend = backend_with_direct_io_posture(BackendCapabilitySupportPosture::Unknown);
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);
    let request = base_request(
        &backend,
        AccessPolicyRequest::direct_io_read().with_alignment_requirement(
            proof
                .direct_io_page_and_sector_aligned(
                    test_reference(),
                    pinned_lifecycle(),
                    4096,
                    PhysicalAlignmentClass::page_start_4k(),
                    PhysicalAlignmentClass::extent_start_4k(),
                )
                .expect("test backend carries direct I/O alignment assumptions"),
        ),
    );
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(request)
        .expect_err("unknown direct I/O posture denies before execution");
    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::BackendCapabilityDenied
    );
    assert!(denial.backend_denial().is_some());
}

#[test]
fn mmap_requires_fault_posture_and_reports_fault_violation() {
    let backend = backend_with_all_access_modes();
    let unsupported = base_request(&backend, AccessPolicyRequest::mmap_read());
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(unsupported)
        .expect_err("mmap without admitted fault posture denies");
    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::MmapFaultPostureUnsupported
    );

    let admitted_request = base_request(
        &backend,
        AccessPolicyRequest::mmap_read().with_mmap_fault_posture(admitted_mmap_posture(&backend)),
    );
    let admitted = AccessPolicyAdmission::for_backend(&backend)
        .admit(admitted_request)
        .expect("typed mmap fault posture admits");
    let mut backend =
        ScriptedAccessBackend::new(AccessPolicyExecutionObservation::mmap_lazy_fault());
    let violation = AccessPolicyExecutionSession::for_owned_backend(&mut backend)
        .execute(admitted)
        .expect("scripted backend executes")
        .expect_err("mmap lazy fault is a typed post-admission violation");
    assert_eq!(violation.kind(), AccessPolicyViolationKind::MmapLazyFault);
    assert_eq!(violation.counters().mmap_fault_observations(), 1);
}

#[test]
fn mixed_access_requires_positive_coherence_basis() {
    let backend = backend_with_all_access_modes();
    let transition = MixedAccessTransition::new(StoreAccessMode::Buffered, StoreAccessMode::Mmap);
    let missing_basis = base_request(&backend, AccessPolicyRequest::mixed_read(transition))
        .with_mmap_fault_posture(admitted_mmap_posture(&backend));
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(missing_basis)
        .expect_err("mixed access without coherence denies");
    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::MixedModeCoherenceRequired
    );

    let reference = test_reference();
    let scope = test_security_scope();
    let transition =
        MixedAccessTransition::new(StoreAccessMode::DirectIo, StoreAccessMode::Buffered);
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);
    let admitted = AccessPolicyAdmission::for_backend(&backend)
        .admit(
            AccessPolicyRequest::mixed_read(transition)
                .for_physical_reference(reference)
                .with_security_scope(scope)
                .with_buffer_lifecycle(
                    AccessPolicyBufferLifecycle::for_certification_pinned_physical_substrate_lease(
                    ),
                )
                .with_page_cache_policy(page_cache_policy(&backend))
                .with_alignment_requirement(
                    proof
                        .direct_io_page_and_sector_aligned(
                            reference,
                            pinned_lifecycle(),
                            4096,
                            PhysicalAlignmentClass::page_start_4k(),
                            PhysicalAlignmentClass::extent_start_4k(),
                        )
                        .expect("test backend carries direct I/O alignment assumptions"),
                )
                .with_coherence_basis(admitted_mixed_basis(&backend, transition, reference, scope)),
        )
        .expect("positive mixed-mode coherence basis admits");
    assert_eq!(admitted.counters().mixed_mode_admissions(), 1);
}

#[test]
fn mixed_access_denies_when_coherence_basis_belongs_to_other_region() {
    let backend = backend_with_all_access_modes();
    let transition =
        MixedAccessTransition::new(StoreAccessMode::Buffered, StoreAccessMode::DirectIo);
    let reference = test_reference();
    let scope = test_security_scope();
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);
    let mismatched = admitted_mixed_basis(&backend, transition, other_reference(), scope);
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(
            AccessPolicyRequest::mixed_read(transition)
                .for_physical_reference(reference)
                .with_security_scope(scope)
                .with_buffer_lifecycle(
                    AccessPolicyBufferLifecycle::for_certification_pinned_physical_substrate_lease(
                    ),
                )
                .with_page_cache_policy(page_cache_policy(&backend))
                .with_alignment_requirement(
                    proof
                        .direct_io_page_and_sector_aligned(
                            reference,
                            pinned_lifecycle(),
                            4096,
                            PhysicalAlignmentClass::page_start_4k(),
                            PhysicalAlignmentClass::extent_start_4k(),
                        )
                        .expect("test backend carries direct I/O alignment assumptions"),
                )
                .with_coherence_basis(mismatched),
        )
        .expect_err("coherence proof is bound to the physical reference");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::MixedModeCoherenceRequired
    );
}

#[test]
fn dirty_mmap_page_blocks_direct_io_before_backend_touch() {
    let backend = backend_with_all_access_modes();
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);
    let request = AccessPolicyRequest::direct_io_read()
        .for_physical_reference(test_reference())
        .with_security_scope(test_security_scope())
        .with_alignment_requirement(
            proof
                .direct_io_page_and_sector_aligned(
                    test_reference(),
                    AccessPolicyBufferLifecycle::for_certification_dirty_mmap_page(),
                    4096,
                    PhysicalAlignmentClass::page_start_4k(),
                    PhysicalAlignmentClass::extent_start_4k(),
                )
                .expect("test backend carries direct I/O alignment assumptions"),
        )
        .with_page_cache_policy(page_cache_policy(&backend))
        .with_buffer_lifecycle(AccessPolicyBufferLifecycle::for_certification_dirty_mmap_page());
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(request)
        .expect_err("dirty mmap page blocks direct I/O");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::DirtyMmapPageBlocksDirectIo
    );
}

#[test]
fn successful_direct_io_receipt_requires_positive_alignment_and_security_observations() {
    let backend = backend_with_all_access_modes();
    let request = base_request(
        &backend,
        AccessPolicyRequest::direct_io_read().with_alignment_requirement(direct_io_alignment(
            &backend,
            test_reference(),
            pinned_lifecycle(),
            4096,
        )),
    );
    let admitted = AccessPolicyAdmission::for_backend(&backend)
        .admit(request)
        .expect("direct I/O policy admits");

    let mut missing_alignment = ScriptedAccessBackend::new(
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed()
            .with_security_scope_preserved(),
    );
    let violation = AccessPolicyExecutionSession::for_owned_backend(&mut missing_alignment)
        .execute(admitted)
        .expect("scripted backend executes")
        .expect_err("missing direct I/O alignment observation violates execution");
    assert_eq!(
        violation.kind(),
        AccessPolicyViolationKind::DirectIoAlignmentContradicted
    );

    let mut complete = ScriptedAccessBackend::new(
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed()
            .with_direct_io_alignment_observed()
            .with_security_scope_preserved(),
    );
    let receipt = AccessPolicyExecutionSession::for_owned_backend(&mut complete)
        .execute(admitted)
        .expect("scripted backend executes")
        .expect("positive observations complete execution");
    assert!(receipt.counters().direct_io_alignment_checks() >= 1);
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

fn pinned_lifecycle() -> AccessPolicyBufferLifecycle {
    AccessPolicyBufferLifecycle::for_certification_pinned_physical_substrate_lease()
}

fn page_cache_policy(backend: &crate::AdmittedBackendCapabilityWitness) -> PageCachePolicyProof {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .page_cache_policy()
        .expect("test backend carries Store-admitted page-cache posture")
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

fn admitted_mmap_posture(backend: &crate::AdmittedBackendCapabilityWitness) -> MmapFaultPosture {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .mmap_posture()
        .expect("test backend carries mmap capability posture")
}

fn admitted_mixed_basis(
    backend: &crate::AdmittedBackendCapabilityWitness,
    transition: MixedAccessTransition,
    reference: PhysicalReference,
    scope: AccessPolicySecurityScope,
) -> MixedAccessCoherenceBasis {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .mixed_coherence(transition, reference, scope)
        .expect("test backend carries page-cache and mixed coherence posture")
}

fn backend_with_all_access_modes() -> crate::AdmittedBackendCapabilityWitness {
    backend_with_direct_io_posture(BackendCapabilitySupportPosture::Supported)
}

fn backend_with_direct_io_posture(
    posture: BackendCapabilitySupportPosture,
) -> crate::AdmittedBackendCapabilityWitness {
    let support = BackendCapabilitySupportSet::all_supported()
        .with_posture(BackendCapabilityKind::DirectIo, posture);
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
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(cell)
        .reference()
}

fn other_reference() -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(2).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(cell)
        .reference()
}

struct ScriptedAccessBackend {
    observation: AccessPolicyExecutionObservation,
}

impl ScriptedAccessBackend {
    const fn new(observation: AccessPolicyExecutionObservation) -> Self {
        Self { observation }
    }
}

impl PhysicalStoreAccessPolicyExecutor for ScriptedAccessBackend {
    type Error = core::convert::Infallible;

    fn execute_access_policy(
        &mut self,
        request: AccessPolicyExecutionRequest,
    ) -> Result<AccessPolicyExecutionObservation, Self::Error> {
        assert!(request.access_request().reference().is_some());
        Ok(self.observation)
    }
}
