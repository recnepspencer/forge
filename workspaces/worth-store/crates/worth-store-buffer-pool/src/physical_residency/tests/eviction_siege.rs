use super::*;

struct IneligibleVictimWorld<'grant> {
    pinned: PhysicalFrameLease,
    loading: PhysicalFrameFaultOwner,
    candidate_batch: PhysicalCandidateBatchReservation<'grant>,
    candidate: PhysicalCandidateFrameReservation,
    claim: PhysicalWritebackClaim,
}

impl<'grant> IneligibleVictimWorld<'grant> {
    fn install(
        pool: &PhysicalResidencyPool,
        read: &OperationAllocationGrant,
        write: &'grant ForegroundWriteAllocationGrant,
        identity: StableStoreIdentity,
    ) -> Self {
        let pinned_key = PhysicalFrameKey::new(identity, coordinate(1, 16));
        let dirty_key = PhysicalFrameKey::new(identity, coordinate(2, 16));
        let loading_key = PhysicalFrameKey::new(identity, coordinate(3, 16));
        let candidate_key = PhysicalFrameKey::new(identity, coordinate(4, 16));
        let candidate_declaration = PhysicalCandidateFrameKey::fragment(candidate_key);
        let claimed_key = PhysicalFrameKey::new(identity, coordinate(5, 16));

        let pinned = expect_fault(pool, read, pinned_key)
            .load(|target| {
                target.fill(1);
                Ok::<_, ()>(())
            })
            .unwrap();
        let dirty = pool
            .materialize_dirty_candidate(write, dirty_key, |bytes| bytes.fill(2))
            .unwrap();
        drop(dirty);
        let loading = expect_fault(pool, read, loading_key);
        let mut candidate_batch = pool
            .reserve_candidate_frames(write, &[candidate_declaration])
            .unwrap();
        let candidate = candidate_batch.reserve_next(candidate_declaration).unwrap();
        let claimed_dirty = pool
            .materialize_dirty_candidate(write, claimed_key, |bytes| bytes.fill(5))
            .unwrap();
        let claim = writeback_claim(pool, &[claimed_key]);
        drop(claimed_dirty);

        Self {
            pinned,
            loading,
            candidate_batch,
            candidate,
            claim,
        }
    }

    fn release(self) {
        let Self {
            pinned,
            loading,
            candidate_batch,
            candidate,
            claim,
        } = self;
        drop(claim);
        drop(candidate);
        drop(candidate_batch);
        drop(loading);
        drop(pinned);
    }

    fn assert_installed(&self, pool: &PhysicalResidencyPool, expected_entries: u32) {
        let counters = pool.counters();
        assert_eq!(counters.frame_entries(), expected_entries);
        // The ordinary lease, loading owner, and reserved candidate each
        // consume one pin. Every admitted dirty frame remains candidate-origin,
        // while the typed candidate handle proves the one CandidateReserved
        // state. The two resident dirty frames are independently unpinned, and
        // exactly one of them is writeback-claimed.
        assert_eq!(counters.pinned_frames(), 3);
        assert_eq!(counters.pin_leases(), 3);
        assert_eq!(counters.dirty_frames(), 3);
        assert_eq!(counters.active_loading_frames(), 2);
        assert_eq!(counters.candidate_frames(), 3);
        assert_eq!(counters.active_writeback_claims(), 1);
    }
}

#[test]
fn every_nominal_victim_ineligible_denies_before_fault_or_source_load() {
    let identity = store(31);
    let operation_bytes = candidate_batches_bytes(&[1, 1]) + 17;
    let pool = PhysicalResidencyPool::open(identity, limits(96, 5, 4, operation_bytes, 5)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let write = candidate_batches_allocation(&pool, &[1, 1]);
    let world = IneligibleVictimWorld::install(&pool, &read, &write, identity);
    world.assert_installed(&pool, 5);
    let before = pool.counters();
    let incoming = PhysicalFrameKey::new(identity, coordinate(6, 16));

    assert_eq!(
        pool.access_frame(&read, incoming).unwrap_err(),
        PhysicalResidencyDenial::Pressure(PhysicalResidencyPressureDenial::new(
            identity,
            pool.incarnation(),
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::FrameEntries,
                scope: READ_SCOPE,
                requested: 1,
                current: 5,
                limit: 5,
            },
        ))
    );

    let after = pool.counters();
    assert_eq!(after.frame_entries(), before.frame_entries());
    assert_eq!(after.resident_bytes(), before.resident_bytes());
    assert_eq!(after.faults(), before.faults());
    assert_eq!(after.source_loads(), before.source_loads());
    assert_eq!(after.evictions(), before.evictions());
    assert_eq!(
        after.eviction_candidate_inspections(),
        before.eviction_candidate_inspections() + 1
    );

    world.release();
    drop(read);
    drop(write);
    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    assert_eq!(shutdown.counters().dirty_frames(), 2);
    assert_eq!(shutdown.counters().active_writeback_claims(), 0);
    assert_eq!(shutdown.counters().pin_leases(), 0);
}

#[test]
fn sole_legal_victim_releases_exactly_and_refaults_through_fault_ownership() {
    let identity = store(32);
    let operation_bytes = candidate_batches_bytes(&[1, 1]) + 17;
    let pool =
        PhysicalResidencyPool::open(identity, limits(112, 6, 4, operation_bytes, 6)).unwrap();
    let read = allocation(&pool, READ_SCOPE);
    let write = candidate_batches_allocation(&pool, &[1, 1]);
    let world = IneligibleVictimWorld::install(&pool, &read, &write, identity);
    let legal_key = PhysicalFrameKey::new(identity, coordinate(6, 16));
    let legal = expect_fault(&pool, &read, legal_key)
        .load(|target| {
            target.fill(9);
            Ok::<_, ()>(())
        })
        .unwrap();
    drop(legal);
    world.assert_installed(&pool, 6);
    let before = pool.counters();
    let incoming_key = PhysicalFrameKey::new(identity, coordinate(7, 16));

    let incoming = expect_fault(&pool, &read, incoming_key)
        .load(|target| {
            target.fill(7);
            Ok::<_, ()>(())
        })
        .unwrap();
    let after_incoming = pool.counters();
    assert_eq!(after_incoming.evictions(), before.evictions() + 1);
    assert_eq!(
        after_incoming.eviction_candidate_inspections(),
        before.eviction_candidate_inspections() + 1
    );
    assert_eq!(after_incoming.frame_entries(), before.frame_entries());
    assert_eq!(after_incoming.resident_bytes(), before.resident_bytes());
    assert_eq!(after_incoming.faults(), before.faults() + 1);
    assert_eq!(after_incoming.source_loads(), before.source_loads() + 1);
    assert_eq!(&*incoming, &[7; 16]);
    drop(incoming);

    let refault = expect_fault(&pool, &read, legal_key)
        .load(|target| {
            target.fill(9);
            Ok::<_, ()>(())
        })
        .unwrap();
    let after_refault = pool.counters();
    assert_eq!(&*refault, &[9; 16]);
    assert_eq!(after_refault.evictions(), before.evictions() + 2);
    assert_eq!(
        after_refault.eviction_candidate_inspections(),
        before.eviction_candidate_inspections() + 2
    );
    assert_eq!(after_refault.frame_entries(), before.frame_entries());
    assert_eq!(after_refault.resident_bytes(), before.resident_bytes());
    assert_eq!(after_refault.faults(), before.faults() + 2);
    assert_eq!(after_refault.source_loads(), before.source_loads() + 2);
    drop(refault);

    world.release();
    drop(read);
    drop(write);
    let shutdown = pool.close();
    assert!(shutdown.requires_inspection());
    assert_eq!(shutdown.counters().dirty_frames(), 2);
    assert_eq!(shutdown.counters().active_writeback_claims(), 0);
    assert_eq!(shutdown.counters().pin_leases(), 0);
}
