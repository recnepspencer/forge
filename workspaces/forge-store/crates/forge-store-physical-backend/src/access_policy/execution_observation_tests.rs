use super::test_support::{
    backend_with_all_access_modes, base_request, direct_io_request, mixed_direct_buffered_request,
    mmap_request, test_reference,
};
use super::{
    AccessPolicyAdmission, AccessPolicyCounterSnapshot, AccessPolicyExecutionObservation,
    AccessPolicyExecutionRequest, AccessPolicyExecutionSession, AccessPolicyViolationKind,
    AdmittedAccessPolicy, MixedAccessTransition, PhysicalStoreAccessPolicyExecutor,
    StoreAccessMode,
};
use crate::PhysicalReference;

#[test]
fn mixed_execution_requires_positive_invalidation_observation() {
    let backend = backend_with_all_access_modes();
    let transition =
        MixedAccessTransition::new(StoreAccessMode::DirectIo, StoreAccessMode::Buffered);
    let admitted = admit(
        mixed_direct_buffered_request(&backend, transition),
        &backend,
    );

    let violation = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::mixed_direct_buffered(transition),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_page_cache_visibility_observed()
                .with_direct_io_alignment_observed()
                .with_security_scope_preserved(),
        ),
    )
    .expect_err("missing mixed invalidation must violate after admission");
    assert_eq!(
        violation.kind(),
        AccessPolicyViolationKind::MixedModeInvalidationMissed
    );
    assert_mixed_counters(violation.counters(), 1, 1);

    let receipt = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::mixed_direct_buffered(transition),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_page_cache_visibility_observed()
                .with_direct_io_alignment_observed()
                .with_mixed_mode_invalidation_observed()
                .with_security_scope_preserved(),
        ),
    )
    .expect("complete mixed observations execute");
    assert_mixed_counters(receipt.counters(), 0, 1);
}

#[test]
fn buffered_execution_observes_page_cache_and_security() {
    let backend = backend_with_all_access_modes();
    let admitted = admit(
        base_request(&backend, super::AccessPolicyRequest::buffered_read()),
        &backend,
    );

    let missing_page_cache = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::buffered(),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_security_scope_preserved(),
        ),
    )
    .expect_err("buffered access must observe page-cache visibility");
    assert_eq!(
        missing_page_cache.kind(),
        AccessPolicyViolationKind::PageCacheVisibilityLost
    );
    assert_eq!(
        missing_page_cache.counters().page_cache_visibility_checks(),
        2
    );

    let missing_security = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::buffered(),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_page_cache_visibility_observed(),
        ),
    )
    .expect_err("buffered access must preserve security scope");
    assert_eq!(
        missing_security.kind(),
        AccessPolicyViolationKind::BackendContradictedWitness
    );
    assert_eq!(missing_security.counters().violations(), 1);
}

#[test]
fn mmap_execution_observes_page_cache_security_and_faults() {
    let backend = backend_with_all_access_modes();
    let admitted = admit(mmap_request(&backend), &backend);

    let missing_page_cache = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::mmap(),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_security_scope_preserved(),
        ),
    )
    .expect_err("mmap access must observe page-cache visibility");
    assert_eq!(
        missing_page_cache.kind(),
        AccessPolicyViolationKind::PageCacheVisibilityLost
    );

    let missing_security = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::mmap(),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_page_cache_visibility_observed(),
        ),
    )
    .expect_err("mmap access must preserve security scope");
    assert_eq!(
        missing_security.kind(),
        AccessPolicyViolationKind::BackendContradictedWitness
    );

    let fault = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::mmap(),
            AccessPolicyExecutionObservation::mmap_lazy_fault(),
        ),
    )
    .expect_err("mmap lazy fault remains typed");
    assert_eq!(fault.kind(), AccessPolicyViolationKind::MmapLazyFault);
    assert_eq!(fault.counters().mmap_fault_observations(), 1);
}

#[test]
fn direct_io_execution_observes_alignment_page_cache_and_security() {
    let backend = backend_with_all_access_modes();
    let admitted = admit(direct_io_request(&backend), &backend);

    let missing_alignment = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::direct_io(),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_page_cache_visibility_observed()
                .with_security_scope_preserved(),
        ),
    )
    .expect_err("direct I/O must observe alignment");
    assert_eq!(
        missing_alignment.kind(),
        AccessPolicyViolationKind::DirectIoAlignmentContradicted
    );
    assert_eq!(missing_alignment.counters().direct_io_alignment_checks(), 2);

    let missing_page_cache = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::direct_io(),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_direct_io_alignment_observed()
                .with_security_scope_preserved(),
        ),
    )
    .expect_err("direct I/O must observe page-cache posture");
    assert_eq!(
        missing_page_cache.kind(),
        AccessPolicyViolationKind::PageCacheVisibilityLost
    );

    let receipt = execute_with(
        admitted,
        StrictAccessBackend::new(
            ExpectedExecutionRequest::direct_io(),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_page_cache_visibility_observed()
                .with_direct_io_alignment_observed()
                .with_security_scope_preserved(),
        ),
    )
    .expect("complete direct I/O observations execute");
    assert_eq!(receipt.counters().direct_io_alignment_checks(), 2);
    assert_eq!(receipt.counters().page_cache_visibility_checks(), 1);
}

fn admit(
    request: super::AccessPolicyRequest,
    backend: &crate::AdmittedBackendCapabilityWitness,
) -> AdmittedAccessPolicy {
    AccessPolicyAdmission::for_backend(backend)
        .admit(request)
        .expect("access policy admits")
}

fn execute_with(
    admitted: AdmittedAccessPolicy,
    mut backend: StrictAccessBackend,
) -> Result<super::AccessPolicyExecutionReceipt, super::AccessPolicyViolation> {
    AccessPolicyExecutionSession::for_owned_backend(&mut backend)
        .execute(admitted)
        .expect("strict backend executes")
}

fn assert_mixed_counters(
    counters: AccessPolicyCounterSnapshot,
    violations: u64,
    mixed_invalidations: u64,
) {
    assert_eq!(counters.mixed_mode_admissions(), 1);
    assert_eq!(counters.page_cache_visibility_checks(), 1);
    assert_eq!(counters.direct_io_alignment_checks(), 1);
    assert_eq!(counters.mixed_mode_invalidations(), mixed_invalidations);
    assert_eq!(counters.security_scope_preservations(), 1);
    assert_eq!(counters.violations(), violations);
}

#[derive(Clone, Copy)]
struct ExpectedExecutionRequest {
    mode: StoreAccessMode,
    reference: PhysicalReference,
    transition: Option<MixedAccessTransition>,
    requires_direct_io: bool,
    requires_mmap: bool,
}

impl ExpectedExecutionRequest {
    fn buffered() -> Self {
        Self::new(StoreAccessMode::Buffered, None, false, false)
    }

    fn mmap() -> Self {
        Self::new(StoreAccessMode::Mmap, None, false, true)
    }

    fn direct_io() -> Self {
        Self::new(StoreAccessMode::DirectIo, None, true, false)
    }

    fn mixed_direct_buffered(transition: MixedAccessTransition) -> Self {
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

struct StrictAccessBackend {
    expected: ExpectedExecutionRequest,
    observation: AccessPolicyExecutionObservation,
}

impl StrictAccessBackend {
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

impl PhysicalStoreAccessPolicyExecutor for StrictAccessBackend {
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
