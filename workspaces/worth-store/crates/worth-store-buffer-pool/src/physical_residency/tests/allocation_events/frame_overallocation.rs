use super::*;
use crate::physical_residency::frame_access::{
    PhysicalFrameAllocator, PhysicalFrameBuffer, PhysicalFrameFaultError,
};

struct OverallocatingFrameAllocator;

impl PhysicalFrameAllocator for OverallocatingFrameAllocator {
    fn allocate(&self, length: usize) -> Result<PhysicalFrameBuffer, ()> {
        Ok(PhysicalFrameBuffer::with_capacity(length, length * 2))
    }
}

#[test]
fn exact_fault_rejects_allocator_capacity_above_its_reservation_before_fill() {
    let identity = store(111);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 2, 1, 64, 3)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalFrameKey::new(identity, coordinate(1, 8));

    let failure = expect_fault(&pool, &allocation, key)
        .load_with_allocator(&OverallocatingFrameAllocator, |_| {
            panic!("over-allocation must fail before source fill")
        })
        .unwrap_err();

    assert_overallocation(failure, 8);
    assert_no_frame_residue(&pool);
    assert_reconciled(&pool, pool.allocation_events().snapshot());
}

#[test]
fn bounded_fault_rejects_capacity_above_its_admitted_limit_before_fill() {
    let identity = store(112);
    let pool = PhysicalResidencyPool::open(identity, limits(64, 2, 1, 64, 3)).unwrap();
    let allocation = allocation(&pool, READ_SCOPE);
    let key = PhysicalBoundedFrameKey::new(
        identity,
        RecordArtifactFile::RootRoutingBlock {
            generation: 1,
            block: 2,
        },
        NonZeroU32::new(8).unwrap(),
    );
    let owner = match pool.access_bounded_frame(&allocation, key).unwrap() {
        PhysicalBoundedFrameAccess::Fault(owner) => owner,
        _ => panic!("expected a bounded fault"),
    };

    let failure = owner
        .load_with_allocator(
            &OverallocatingFrameAllocator,
            |_| Ok::<_, ()>(8),
            |_| panic!("over-allocation must fail before source fill"),
        )
        .unwrap_err();

    assert_overallocation(failure, 8);
    assert_no_frame_residue(&pool);
    assert_reconciled(&pool, pool.allocation_events().snapshot());
}

fn assert_overallocation(failure: PhysicalFrameFaultError<()>, requested: u64) {
    let PhysicalFrameFaultError::Residency { denial, .. } = failure else {
        panic!("allocator overage must be a residency denial");
    };
    let PhysicalResidencyDenial::AllocatorExceededReservation {
        requested: found,
        actual,
    } = denial
    else {
        panic!("expected typed allocator overage, found {denial:?}");
    };
    assert_eq!(found, requested);
    assert!(actual > requested);
}

fn assert_no_frame_residue(pool: &PhysicalResidencyPool) {
    let counters = pool.counters();
    assert_eq!(counters.resident_bytes(), 0);
    assert_eq!(counters.frame_entries(), 0);
    assert_eq!(counters.pinned_frames(), 0);
    assert_eq!(counters.pin_leases(), 0);
    assert_eq!(counters.active_loading_frames(), 0);
}
