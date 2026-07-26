use super::*;

fn candidate(key: PhysicalFrameKey) -> PhysicalCandidateFrameKey {
    PhysicalCandidateFrameKey::fragment(key)
}

fn assert_identity_posture_unchanged(
    before: PhysicalResidencyCounters,
    after: PhysicalResidencyCounters,
) {
    assert_eq!(after.resident_bytes(), before.resident_bytes());
    assert_eq!(after.frame_entries(), before.frame_entries());
    assert_eq!(after.pinned_frames(), before.pinned_frames());
    assert_eq!(after.pin_leases(), before.pin_leases());
    assert_eq!(
        after.active_loading_frames(),
        before.active_loading_frames()
    );
    assert_eq!(after.source_loads(), before.source_loads());
    assert_eq!(after.denials(), before.denials() + 1);
}

#[test]
fn batch_admission_names_live_exact_loading_identity() {
    let identity = store(43);
    let operation_bytes = candidate_batch_bytes(1) + 1;
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 1, operation_bytes, 4)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let write = candidate_allocation(&pool, WRITE_SCOPE, 1);
    let key = PhysicalFrameKey::new(identity, coordinate(23, 32));
    let owner = expect_fault(&pool, &read, key);
    let before = pool.counters();

    assert_eq!(
        pool.reserve_candidate_frames(&write, &[candidate(key)])
            .unwrap_err(),
        PhysicalResidencyDenial::FrameIdentityOccupied
    );
    assert_identity_posture_unchanged(before, pool.counters());
    assert_eq!(&*owner.load(|bytes| fill(bytes, 23)).unwrap(), &[23; 32]);
}

#[test]
fn per_frame_admission_preserves_retained_exact_failure_terminal() {
    let identity = store(44);
    let operation_bytes = candidate_batch_bytes(1) + 1;
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 1, operation_bytes, 4)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let write = candidate_allocation(&pool, WRITE_SCOPE, 1);
    let key = PhysicalFrameKey::new(identity, coordinate(24, 32));
    let declaration = candidate(key);
    let mut batch = pool
        .reserve_candidate_frames(&write, &[declaration])
        .unwrap();
    let owner = expect_fault(&pool, &read, key);
    let waiter = match pool.access_frame(&read, key).unwrap() {
        PhysicalFrameAccess::Coalesced(waiter) => waiter,
        _ => panic!("overlapping exact access must retain one loading authority"),
    };
    let terminal = owner.reject_before_source();
    let before = pool.counters();

    assert_eq!(
        batch.reserve_next(declaration).unwrap_err(),
        PhysicalResidencyDenial::FrameLoadTerminated(terminal)
    );
    assert_identity_posture_unchanged(before, pool.counters());
    assert_eq!(waiter.wait().unwrap_err(), terminal);
    drop(batch);
    assert!(matches!(
        pool.access_frame(&read, key).unwrap(),
        PhysicalFrameAccess::Fault(_)
    ));
}

#[test]
fn bounded_loading_alias_is_not_called_an_exact_resident_frame() {
    let identity = store(45);
    let operation_bytes = candidate_batch_bytes(1) + 1;
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 1, operation_bytes, 4)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let write = candidate_allocation(&pool, WRITE_SCOPE, 1);
    let artifact = RecordArtifactFile::RootManifest { generation: 15 };
    let key = PhysicalFrameKey::new(
        identity,
        RecordFrameCoordinate::new(artifact, 0, 32).unwrap(),
    );
    let declaration = PhysicalCandidateFrameKey::complete_artifact(key).unwrap();
    let mut batch = pool
        .reserve_candidate_frames(&write, &[declaration])
        .unwrap();
    let bounded = PhysicalBoundedFrameKey::new(identity, artifact, NonZeroU32::new(64).unwrap());
    let owner = match pool.access_bounded_frame(&read, bounded).unwrap() {
        PhysicalBoundedFrameAccess::Fault(owner) => owner,
        _ => panic!("artifact must begin one bounded loading identity"),
    };
    let before = pool.counters();

    assert_eq!(
        batch.reserve_next(declaration).unwrap_err(),
        PhysicalResidencyDenial::ArtifactIdentityOccupied
    );
    assert_identity_posture_unchanged(before, pool.counters());
    assert_eq!(
        &*owner
            .load(|_| Ok::<_, ()>(32), |bytes| fill(bytes, 25))
            .unwrap(),
        &[25; 32]
    );
}

#[test]
fn actual_resident_collision_retains_residency_denial() {
    let identity = store(46);
    let operation_bytes = candidate_batch_bytes(1) + 1;
    let pool =
        PhysicalResidencyPool::open(identity, limits(256, 3, 1, operation_bytes, 4)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let write = candidate_allocation(&pool, WRITE_SCOPE, 1);
    let key = PhysicalFrameKey::new(identity, coordinate(26, 32));
    drop(
        expect_fault(&pool, &read, key)
            .load(|bytes| fill(bytes, 26))
            .unwrap(),
    );

    assert_eq!(
        pool.reserve_candidate_frames(&write, &[candidate(key)])
            .unwrap_err(),
        PhysicalResidencyDenial::FrameAlreadyResident
    );
}
