use super::*;
use crate::physical_residency::lease::candidate_allocation::{
    CandidateFrameAllocator, CandidateFrameBuffer,
};

struct FailingCandidateAllocator;

impl CandidateFrameAllocator for FailingCandidateAllocator {
    fn allocate(&self, _length: usize) -> Result<CandidateFrameBuffer, ()> {
        Err(())
    }
}

#[test]
fn candidate_materialization_exposes_only_the_exact_admitted_slice() {
    let identity = store(108);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 2, 2, candidate_batch_bytes(1), 2))
        .unwrap();
    let allocation = candidate_allocation(&pool, 1);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let candidate = PhysicalCandidateFrameKey::fragment(key);
    let mut batch = pool
        .reserve_candidate_frames(&allocation, &[candidate])
        .unwrap();
    let mut exposed_length = 0;

    let dirty = batch
        .reserve_next(candidate)
        .unwrap()
        .materialize(|bytes| {
            exposed_length = bytes.len();
            bytes.fill(0xA5);
        })
        .unwrap();

    assert_eq!(exposed_length, 8);
    assert_eq!(dirty.bytes(), &[0xA5; 8]);
    dirty.discard_candidate().unwrap();
    drop(batch);
    drop(allocation);
    assert_reconciled(&pool, pool.allocation_events().snapshot());
}

#[test]
fn candidate_allocator_failure_releases_every_reserved_dimension_before_fill() {
    let identity = store(109);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 2, 2, candidate_batch_bytes(1), 2))
        .unwrap();
    let observer = pool.allocation_events();
    let allocation = candidate_allocation(&pool, 1);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let candidate = PhysicalCandidateFrameKey::fragment(key);
    let mut batch = pool
        .reserve_candidate_frames(&allocation, &[candidate])
        .unwrap();

    let failure = batch
        .reserve_next(candidate)
        .unwrap()
        .materialize_with_allocator(&FailingCandidateAllocator, |_| {
            panic!("allocator rejection must happen before candidate fill")
        })
        .unwrap_err();

    assert_eq!(failure, PhysicalResidencyDenial::AllocationFailed);
    assert_candidate_reservation_released(&pool);
    let events = observer.snapshot();
    for dimension in [
        PhysicalResidencyDimension::ResidentBytes,
        PhysicalResidencyDimension::TotalBytes,
    ] {
        let dimension = events.for_dimension(dimension);
        assert_eq!(dimension.allocator_failures(), 1);
        assert_eq!(dimension.released_units(), 8);
    }
    drop(batch);
    let mut retry = pool
        .reserve_candidate_frames(&allocation, &[candidate])
        .unwrap();
    let dirty = retry
        .reserve_next(candidate)
        .unwrap()
        .materialize(|bytes| bytes.fill(7))
        .unwrap();
    assert_eq!(dirty.bytes(), &[7; 8]);
    dirty.discard_candidate().unwrap();
    drop(retry);
    drop(allocation);
    assert_reconciled(&pool, observer.snapshot());
}

#[test]
fn panic_during_candidate_fill_releases_the_exact_reservation() {
    let identity = store(110);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 2, 2, candidate_batch_bytes(1), 2))
        .unwrap();
    let observer = pool.allocation_events();
    let allocation = candidate_allocation(&pool, 1);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));
    let candidate = PhysicalCandidateFrameKey::fragment(key);
    let mut batch = pool
        .reserve_candidate_frames(&allocation, &[candidate])
        .unwrap();

    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = batch
            .reserve_next(candidate)
            .unwrap()
            .materialize(|_| panic!("hostile candidate fill panic"));
    }));

    assert!(unwind.is_err());
    assert_candidate_reservation_released(&pool);
    let resident = observer
        .snapshot()
        .for_dimension(PhysicalResidencyDimension::ResidentBytes);
    assert_eq!(resident.allocator_failures(), 0);
    assert_eq!(resident.admitted_units(), resident.released_units());
    drop(batch);
    drop(allocation);
    assert_reconciled(&pool, observer.snapshot());
}

fn assert_candidate_reservation_released(pool: &PhysicalResidencyPool) {
    let counters = pool.counters();
    assert_eq!(counters.resident_bytes(), 0);
    assert_eq!(counters.frame_entries(), 0);
    assert_eq!(counters.pinned_frames(), 0);
    assert_eq!(counters.pin_leases(), 0);
    assert_eq!(counters.dirty_frames(), 0);
    assert_eq!(counters.candidate_frames(), 0);
    assert_eq!(counters.active_loading_frames(), 0);
}
