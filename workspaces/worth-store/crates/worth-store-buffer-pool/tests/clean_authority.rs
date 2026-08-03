use worth_store_buffer_pool::{
    PhysicalFrameKey, PhysicalOperationAllocationScope, PhysicalResidencyDenial,
    PhysicalResidencyLimits, PhysicalResidencyPool, PhysicalResidencyPoolOwner,
    PhysicalSpeculativeWorkKind,
};
use worth_store_physical_format::{
    store_namespace::{
        ProposedStoreIdentity, StableStoreIdentity, StoreNamespaceIdentityRecord,
        StoreNamespaceVersion,
    },
    RecordArtifactFile, RecordFrameCoordinate,
};

#[test]
fn owner_capabilities_clean_candidate_and_writeback_transitions() {
    let (pool, candidate_clean, writeback_clean) = owned_pool(41);
    let allocation = foreground_write(&pool);

    let candidate_key = frame_key(pool.store_identity(), 1);
    let candidate = pool
        .materialize_dirty_candidate(&allocation, candidate_key, |bytes| bytes.fill(0x41))
        .unwrap();
    let clean_candidate = candidate
        .complete_candidate_publication(&candidate_clean)
        .unwrap();
    assert_eq!(pool.counters().dirty_frames(), 0);
    drop(clean_candidate);

    let writeback_key = frame_key(pool.store_identity(), 2);
    let dirty = pool
        .materialize_dirty_candidate(&allocation, writeback_key, |bytes| bytes.fill(0x42))
        .unwrap();
    writeback_claim(&pool, writeback_key)
        .unwrap()
        .complete_writeback(&writeback_clean)
        .unwrap();
    assert_eq!(pool.counters().dirty_frames(), 0);
    assert_eq!(pool.counters().active_writeback_claims(), 0);

    drop(dirty);
    drop(allocation);
    assert!(!pool.close().requires_inspection());
}

#[test]
fn foreign_candidate_authority_cannot_clean_an_identical_store_incarnation() {
    let (pool, _, _) = owned_pool(42);
    let (_, foreign_candidate_clean, _) = owned_pool(42);
    let allocation = foreground_write(&pool);
    let key = frame_key(pool.store_identity(), 1);
    let dirty = pool
        .materialize_dirty_candidate(&allocation, key, |bytes| bytes.fill(0x51))
        .unwrap();

    assert!(matches!(
        dirty.complete_candidate_publication(&foreign_candidate_clean),
        Err(PhysicalResidencyDenial::CandidateCleanAuthorityMismatch)
    ));
    let counters = pool.counters();
    assert_eq!(counters.dirty_frames(), 1);
    assert_eq!(counters.candidate_frames(), 1);
    assert_eq!(counters.pinned_frames(), 0);
    assert_eq!(counters.pin_leases(), 0);

    drop(allocation);
    assert!(pool.close().requires_inspection());
}

#[test]
fn foreign_writeback_authority_releases_the_claim_without_cleaning() {
    let (pool, _, _) = owned_pool(43);
    let (_, _, foreign_writeback_clean) = owned_pool(43);
    let allocation = foreground_write(&pool);
    let key = frame_key(pool.store_identity(), 1);
    let dirty = pool
        .materialize_dirty_candidate(&allocation, key, |bytes| bytes.fill(0x61))
        .unwrap();
    let claim = writeback_claim(&pool, key).unwrap();

    assert_eq!(
        claim.complete_writeback(&foreign_writeback_clean),
        Err(PhysicalResidencyDenial::WritebackCleanAuthorityMismatch)
    );
    let counters = pool.counters();
    assert_eq!(counters.dirty_frames(), 1);
    assert_eq!(counters.active_writeback_claims(), 0);

    drop(dirty);
    drop(allocation);
    assert!(pool.close().requires_inspection());
}

fn owned_pool(
    identity_byte: u8,
) -> (
    PhysicalResidencyPool,
    worth_store_buffer_pool::CandidateFrameCleanAuthority,
    worth_store_buffer_pool::FrameWritebackCleanAuthority,
) {
    PhysicalResidencyPoolOwner::open(store(identity_byte), residency_limits())
        .unwrap()
        .into_parts()
}

fn foreground_write(
    pool: &PhysicalResidencyPool,
) -> worth_store_buffer_pool::ForegroundWriteAllocationGrant {
    pool.begin_foreground_write_operation(
        PhysicalResidencyPool::candidate_batch_operation_bytes(std::num::NonZeroUsize::MIN)
            .unwrap(),
    )
    .unwrap()
}

fn writeback_claim(
    pool: &PhysicalResidencyPool,
    key: PhysicalFrameKey,
) -> Result<worth_store_buffer_pool::PhysicalWritebackClaim, PhysicalResidencyDenial> {
    let allocation = pool.begin_foreground_write_operation(
        std::num::NonZeroU64::new(u64::from(key.coordinate().length())).unwrap(),
    )?;
    pool.claim_writeback(allocation, &[key])
}

fn frame_key(store: StableStoreIdentity, ordinal: u64) -> PhysicalFrameKey {
    let artifact = RecordArtifactFile::RootManifest {
        generation: ordinal,
    };
    let coordinate = RecordFrameCoordinate::new(artifact, 0, 8).unwrap();
    PhysicalFrameKey::new(store, coordinate)
}

fn store(identity_byte: u8) -> StableStoreIdentity {
    StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([identity_byte; 16]).unwrap(),
    )
    .published_identity()
}

fn residency_limits() -> PhysicalResidencyLimits {
    let bytes = std::num::NonZeroU64::new(4096).unwrap();
    let candidate_operation =
        PhysicalResidencyPool::candidate_batch_operation_bytes(std::num::NonZeroUsize::MIN)
            .unwrap();
    let operation = std::num::NonZeroU64::new(candidate_operation.get() + 8).unwrap();
    let frames = std::num::NonZeroU32::new(4).unwrap();
    let mut limits = PhysicalResidencyLimits::builder()
        .total_bytes(std::num::NonZeroU64::new(12_288 + operation.get()).unwrap())
        .resident_bytes(bytes)
        .metadata_bytes(bytes)
        .frame_entries(frames)
        .pinned_frames(frames)
        .pin_leases(frames)
        .dirty_frames(frames)
        .dirty_replacement_bytes(bytes)
        .operation_bytes(operation);
    for scope in [
        PhysicalOperationAllocationScope::ForegroundRead,
        PhysicalOperationAllocationScope::ForegroundWrite,
        PhysicalOperationAllocationScope::Recovery,
        PhysicalOperationAllocationScope::Scrub,
        PhysicalOperationAllocationScope::Maintenance,
        PhysicalOperationAllocationScope::Verification,
        PhysicalOperationAllocationScope::Blob,
    ] {
        limits = limits.scope_bytes(scope, operation);
    }
    for kind in [
        PhysicalSpeculativeWorkKind::Prefetch,
        PhysicalSpeculativeWorkKind::ReadAhead,
        PhysicalSpeculativeWorkKind::WriteBehind,
    ] {
        limits = limits.speculative_frames(kind, frames);
    }
    limits.admit(std::num::NonZeroU64::MIN).unwrap()
}
