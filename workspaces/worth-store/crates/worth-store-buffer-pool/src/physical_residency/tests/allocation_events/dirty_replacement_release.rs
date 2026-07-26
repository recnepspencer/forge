use super::*;

#[test]
fn allocator_failure_after_grant_releases_replacement_authority_before_fill() {
    struct RejectingAllocator;

    impl super::super::super::lease::dirty_replacement_allocation::DirtyReplacementAllocator
        for RejectingAllocator
    {
        fn allocate(&self, _length: usize) -> Result<Vec<u8>, ()> {
            Err(())
        }
    }

    let identity = store(103);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 3, 2, 64, 3)).unwrap();
    let observer = pool.allocation_events();
    let grant = pool.begin_operation(WRITE_SCOPE, nonzero_bytes(8)).unwrap();
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let clean = load_clean(&pool, &grant, key, 1);

    let failure = clean
        .begin_dirty_replacement(&grant)
        .unwrap()
        .replace_with_allocator(&RejectingAllocator, |_, _| -> Result<(), ()> {
            panic!("allocator rejection must happen before fill")
        })
        .unwrap_err();
    assert_eq!(
        failure,
        PhysicalDirtyReplacementError::Residency(PhysicalResidencyDenial::AllocationFailed)
    );
    let events = observer.snapshot();
    let replacement_events =
        events.for_dimension(PhysicalResidencyDimension::DirtyReplacementBytes);
    assert_eq!(replacement_events.admissions(), 1);
    assert_eq!(replacement_events.allocator_failures(), 1);
    assert_eq!(replacement_events.releases(), 1);
    assert_eq!(
        events
            .for_dimension(PhysicalResidencyDimension::TotalBytes)
            .allocator_failures(),
        1
    );
    assert_clean_replacement_posture(&pool, events);
    let clean = load_clean_hit(&pool, &grant, key);
    assert_eq!(clean.as_ref(), &[1; 8]);
}

#[test]
fn panic_during_fill_unwinds_every_replacement_grant_and_preserves_source() {
    let identity = store(104);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 3, 2, 64, 3)).unwrap();
    let observer = pool.allocation_events();
    let grant = pool.begin_operation(WRITE_SCOPE, nonzero_bytes(8)).unwrap();
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let clean = load_clean(&pool, &grant, key, 1);

    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = clean
            .begin_dirty_replacement(&grant)
            .unwrap()
            .replace::<(), _>(|_, _| panic!("hostile fill unwind"));
    }));
    assert!(unwind.is_err());
    assert_clean_replacement_posture(&pool, observer.snapshot());
    let clean = load_clean_hit(&pool, &grant, key);
    assert_eq!(clean.as_ref(), &[1; 8]);
}

#[test]
fn held_replacement_denies_one_past_its_exact_byte_ceiling() {
    let identity = store(105);
    let pool = PhysicalResidencyPool::open(identity, replacement_ceiling_limits()).unwrap();
    let observer = pool.allocation_events();
    let grant = pool
        .begin_operation(WRITE_SCOPE, nonzero_bytes(16))
        .unwrap();
    let first_key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let second_key = PhysicalFrameKey::new(identity, coordinate(2, 8));
    let first = load_clean(&pool, &grant, first_key, 1);
    let second = load_clean(&pool, &grant, second_key, 2);

    let held = first.begin_dirty_replacement(&grant).unwrap();
    let denial = second.begin_dirty_replacement(&grant).unwrap_err();
    assert!(matches!(
        denial,
        PhysicalResidencyDenial::Pressure(pressure)
            if pressure.dimension() == PhysicalResidencyDimension::DirtyReplacementBytes
                && pressure.requested() == 8
                && pressure.current() == 8
                && pressure.limit() == 8
    ));
    assert_dimension(
        observer.snapshot(),
        PhysicalResidencyDimension::DirtyReplacementBytes,
        8,
    );

    drop(held);
    assert_clean_replacement_posture(&pool, observer.snapshot());
    let second = load_clean_hit(&pool, &grant, second_key);
    let dirty = second
        .begin_dirty_replacement(&grant)
        .unwrap()
        .replace(|source, target| {
            assert_eq!(source, &[2; 8]);
            target.copy_from_slice(&[3; 8]);
            Ok::<_, ()>(())
        })
        .unwrap();
    assert_eq!(dirty.bytes(), &[3; 8]);
}

fn load_clean(
    pool: &PhysicalResidencyPool,
    grant: &OperationAllocationGrant,
    key: PhysicalFrameKey,
    value: u8,
) -> PhysicalFrameLease {
    expect_fault(pool, grant, key)
        .load(|target| {
            target.fill(value);
            Ok::<_, ()>(())
        })
        .unwrap()
}

fn load_clean_hit(
    pool: &PhysicalResidencyPool,
    grant: &OperationAllocationGrant,
    key: PhysicalFrameKey,
) -> PhysicalFrameLease {
    expect_hit(pool, grant, key)
}

fn assert_clean_replacement_posture(
    pool: &PhysicalResidencyPool,
    events: PhysicalResidencyAllocationEventSnapshot,
) {
    assert_eq!(pool.counters().dirty_replacement_bytes(), 0);
    assert_eq!(pool.counters().dirty_frames(), 0);
    assert_dimension(events, PhysicalResidencyDimension::DirtyReplacementBytes, 0);
    assert_reconciled(pool, events);
}

fn replacement_ceiling_limits() -> PhysicalResidencyLimits {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Kind;

    let mut builder = PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(8192))
        .resident_bytes(nonzero_bytes(16))
        .metadata_bytes(nonzero_bytes(4096))
        .frame_entries(nonzero_count(2))
        .pinned_frames(nonzero_count(2))
        .pin_leases(nonzero_count(2))
        .dirty_frames(nonzero_count(2))
        .dirty_replacement_bytes(nonzero_bytes(8))
        .operation_bytes(nonzero_bytes(16));
    for scope in [
        Scope::ForegroundRead,
        Scope::ForegroundWrite,
        Scope::Recovery,
        Scope::Scrub,
        Scope::Maintenance,
        Scope::Verification,
        Scope::Blob,
    ] {
        builder = builder.scope_bytes(scope, nonzero_bytes(16));
    }
    for kind in [Kind::ReadAhead, Kind::Prefetch, Kind::WriteBehind] {
        builder = builder.speculative_frames(kind, nonzero_count(2));
    }
    builder.admit(NonZeroU64::MIN).unwrap()
}
