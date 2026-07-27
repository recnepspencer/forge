use super::*;

fn candidate(
    identity: StableStoreIdentity,
    artifact: RecordArtifactFile,
    offset: u64,
) -> PhysicalCandidateFrameKey {
    let coordinate = RecordFrameCoordinate::new(artifact, offset, 16).unwrap();
    PhysicalCandidateFrameKey::fragment(PhysicalFrameKey::new(identity, coordinate))
}

#[test]
fn empty_candidate_batch_has_contract_specific_denial() {
    let identity = store(33);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 2, candidate_batch_bytes(1), 4))
            .unwrap();
    let allocation = candidate_allocation(&pool, 1);

    assert_eq!(
        pool.reserve_candidate_frames(&allocation, &[]).unwrap_err(),
        PhysicalResidencyDenial::EmptyCandidateBatch
    );
    assert_eq!(pool.counters().frame_entries(), 0);
}

#[test]
fn duplicate_candidate_identity_does_not_impersonate_residency() {
    let identity = store(34);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 2, candidate_batch_bytes(2), 4))
            .unwrap();
    let allocation = candidate_allocation(&pool, 2);
    let frame = candidate(
        identity,
        RecordArtifactFile::RootRoutingBlock {
            generation: 1,
            block: 4,
        },
        0,
    );

    assert_eq!(
        pool.reserve_candidate_frames(&allocation, &[frame, frame])
            .unwrap_err(),
        PhysicalResidencyDenial::DuplicateCandidateIdentity
    );
    assert_eq!(pool.counters().frame_entries(), 0);
}

#[test]
fn conflicting_candidate_coverage_has_contract_specific_denial() {
    let identity = store(35);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 2, candidate_batch_bytes(2), 4))
            .unwrap();
    let allocation = candidate_allocation(&pool, 2);
    let artifact = RecordArtifactFile::RootManifest { generation: 4 };
    let complete_coordinate = RecordFrameCoordinate::new(artifact, 0, 16).unwrap();
    let complete = PhysicalCandidateFrameKey::complete_artifact(PhysicalFrameKey::new(
        identity,
        complete_coordinate,
    ))
    .unwrap();
    let fragment = candidate(identity, artifact, 16);

    assert_eq!(
        pool.reserve_candidate_frames(&allocation, &[complete, fragment])
            .unwrap_err(),
        PhysicalResidencyDenial::CandidateCoverageConflict
    );
    assert_eq!(pool.counters().frame_entries(), 0);
}

#[test]
fn candidate_sequence_conflict_preserves_the_admitted_batch() {
    let identity = store(36);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 2, candidate_batch_bytes(2), 4))
            .unwrap();
    let allocation = candidate_allocation(&pool, 2);
    let first = candidate(
        identity,
        RecordArtifactFile::RootRoutingBlock {
            generation: 1,
            block: 5,
        },
        0,
    );
    let second = candidate(
        identity,
        RecordArtifactFile::RootRoutingBlock {
            generation: 1,
            block: 6,
        },
        0,
    );
    let mut batch = pool
        .reserve_candidate_frames(&allocation, &[first, second])
        .unwrap();

    assert_eq!(
        batch.reserve_next(second).unwrap_err(),
        PhysicalResidencyDenial::CandidateSequenceConflict
    );
    let first_reservation = batch.reserve_next(first).unwrap();
    assert_eq!(first_reservation.key(), first.frame_key());
    drop(first_reservation);
    assert_eq!(pool.counters().frame_entries(), 0);
}

#[test]
fn failed_progression_keeps_batch_active_until_exact_drop_then_allows_retry() {
    let identity = store(37);
    let operation_bytes = candidate_batches_bytes(&[1, 1]);
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 1, 1, operation_bytes, 4)).unwrap();
    let allocation = candidate_batches_allocation(&pool, &[1, 1]);
    let first = candidate(
        identity,
        RecordArtifactFile::RootRoutingBlock {
            generation: 1,
            block: 7,
        },
        0,
    );
    let second = candidate(
        identity,
        RecordArtifactFile::RootRoutingBlock {
            generation: 1,
            block: 8,
        },
        0,
    );
    let mut first_batch = pool
        .reserve_candidate_frames(&allocation, &[first])
        .unwrap();

    assert_eq!(
        first_batch.reserve_next(second).unwrap_err(),
        PhysicalResidencyDenial::CandidateSequenceConflict
    );
    assert_eq!(
        pool.reserve_candidate_frames(&allocation, &[second])
            .unwrap_err(),
        PhysicalResidencyDenial::CandidatePublicationActive
    );

    drop(first_batch);
    let second_batch = pool
        .reserve_candidate_frames(&allocation, &[second])
        .unwrap();
    drop(second_batch);
    drop(allocation);
    assert_eq!(pool.counters().active_operation_bytes(), 0);
}
