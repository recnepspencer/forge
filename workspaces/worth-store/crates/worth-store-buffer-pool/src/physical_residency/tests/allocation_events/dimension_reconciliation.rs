use super::*;

#[test]
fn independent_events_reconcile_every_active_hard_dimension() {
    let identity = store(101);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 3, 2, 64, 3)).unwrap();
    let observer = pool.allocation_events();
    let grant = pool
        .begin_foreground_write_operation(nonzero_bytes(16))
        .unwrap();
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let clean = expect_fault(&pool, &grant, key)
        .load(|target| {
            target.copy_from_slice(&[1; 8]);
            Ok::<_, ()>(())
        })
        .unwrap();

    assert_reconciled(&pool, observer.snapshot());
    let replacement = clean.begin_dirty_replacement(&grant).unwrap();
    assert_dimension(
        observer.snapshot(),
        PhysicalResidencyDimension::DirtyReplacementBytes,
        8,
    );
    assert_reconciled(&pool, observer.snapshot());

    let dirty = replacement
        .replace(|source, target| {
            assert_eq!(source, &[1; 8]);
            target.copy_from_slice(&[2; 8]);
            Ok::<_, ()>(())
        })
        .unwrap();
    assert_reconciled(&pool, observer.snapshot());
    dirty.discard_candidate().unwrap();
    drop(grant);
    assert_reconciled(&pool, observer.snapshot());
    let metadata = pool.counters().metadata_bytes();
    assert_dimension(
        observer.snapshot(),
        PhysicalResidencyDimension::TotalBytes,
        metadata,
    );
    drop(pool);
    assert_dimension(
        observer.snapshot(),
        PhysicalResidencyDimension::MetadataBytes,
        0,
    );
    assert_dimension(
        observer.snapshot(),
        PhysicalResidencyDimension::TotalBytes,
        0,
    );
}

#[test]
fn pressure_and_failed_fill_leave_no_phantom_admission() {
    let identity = store(102);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 3, 2, 8, 3)).unwrap();
    let observer = pool.allocation_events();
    let grant = pool
        .begin_foreground_write_operation(nonzero_bytes(8))
        .unwrap();
    let denial = pool
        .begin_operation(READ_SCOPE, nonzero_bytes(1))
        .unwrap_err();
    assert!(matches!(
        denial,
        PhysicalResidencyDenial::Pressure(pressure)
            if pressure.dimension() == PhysicalResidencyDimension::OperationBytes
    ));
    let operation_events = observer
        .snapshot()
        .for_dimension(PhysicalResidencyDimension::OperationBytes);
    assert_eq!(operation_events.attempts(), 2);
    assert_eq!(operation_events.admissions(), 1);
    assert_eq!(operation_events.denials(), 1);
    assert_eq!(operation_events.active_units(), 8);

    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let clean = expect_fault(&pool, &grant, key)
        .load(|target| {
            target.copy_from_slice(&[1; 8]);
            Ok::<_, ()>(())
        })
        .unwrap();
    let failure = clean
        .begin_dirty_replacement(&grant)
        .unwrap()
        .replace(|_, _| Err("fill rejected"))
        .unwrap_err();
    assert_eq!(
        failure,
        PhysicalDirtyReplacementError::Fill("fill rejected")
    );
    assert_eq!(pool.counters().dirty_replacement_bytes(), 0);
    assert_eq!(pool.counters().dirty_frames(), 0);
    assert_dimension(
        observer.snapshot(),
        PhysicalResidencyDimension::DirtyReplacementBytes,
        0,
    );
    assert_reconciled(&pool, observer.snapshot());
}
