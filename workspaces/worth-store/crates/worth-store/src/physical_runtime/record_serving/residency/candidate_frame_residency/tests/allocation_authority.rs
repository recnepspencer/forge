use worth_store_buffer_pool::{
    CandidateFrameCleanAuthority, ForegroundWriteAllocationGrant, PhysicalOperationAllocationScope,
    PhysicalResidencyLimits, PhysicalResidencyPool, PhysicalResidencyPoolOwner,
    PhysicalSpeculativeWorkKind,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};

use super::{
    declared_inline_frames, BoundedCandidateFramePublisher, CandidateFrameCounterCells,
    CandidateFramePublicationPort, RecordAppendDenial,
};

pub(super) fn test_pool(identity_byte: u8) -> PhysicalResidencyPool {
    PhysicalResidencyPool::open(test_store(identity_byte), test_residency_limits()).unwrap()
}

fn owned_test_pool(
    identity_byte: u8,
) -> (
    PhysicalResidencyPool,
    std::sync::Arc<CandidateFrameCleanAuthority>,
) {
    let (pool, candidate_clean, _) =
        PhysicalResidencyPoolOwner::open(test_store(identity_byte), test_residency_limits())
            .unwrap()
            .into_parts();
    (pool, std::sync::Arc::new(candidate_clean))
}

fn test_store(
    identity_byte: u8,
) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
    StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([identity_byte; 16]).unwrap(),
    )
    .published_identity()
}

pub(super) fn publication_allocation(
    pool: &PhysicalResidencyPool,
) -> ForegroundWriteAllocationGrant {
    pool.begin_foreground_write_operation(
        PhysicalResidencyPool::candidate_batch_operation_bytes(std::num::NonZeroUsize::MIN)
            .unwrap(),
    )
    .unwrap()
}

fn test_residency_limits() -> PhysicalResidencyLimits {
    let bytes = std::num::NonZeroU64::new(4096).unwrap();
    let operation =
        PhysicalResidencyPool::candidate_batch_operation_bytes(std::num::NonZeroUsize::MIN)
            .unwrap();
    let frames = std::num::NonZeroU32::new(4).unwrap();
    PhysicalResidencyLimits::builder()
        .total_bytes(std::num::NonZeroU64::new(12_288 + operation.get()).unwrap())
        .resident_bytes(bytes)
        .metadata_bytes(bytes)
        .frame_entries(frames)
        .pinned_frames(frames)
        .pin_leases(frames)
        .dirty_frames(frames)
        .dirty_replacement_bytes(bytes)
        .operation_bytes(operation)
        .scope_bytes(PhysicalOperationAllocationScope::ForegroundRead, operation)
        .scope_bytes(PhysicalOperationAllocationScope::ForegroundWrite, operation)
        .scope_bytes(PhysicalOperationAllocationScope::Recovery, operation)
        .scope_bytes(PhysicalOperationAllocationScope::Scrub, operation)
        .scope_bytes(PhysicalOperationAllocationScope::Maintenance, operation)
        .scope_bytes(PhysicalOperationAllocationScope::Verification, operation)
        .scope_bytes(PhysicalOperationAllocationScope::Blob, operation)
        .speculative_frames(PhysicalSpeculativeWorkKind::Prefetch, frames)
        .speculative_frames(PhysicalSpeculativeWorkKind::ReadAhead, frames)
        .speculative_frames(PhysicalSpeculativeWorkKind::WriteBehind, frames)
        .admit(std::num::NonZeroU64::MIN)
        .unwrap()
}

#[test]
fn foreign_incarnation_grant_is_denied_before_candidate_publication_activity() {
    let (governed_pool, candidate_clean) = owned_test_pool(106);
    let foreign_pool = test_pool(106);
    let foreign_allocation = publication_allocation(&foreign_pool);
    let counters = std::sync::Arc::new(CandidateFrameCounterCells::default());
    let publisher = BoundedCandidateFramePublisher::new(
        governed_pool.clone(),
        std::sync::Arc::clone(&counters),
        candidate_clean,
    );
    let before = governed_pool.counters();

    let denial = match publisher.begin(&foreign_allocation, &declared_inline_frames(&[(0, 3)])) {
        Ok(_) => panic!("foreign pool allocation opened candidate publication"),
        Err(denial) => denial,
    };

    let RecordAppendDenial::ResidencyUnavailable(failure) = denial else {
        panic!("foreign allocation must remain a residency failure");
    };
    assert_eq!(
        failure.kind(),
        crate::physical_runtime::PhysicalRecordResidencyFailureKind::AllocationAuthorityMismatch
    );
    assert_eq!(counters.submissions(), 0);
    assert_eq!(counters.declared_frames(), 0);
    assert_eq!(counters.declared_bytes(), 0);
    assert_eq!(counters.retained_frames(), 0);
    assert_eq!(counters.retained_bytes(), 0);
    let after = governed_pool.counters();
    assert_eq!(after.resident_bytes(), before.resident_bytes());
    assert_eq!(after.dirty_frames(), before.dirty_frames());
    assert_eq!(after.candidate_frames(), before.candidate_frames());
}

#[test]
fn undersized_live_grant_is_denied_before_candidate_projection_or_activity() {
    let (pool, candidate_clean) = owned_test_pool(107);
    let undersized = pool
        .begin_foreground_write_operation(std::num::NonZeroU64::MIN)
        .unwrap();
    let counters = std::sync::Arc::new(CandidateFrameCounterCells::default());
    let publisher = BoundedCandidateFramePublisher::new(
        pool.clone(),
        std::sync::Arc::clone(&counters),
        candidate_clean,
    );
    let before = pool.counters();

    let denial = match publisher.begin(&undersized, &declared_inline_frames(&[(0, 3)])) {
        Ok(_) => panic!("undersized operation authority opened candidate publication"),
        Err(denial) => denial,
    };

    let RecordAppendDenial::ResidencyUnavailable(failure) = denial else {
        panic!("undersized operation authority must remain a residency failure");
    };
    assert_eq!(
        failure.kind(),
        crate::physical_runtime::PhysicalRecordResidencyFailureKind::PhysicalPressure
    );
    assert_eq!(counters.submissions(), 0);
    assert_eq!(counters.declared_frames(), 0);
    assert_eq!(counters.declared_bytes(), 0);
    assert_eq!(counters.retained_frames(), 0);
    assert_eq!(counters.retained_bytes(), 0);
    let after = pool.counters();
    assert_eq!(after.candidate_frames(), before.candidate_frames());
    assert_eq!(
        after.candidate_publications(),
        before.candidate_publications()
    );
    assert_eq!(
        after.active_loading_frames(),
        before.active_loading_frames()
    );
}
