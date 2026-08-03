use super::super::write_progression::CandidateFrameEffectFailure;
use std::sync::Arc;

use super::*;
use worth_store_buffer_pool::{
    PhysicalResidencyLimits, PhysicalResidencyPool, PhysicalResidencyPoolOwner,
};
use worth_store_physical_format::store_namespace::{
    ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
};

#[derive(Debug, PartialEq, Eq)]
struct ProvenNoEffect;

impl CandidateFrameEffectFailure for ProvenNoEffect {
    fn effect_fate(&self) -> crate::physical_runtime::PhysicalWorkEffectFate {
        crate::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EffectPossible;

impl CandidateFrameEffectFailure for EffectPossible {
    fn effect_fate(&self) -> crate::physical_runtime::PhysicalWorkEffectFate {
        crate::physical_runtime::PhysicalWorkEffectFate::Indeterminate
    }
}

#[test]
fn proven_no_effect_discards_the_resident_candidate_before_returning_failure() {
    let (pool, publisher) = pool_and_publisher(31);
    let allocation = publication_allocation(&pool);
    let mut session = StoreCandidateFramePublicationSession::begin(
        &publisher,
        &allocation,
        declared_inline_frames(&[(0, 3)]),
    )
    .unwrap();

    let failure = session
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::InlinePage,
                segment_coordinate(0),
                vec![1, 2, 3],
            ),
            &mut |_| Err::<CandidateFramePhysicalWrite, _>(ProvenNoEffect),
        )
        .unwrap_err();

    assert!(matches!(
        failure,
        CandidateFrameWriteFailure::Effect(ProvenNoEffect)
    ));
    let counters = pool.counters();
    assert_eq!(counters.resident_bytes(), 0);
    assert_eq!(counters.pinned_frames(), 0);
    assert_eq!(counters.pin_leases(), 0);
    assert_eq!(counters.dirty_frames(), 0);
    assert_eq!(counters.candidate_frames(), 0);
    assert_eq!(counters.administrative_drains(), 1);
    drop(session);
    drop(allocation);
    assert!(!pool.close().requires_inspection());
}

#[test]
fn effect_possible_failure_retains_dirty_candidate_for_inspection() {
    let (pool, publisher) = pool_and_publisher(32);
    let allocation = publication_allocation(&pool);
    let mut session = StoreCandidateFramePublicationSession::begin(
        &publisher,
        &allocation,
        declared_inline_frames(&[(0, 3)]),
    )
    .unwrap();

    let failure = session
        .write_frame(
            CandidateFrame::new(
                CandidateFrameRole::InlinePage,
                segment_coordinate(0),
                vec![4, 5, 6],
            ),
            &mut |_| Err::<CandidateFramePhysicalWrite, _>(EffectPossible),
        )
        .unwrap_err();

    assert!(matches!(
        failure,
        CandidateFrameWriteFailure::Effect(EffectPossible)
    ));
    let counters = pool.counters();
    assert_eq!(counters.resident_bytes(), 3);
    assert_eq!(counters.pinned_frames(), 0);
    assert_eq!(counters.pin_leases(), 0);
    assert_eq!(counters.dirty_frames(), 1);
    assert_eq!(counters.candidate_frames(), 1);
    assert_eq!(counters.administrative_drains(), 0);
    drop(session);
    drop(allocation);
    assert!(pool.close().requires_inspection());
}

fn pool_and_publisher(
    identity_byte: u8,
) -> (PhysicalResidencyPool, BoundedCandidateFramePublisher) {
    let store = StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([identity_byte; 16]).unwrap(),
    )
    .published_identity();
    let limits = residency_limits();
    let (pool, candidate_clean, _) = PhysicalResidencyPoolOwner::open(store, limits)
        .unwrap()
        .into_parts();
    let publisher = BoundedCandidateFramePublisher::new(
        pool.clone(),
        Arc::new(CandidateFrameCounterCells::default()),
        Arc::new(candidate_clean),
    );
    (pool, publisher)
}

fn residency_limits() -> PhysicalResidencyLimits {
    use worth_store_buffer_pool::{
        PhysicalOperationAllocationScope as Scope, PhysicalSpeculativeWorkKind as Speculation,
    };

    let operation =
        PhysicalResidencyPool::candidate_batch_operation_bytes(std::num::NonZeroUsize::MIN)
            .unwrap();
    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(12_288 + operation.get()))
        .resident_bytes(nonzero_bytes(4096))
        .metadata_bytes(nonzero_bytes(4096))
        .frame_entries(nonzero_count(4))
        .pinned_frames(nonzero_count(2))
        .pin_leases(nonzero_count(2))
        .dirty_frames(nonzero_count(2))
        .dirty_replacement_bytes(nonzero_bytes(4096))
        .operation_bytes(operation)
        .scope_bytes(Scope::ForegroundRead, operation)
        .scope_bytes(Scope::ForegroundWrite, operation)
        .scope_bytes(Scope::Recovery, operation)
        .scope_bytes(Scope::Scrub, operation)
        .scope_bytes(Scope::Maintenance, operation)
        .scope_bytes(Scope::Verification, operation)
        .scope_bytes(Scope::Blob, operation)
        .speculative_frames(Speculation::Prefetch, nonzero_count(2))
        .speculative_frames(Speculation::ReadAhead, nonzero_count(2))
        .speculative_frames(Speculation::WriteBehind, nonzero_count(2))
        .admit(std::num::NonZeroU64::MIN)
        .unwrap()
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}
