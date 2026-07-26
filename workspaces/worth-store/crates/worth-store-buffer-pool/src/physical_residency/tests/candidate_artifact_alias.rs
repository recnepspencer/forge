use super::*;

#[test]
fn complete_candidate_blocks_bounded_source_then_becomes_a_zero_source_hit() {
    let identity = store(27);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 1, candidate_batch_bytes(1), 4))
            .unwrap();
    let allocation = candidate_allocation(&pool, WRITE_SCOPE, 1);
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::RootManifest { generation: 7 }, 0, 32)
            .unwrap();
    let frame = PhysicalFrameKey::new(identity, coordinate);
    let candidate = PhysicalCandidateFrameKey::complete_artifact(frame).unwrap();
    let mut batch = pool
        .reserve_candidate_frames(&allocation, &[candidate])
        .unwrap();
    let dirty = batch
        .reserve_next(candidate)
        .unwrap()
        .admit(vec![6; 32])
        .unwrap();
    let bounded = PhysicalBoundedFrameKey::new(
        identity,
        coordinate.artifact(),
        NonZeroU32::new(64).unwrap(),
    );

    assert_eq!(
        pool.access_bounded_frame(&allocation, bounded).unwrap_err(),
        PhysicalResidencyDenial::CandidatePublicationActive
    );
    assert_eq!(pool.counters().source_loads(), 0);

    pool.inner.publish_clean(frame).unwrap();
    let hit = match pool.access_bounded_frame(&allocation, bounded).unwrap() {
        PhysicalBoundedFrameAccess::Hit(hit) => hit,
        _ => panic!("a clean complete candidate must satisfy a bounded access"),
    };
    assert_eq!(&*hit, &[6; 32]);
    assert_eq!(pool.counters().source_loads(), 0);
    drop((hit, dirty, batch));
}

#[test]
fn fragment_candidate_never_claims_complete_artifact_residency() {
    let identity = store(28);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 1, candidate_batch_bytes(1), 4))
            .unwrap();
    let allocation = candidate_allocation(&pool, WRITE_SCOPE, 1);
    let coordinate = RecordFrameCoordinate::new(
        RecordArtifactFile::Segment {
            segment: 8,
            generation: 2,
        },
        0,
        32,
    )
    .unwrap();
    let frame = PhysicalFrameKey::new(identity, coordinate);
    let candidate = PhysicalCandidateFrameKey::fragment(frame);
    let mut batch = pool
        .reserve_candidate_frames(&allocation, &[candidate])
        .unwrap();
    let dirty = batch
        .reserve_next(candidate)
        .unwrap()
        .admit(vec![7; 32])
        .unwrap();
    pool.inner.publish_clean(frame).unwrap();
    let bounded = PhysicalBoundedFrameKey::new(
        identity,
        coordinate.artifact(),
        NonZeroU32::new(64).unwrap(),
    );

    assert!(matches!(
        pool.access_bounded_frame(&allocation, bounded).unwrap(),
        PhysicalBoundedFrameAccess::Fault(_)
    ));
    assert_eq!(pool.counters().source_loads(), 0);
    drop((dirty, batch));
}

#[test]
fn abandoned_complete_candidate_releases_its_artifact_alias() {
    let identity = store(29);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 1, candidate_batch_bytes(1), 4))
            .unwrap();
    let allocation = candidate_allocation(&pool, WRITE_SCOPE, 1);
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::RootManifest { generation: 9 }, 0, 32)
            .unwrap();
    let frame = PhysicalFrameKey::new(identity, coordinate);
    let candidate = PhysicalCandidateFrameKey::complete_artifact(frame).unwrap();
    let mut batch = pool
        .reserve_candidate_frames(&allocation, &[candidate])
        .unwrap();
    let reservation = batch.reserve_next(candidate).unwrap();
    let bounded = PhysicalBoundedFrameKey::new(
        identity,
        coordinate.artifact(),
        NonZeroU32::new(64).unwrap(),
    );
    assert_eq!(
        pool.access_bounded_frame(&allocation, bounded).unwrap_err(),
        PhysicalResidencyDenial::CandidatePublicationActive
    );

    drop(reservation);
    assert!(matches!(
        pool.access_bounded_frame(&allocation, bounded).unwrap(),
        PhysicalBoundedFrameAccess::Fault(_)
    ));
    assert_eq!(pool.counters().source_loads(), 0);
    drop(batch);
}

#[test]
fn complete_artifact_candidate_requires_an_offset_zero_frame() {
    let identity = store(30);
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::RootManifest { generation: 10 }, 8, 32)
            .unwrap();
    let frame = PhysicalFrameKey::new(identity, coordinate);

    assert!(PhysicalCandidateFrameKey::complete_artifact(frame).is_none());
}
