use super::super::write_progression::CandidateFrameEffectFailure;
use super::*;

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
    let mut session =
        StoreCandidateFramePublicationSession::begin(&publisher, declared_inline_frames(&[(0, 3)]))
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
    assert!(!pool.close().requires_inspection());
}

#[test]
fn effect_possible_failure_retains_dirty_candidate_for_inspection() {
    let (pool, publisher) = pool_and_publisher(32);
    let mut session =
        StoreCandidateFramePublicationSession::begin(&publisher, declared_inline_frames(&[(0, 3)]))
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
    let limits =
        PhysicalResidencyLimits::new_with_metadata_budget(4096, 4096, 2, 2, 64, 4).unwrap();
    let pool = PhysicalResidencyPool::open(store, limits).unwrap();
    let publisher = BoundedCandidateFramePublisher::new(
        pool.clone(),
        Arc::new(CandidateFrameCounterCells::default()),
    );
    (pool, publisher)
}
