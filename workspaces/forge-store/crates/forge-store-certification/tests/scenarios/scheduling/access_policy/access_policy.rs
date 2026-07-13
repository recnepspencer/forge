use forge_store_certification::{S6AccessPolicyEvidenceOutcomeKind, S6AccessPolicyEvidenceRow};
use forge_store_physical_backend::{
    AccessPolicyAdmission, AccessPolicyBufferLifecycle, AccessPolicyDenialKind,
    AccessPolicyExecutionObservation, AccessPolicyRequest, AccessPolicyViolationKind,
    BackendCapabilitySupportPosture, MixedAccessTransition, StoreAccessMode,
    StoreAccessPolicyProofAuthority,
};
use forge_store_physical_format::PhysicalAlignmentClass;

#[path = "../../../support/scheduling/access_policy_support/extended.rs"]
mod extended_access_policy_support;
#[path = "../../../support/scheduling/access_policy_support/access_policy_support.rs"]
mod s6_access_policy_support;

use extended_access_policy_support::{backend_with_access, direct_io_request, mixed_request};
use s6_access_policy_support::{
    admit, base_request, executed_row, mmap_request, pinned_lifecycle, test_reference,
    violation_row, ExpectedExecutionRequest,
};

#[test]
fn certification_materializes_buffered_access_policy_without_minting_authority() {
    let backend = backend_with_access(BackendCapabilitySupportPosture::Supported);
    let request = base_request(&backend, AccessPolicyRequest::buffered_read());
    let admitted = AccessPolicyAdmission::for_backend(&backend)
        .admit(request)
        .expect("buffered access admits");

    let row = S6AccessPolicyEvidenceRow::from_admitted(admitted);

    assert_eq!(row.mode(), StoreAccessMode::Buffered);
    assert_eq!(row.outcome(), S6AccessPolicyEvidenceOutcomeKind::Admitted);
    assert_eq!(row.counters().buffered_admissions(), 1);
    assert!(row.security_scope().is_some());
}

#[test]
fn certification_records_direct_io_unknown_as_denial() {
    let backend = backend_with_access(BackendCapabilitySupportPosture::Unknown);
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
        .expect_err("unknown direct I/O denies");

    let row = S6AccessPolicyEvidenceRow::from_denial(StoreAccessMode::DirectIo, denial);

    assert_eq!(
        row.outcome(),
        S6AccessPolicyEvidenceOutcomeKind::Denied(AccessPolicyDenialKind::BackendCapabilityDenied)
    );
    assert_eq!(row.counters().denials(), 1);
}

#[test]
fn certification_materializes_successful_execution_receipts() {
    let backend = backend_with_access(BackendCapabilitySupportPosture::Supported);

    let buffered = executed_row(
        admit(
            &backend,
            base_request(&backend, AccessPolicyRequest::buffered_read()),
        ),
        ExpectedExecutionRequest::buffered(),
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed()
            .with_security_scope_preserved(),
    );
    assert_eq!(
        buffered.outcome(),
        S6AccessPolicyEvidenceOutcomeKind::Executed
    );
    assert_eq!(buffered.mode(), StoreAccessMode::Buffered);
    assert_eq!(buffered.counters().page_cache_visibility_checks(), 2);

    let direct = executed_row(
        admit(&backend, direct_io_request(&backend)),
        ExpectedExecutionRequest::direct_io(),
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed()
            .with_direct_io_alignment_observed()
            .with_security_scope_preserved(),
    );
    assert_eq!(direct.mode(), StoreAccessMode::DirectIo);
    assert_eq!(direct.counters().direct_io_alignment_checks(), 2);

    let mmap = executed_row(
        admit(&backend, mmap_request(&backend)),
        ExpectedExecutionRequest::mmap(),
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed()
            .with_security_scope_preserved(),
    );
    assert_eq!(mmap.mode(), StoreAccessMode::Mmap);
    assert_eq!(mmap.counters().page_cache_visibility_checks(), 1);

    let transition =
        MixedAccessTransition::new(StoreAccessMode::DirectIo, StoreAccessMode::Buffered);
    let mixed = executed_row(
        admit(&backend, mixed_request(&backend, transition)),
        ExpectedExecutionRequest::mixed(transition),
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed()
            .with_direct_io_alignment_observed()
            .with_mixed_mode_invalidation_observed()
            .with_security_scope_preserved(),
    );
    assert_eq!(mixed.mode(), StoreAccessMode::Mixed);
    assert_eq!(mixed.counters().mixed_mode_invalidations(), 1);
}

#[test]
fn certification_materializes_execution_violation_rows() {
    let backend = backend_with_access(BackendCapabilitySupportPosture::Supported);

    let page_cache = violation_row(
        admit(
            &backend,
            base_request(&backend, AccessPolicyRequest::buffered_read()),
        ),
        ExpectedExecutionRequest::buffered(),
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_security_scope_preserved(),
    );
    assert_eq!(
        page_cache.outcome(),
        S6AccessPolicyEvidenceOutcomeKind::Violated(
            AccessPolicyViolationKind::PageCacheVisibilityLost
        )
    );

    let security = violation_row(
        admit(
            &backend,
            base_request(&backend, AccessPolicyRequest::buffered_read()),
        ),
        ExpectedExecutionRequest::buffered(),
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed(),
    );
    assert_eq!(
        security.outcome(),
        S6AccessPolicyEvidenceOutcomeKind::Violated(
            AccessPolicyViolationKind::BackendContradictedWitness
        )
    );

    let direct_alignment = violation_row(
        admit(&backend, direct_io_request(&backend)),
        ExpectedExecutionRequest::direct_io(),
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed()
            .with_security_scope_preserved(),
    );
    assert_eq!(
        direct_alignment.outcome(),
        S6AccessPolicyEvidenceOutcomeKind::Violated(
            AccessPolicyViolationKind::DirectIoAlignmentContradicted
        )
    );

    let transition =
        MixedAccessTransition::new(StoreAccessMode::DirectIo, StoreAccessMode::Buffered);
    let mixed_invalidation = violation_row(
        admit(&backend, mixed_request(&backend, transition)),
        ExpectedExecutionRequest::mixed(transition),
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed()
            .with_direct_io_alignment_observed()
            .with_security_scope_preserved(),
    );
    assert_eq!(
        mixed_invalidation.outcome(),
        S6AccessPolicyEvidenceOutcomeKind::Violated(
            AccessPolicyViolationKind::MixedModeInvalidationMissed
        )
    );

    let mmap_fault = violation_row(
        admit(&backend, mmap_request(&backend)),
        ExpectedExecutionRequest::mmap(),
        AccessPolicyExecutionObservation::mmap_lazy_fault(),
    );
    assert_eq!(
        mmap_fault.outcome(),
        S6AccessPolicyEvidenceOutcomeKind::Violated(AccessPolicyViolationKind::MmapLazyFault)
    );
    assert_eq!(mmap_fault.counters().mmap_fault_observations(), 1);
}

#[test]
fn certification_materializes_dirty_mmap_direct_io_denial() {
    let backend = backend_with_access(BackendCapabilitySupportPosture::Supported);
    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(
            base_request(&backend, AccessPolicyRequest::direct_io_read()).with_buffer_lifecycle(
                AccessPolicyBufferLifecycle::for_certification_dirty_mmap_page(),
            ),
        )
        .expect_err("dirty mmap page blocks direct I/O");

    let row = S6AccessPolicyEvidenceRow::from_denial(StoreAccessMode::DirectIo, denial);

    assert_eq!(
        row.outcome(),
        S6AccessPolicyEvidenceOutcomeKind::Denied(
            AccessPolicyDenialKind::DirtyMmapPageBlocksDirectIo
        )
    );
}
