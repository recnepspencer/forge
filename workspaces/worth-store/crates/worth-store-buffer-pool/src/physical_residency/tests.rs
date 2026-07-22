use super::*;
use std::sync::{Arc, Barrier};
use worth_store_physical_format::{
    store_namespace::{
        ProposedStoreIdentity, StableStoreIdentity, StoreNamespaceIdentityRecord,
        StoreNamespaceVersion,
    },
    RecordArtifactFile, RecordFrameCoordinate,
};

#[path = "tests/c6_readiness.rs"]
mod c6_readiness;
#[path = "tests/metadata_admission.rs"]
mod metadata_admission;
#[path = "tests/shutdown.rs"]
mod shutdown;

fn store(byte: u8) -> StableStoreIdentity {
    StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([byte; 16]).unwrap(),
    )
    .published_identity()
}

fn coordinate(block: u64, length: u32) -> RecordFrameCoordinate {
    RecordFrameCoordinate::new(
        RecordArtifactFile::RootRoutingBlock {
            generation: 1,
            block,
        },
        0,
        length,
    )
    .unwrap()
}

fn limits(
    resident_bytes: u64,
    pinned_frames: u32,
    dirty_frames: u32,
    operation_bytes: u64,
    frame_entries: u32,
) -> PhysicalResidencyLimits {
    PhysicalResidencyLimits::new_with_metadata_budget(
        resident_bytes,
        4096,
        pinned_frames,
        dirty_frames,
        operation_bytes,
        frame_entries,
    )
    .unwrap()
}

fn fill(bytes: &mut [u8], value: u8) -> Result<(), ()> {
    bytes.fill(value);
    Ok(())
}

#[test]
fn hot_load_reuses_bytes_and_leases_pin_independently() {
    let identity = store(1);
    let pool = PhysicalResidencyPool::open(identity, limits(1024, 2, 1, 64, 4)).unwrap();
    let key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let first = pool
        .load(key, |target| {
            target.fill(7);
            Ok::<_, ()>(())
        })
        .unwrap();
    let second = pool
        .load(key, |_| -> Result<(), ()> {
            panic!("a hot frame must not invoke its source")
        })
        .unwrap();
    assert_eq!(&*first, &[7; 32]);
    assert_eq!(&*second, &[7; 32]);
    assert_eq!(pool.counters().pinned_frames(), 1);
    assert_eq!(pool.counters().pin_leases(), 2);
    let denial = pool.load(key, |_| Ok::<_, ()>(())).unwrap_err();
    assert_eq!(
        denial,
        PhysicalFrameLoadError::Residency(PhysicalResidencyDenial::PinLeaseBudgetExceeded,)
    );
    drop((first, second));
    assert_eq!(pool.counters().pinned_frames(), 0);
    assert_eq!(pool.counters().pin_leases(), 0);
}

#[test]
fn stable_store_identity_prevents_cross_store_aliases() {
    let owner = store(2);
    let pool = PhysicalResidencyPool::open(owner, limits(1024, 2, 1, 64, 4)).unwrap();
    let foreign = PhysicalFrameKey::new(store(3), coordinate(1, 16));
    let denial = pool.load(foreign, |_| Ok::<_, ()>(())).unwrap_err();
    assert_eq!(
        denial,
        PhysicalFrameLoadError::Residency(PhysicalResidencyDenial::WrongStore)
    );
    assert_eq!(pool.counters().resident_bytes(), 0);
}

#[test]
fn clean_unpinned_frames_evict_and_refault_under_real_pressure() {
    let identity = store(4);
    let pool = PhysicalResidencyPool::open(identity, limits(1000, 2, 1, 64, 2)).unwrap();
    for block in 1..=3 {
        let key = PhysicalFrameKey::new(identity, coordinate(block, 300));
        let lease = pool
            .load(key, |target| {
                target.fill(block as u8);
                Ok::<_, ()>(())
            })
            .unwrap();
        drop(lease);
    }
    let counters = pool.counters();
    assert_eq!(counters.evictions(), 1);
    assert_eq!(counters.eviction_candidate_inspections(), 1);
    assert!(counters.metadata_bytes() <= 4096);
    assert!(counters.resident_bytes() <= 1000);
    assert_eq!(counters.peak_resident_bytes(), 600);
}

#[test]
fn candidate_dirty_posture_and_operation_allocations_are_exact() {
    let identity = store(5);
    let pool = PhysicalResidencyPool::open(identity, limits(1024, 2, 1, 40, 4)).unwrap();
    let key = PhysicalFrameKey::new(identity, coordinate(1, 20));
    let dirty = pool.admit_dirty(key, vec![9; 20]).unwrap();
    assert_eq!(pool.counters().dirty_frames(), 1);
    let grant = pool
        .begin_operation(OperationAllocationScope::ForegroundWrite, 40)
        .unwrap();
    assert_eq!(grant.bytes(), 40);
    assert_eq!(
        pool.begin_operation(OperationAllocationScope::ForegroundRead, 1)
            .unwrap_err(),
        PhysicalResidencyDenial::OperationBudgetExceeded,
    );
    let clean = dirty.publish_clean_for_pool_test().unwrap();
    drop((clean, grant));
    let counters = pool.counters();
    assert_eq!(counters.dirty_frames(), 0);
    assert_eq!(counters.writebacks(), 0);
    assert_eq!(counters.candidate_publications(), 1);
    assert_eq!(counters.active_operation_bytes(), 0);
    assert!(!pool.close().requires_inspection());
}

#[test]
fn unrelated_faults_do_not_hold_the_metadata_lock_during_io() {
    let identity = store(6);
    let pool = Arc::new(PhysicalResidencyPool::open(identity, limits(2048, 2, 1, 64, 4)).unwrap());
    let barrier = Arc::new(Barrier::new(2));
    let mut workers = Vec::new();
    for block in 1..=2 {
        let pool = Arc::clone(&pool);
        let barrier = Arc::clone(&barrier);
        workers.push(std::thread::spawn(move || {
            let key = PhysicalFrameKey::new(identity, coordinate(block, 32));
            pool.load(key, |target| {
                barrier.wait();
                target.fill(block as u8);
                Ok::<_, ()>(())
            })
            .unwrap()
        }));
    }
    let leases: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();
    assert_eq!(leases.len(), 2);
    assert_eq!(pool.counters().source_loads(), 2);
}

#[test]
fn concurrent_same_coordinate_faults_once_and_share_one_frame() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let identity = store(7);
    let pool = Arc::new(PhysicalResidencyPool::open(identity, limits(2048, 2, 1, 64, 4)).unwrap());
    let key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let fills = Arc::new(AtomicUsize::new(0));
    let first_pool = Arc::clone(&pool);
    let first_fills = Arc::clone(&fills);
    let first = std::thread::spawn(move || {
        first_pool
            .load(key, |target| {
                first_fills.fetch_add(1, Ordering::SeqCst);
                target.fill(11);
                Ok::<_, ()>(())
            })
            .unwrap()
    });
    let second_pool = Arc::clone(&pool);
    let second_fills = Arc::clone(&fills);
    let second = std::thread::spawn(move || {
        second_pool
            .load(key, |target| {
                second_fills.fetch_add(1, Ordering::SeqCst);
                target.fill(22);
                Ok::<_, ()>(())
            })
            .unwrap()
    });
    let first = first.join().unwrap();
    let second = second.join().unwrap();
    assert_eq!(fills.load(Ordering::SeqCst), 1);
    assert_eq!(&*first, &*second);
    assert_eq!(pool.counters().faults(), 1);
    assert_eq!(pool.counters().hits(), 1);
}

#[test]
fn panicking_source_releases_loading_reservation_and_pin_budget() {
    let identity = store(8);
    let pool = PhysicalResidencyPool::open(identity, limits(1024, 1, 1, 64, 4)).unwrap();
    let key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = pool.load::<(), _>(key, |_| panic!("source panic"));
    }));
    assert!(panic.is_err());
    assert_eq!(pool.counters().pinned_frames(), 0);
    let recovered = pool
        .load(key, |target| {
            target.fill(3);
            Ok::<_, ()>(())
        })
        .unwrap();
    assert_eq!(&*recovered, &[3; 32]);
}

#[test]
fn close_during_source_io_cancels_the_unpublished_frame() {
    let identity = store(9);
    let pool = Arc::new(PhysicalResidencyPool::open(identity, limits(1024, 1, 1, 64, 4)).unwrap());
    let key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let entered = Arc::new(Barrier::new(2));
    let release = Arc::new(Barrier::new(2));
    let worker_pool = Arc::clone(&pool);
    let worker_entered = Arc::clone(&entered);
    let worker_release = Arc::clone(&release);
    let worker = std::thread::spawn(move || {
        worker_pool.load(key, |target| {
            worker_entered.wait();
            worker_release.wait();
            target.fill(4);
            Ok::<_, ()>(())
        })
    });
    entered.wait();
    let closing_pool = Arc::clone(&pool);
    let closer = std::thread::spawn(move || closing_pool.close());
    loop {
        match pool.begin_operation(OperationAllocationScope::ForegroundRead, 1) {
            Err(PhysicalResidencyDenial::PoolClosed) => break,
            Ok(grant) => drop(grant),
            Err(other) => panic!("unexpected close probe denial: {other:?}"),
        }
        std::thread::yield_now();
    }
    release.wait();
    assert_eq!(
        worker.join().unwrap().unwrap_err(),
        PhysicalFrameLoadError::Residency(PhysicalResidencyDenial::PoolClosed),
    );
    let shutdown = closer.join().unwrap();
    assert!(!shutdown.requires_inspection());
    assert_eq!(pool.counters().pinned_frames(), 0);
    assert_eq!(pool.counters().resident_bytes(), 0);
}

#[test]
fn pinned_frames_are_never_selected_for_eviction() {
    let identity = store(10);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 2, 1, 64, 3)).unwrap();
    let first_key = PhysicalFrameKey::new(identity, coordinate(1, 32));
    let second_key = PhysicalFrameKey::new(identity, coordinate(2, 32));
    let first = pool.load(first_key, |bytes| fill(bytes, 1)).unwrap();
    let second = pool.load(second_key, |bytes| fill(bytes, 2)).unwrap();
    let third_key = PhysicalFrameKey::new(identity, coordinate(3, 32));
    assert_eq!(
        pool.load(third_key, |_| Ok::<_, ()>(())).unwrap_err(),
        PhysicalFrameLoadError::Residency(PhysicalResidencyDenial::PinnedFrameBudgetExceeded)
    );
    assert_eq!(&*first, &[1; 32]);
    assert_eq!(&*second, &[2; 32]);
    drop(first);
    let third = pool.load(third_key, |bytes| fill(bytes, 3)).unwrap();
    assert_eq!(&*third, &[3; 32]);
    assert_eq!(pool.counters().evictions(), 1);
    assert_eq!(pool.counters().eviction_candidate_inspections(), 1);
}

#[test]
fn catalog_publication_moves_the_cached_identity_atomically() {
    let identity = store(11);
    let pool = PhysicalResidencyPool::open(identity, limits(128, 2, 1, 64, 3)).unwrap();
    let old = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 4).unwrap();
    let candidate = RecordFrameCoordinate::new(
        RecordArtifactFile::CatalogCandidate { publication: 2 },
        0,
        4,
    )
    .unwrap();
    let old_key = PhysicalFrameKey::new(identity, old);
    let candidate_key = PhysicalFrameKey::new(identity, candidate);
    drop(pool.load(old_key, |bytes| fill(bytes, 1)).unwrap());
    let dirty = pool.admit_dirty(candidate_key, vec![9; 4]).unwrap();
    drop(dirty.publish_clean_for_pool_test().unwrap());
    pool.promote_clean_identity(candidate_key, old_key).unwrap();
    let current = pool
        .load(old_key, |_| -> Result<(), ()> {
            panic!("published catalog must be hot")
        })
        .unwrap();
    assert_eq!(&*current, &[9; 4]);
    drop(current);
    let vanished = pool.load(candidate_key, |bytes| fill(bytes, 7)).unwrap();
    assert_eq!(&*vanished, &[7; 4]);
    assert_eq!(pool.counters().identity_transitions(), 1);
    assert_eq!(pool.counters().source_loads(), 2);
}

#[test]
fn speculative_work_uses_live_pool_limits_and_dirty_posture() {
    let identity = store(12);
    let policy = limits(128, 2, 1, 64, 3)
        .with_speculative_frame_limits(1, 1, 1)
        .unwrap();
    let pool = PhysicalResidencyPool::open(identity, policy).unwrap();
    let prefetch = pool
        .begin_speculative(crate::SpeculativePhysicalWorkKind::Prefetch, 1)
        .unwrap();
    assert_eq!(
        pool.begin_speculative(crate::SpeculativePhysicalWorkKind::Prefetch, 1)
            .unwrap_err(),
        PhysicalResidencyDenial::SpeculativeFrameBudgetExceeded
    );
    drop(prefetch);
    assert_eq!(
        pool.begin_speculative(crate::SpeculativePhysicalWorkKind::WriteBehind, 1)
            .unwrap_err(),
        PhysicalResidencyDenial::WriteBackExceedsDirtyPosture
    );
    let dirty = pool
        .admit_dirty(
            PhysicalFrameKey::new(identity, coordinate(1, 16)),
            vec![5; 16],
        )
        .unwrap();
    let writeback = pool
        .begin_speculative(crate::SpeculativePhysicalWorkKind::WriteBehind, 1)
        .unwrap();
    assert_eq!(writeback.frames(), 1);
    drop(writeback);
    drop(dirty.publish_clean_for_pool_test().unwrap());
    let counters = pool.counters();
    assert_eq!(
        counters.speculative_attempts(crate::SpeculativePhysicalWorkKind::Prefetch),
        2
    );
    assert_eq!(
        counters.speculative_admissions(crate::SpeculativePhysicalWorkKind::Prefetch),
        1
    );
}
