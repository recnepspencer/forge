#[path = "../../../support/recovery/closeout/fixture.rs"]
mod closeout_fixture;
#[path = "../../../support/physical_isolation/reclaim_reachability_hazard_barriers/support.rs"]
mod reclaim_support;
#[allow(dead_code)]
#[path = "../../../support/physical_isolation/epoch_scope_and_root_kind/support.rs"]
mod support;

use forge_store_physical_isolation::{
    reject_backend_residue_as_reclaim_authority,
    reject_copied_read_plan_fields_as_reclaim_authority,
    reject_current_root_absence_as_reclaim_authority,
    reject_raw_reader_handle_scan_as_reclaim_authority, AllocatorPublicationReceipt,
    CrashStableReclaimReuseFence, ExecutedReachabilityEvidence, FreeReuseFenceDenial,
    GenerationAdvanceReceipt, HazardLeaseDenial, HazardLeaseKind, HazardLeaseTable,
    HazardLeaseTableCapacity, LeaseExpiryPosture, PhysicalOrderingContract, PhysicalOrderingSite,
    ReclaimDenial, ReclaimEligibilityProof,
};
use reclaim_support::{
    assert_reclaim_blocked_by_live_hazard, assert_reclaim_eligible_from_live_table,
    eligibility_after_releases, ReclaimFixture,
};
use support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_operation_digest_closeout,
};

#[test]
fn live_hazard_lease_blocks_reclaim_until_release() {
    let world = ReclaimFixture::new(7);
    let mut table =
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(4).unwrap());
    let active = table.acquire(world.root, world.lease.clone()).unwrap();
    let blocked =
        ReclaimEligibilityProof::admit(world.executed_reachability(), table.live_index_snapshot())
            .unwrap();

    assert!(matches!(
        blocked.try_reclaim(),
        Err(ReclaimDenial::BlockedByLiveHazardLease {
            kind: HazardLeaseKind::ForegroundRead,
            overlapping_ranges: 1,
            ..
        })
    ));
    assert_eq!(blocked.counters().blocked_reclaims(), 1);

    let release = table.release(active).unwrap();
    let eligible =
        ReclaimEligibilityProof::admit(world.executed_reachability(), table.live_index_snapshot())
            .unwrap();

    assert!(eligible.decision().is_eligible());
    assert_eq!(
        eligible
            .admit_reachability_removal()
            .unwrap()
            .counters()
            .eligible_reclaims(),
        1
    );
    assert_eq!(
        release.footprint_basis(),
        world.candidates.footprint_basis()
    );
}

#[test]
fn expiry_alone_is_not_reclaim_authority() {
    let released_world = ReclaimFixture::new(11);
    let revoked_world = ReclaimFixture::new(12);
    let owned_copy_world = ReclaimFixture::new(13);
    let mut table =
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(3).unwrap());
    let released_active = table
        .acquire(released_world.root, released_world.lease.clone())
        .unwrap();
    let revoked_active = table
        .acquire(revoked_world.root, revoked_world.lease.clone())
        .unwrap();
    let owned_copy_active = table
        .acquire(owned_copy_world.root, owned_copy_world.lease.clone())
        .unwrap();

    let expired = LeaseExpiryPosture::expired_without_authority(
        released_active.slot(),
        released_active.generation(),
    );
    assert_eq!(
        expired.require_reclaim_authority(),
        Err(
            HazardLeaseDenial::ExpiredLeaseWithoutReleaseRevocationOrOwnedCopy {
                slot: released_active.slot()
            }
        )
    );

    let release = table.release(released_active).unwrap();
    let released = LeaseExpiryPosture::from_release(release);
    assert_eq!(released.require_reclaim_authority(), Ok(()));
    assert_reclaim_eligible_from_live_table(&released_world, &table, 2);
    assert_reclaim_blocked_by_live_hazard(&revoked_world, &table, 2);
    assert_reclaim_blocked_by_live_hazard(&owned_copy_world, &table, 2);

    let revocation = table.revoke(revoked_active).unwrap();
    let revoked = LeaseExpiryPosture::from_revocation(revocation);
    assert_eq!(revoked.require_reclaim_authority(), Ok(()));
    assert_reclaim_eligible_from_live_table(&released_world, &table, 1);
    assert_reclaim_eligible_from_live_table(&revoked_world, &table, 1);
    assert_reclaim_blocked_by_live_hazard(&owned_copy_world, &table, 1);

    let owned_copy = table.convert_to_owned_copy(owned_copy_active).unwrap();
    let copied = LeaseExpiryPosture::from_owned_copy(owned_copy);
    assert_eq!(copied.require_reclaim_authority(), Ok(()));
    assert_eq!(table.counters().released_leases(), 1);
    assert_eq!(table.counters().revoked_leases(), 1);
    assert_eq!(table.counters().owned_copy_conversions(), 1);
    assert_reclaim_eligible_from_live_table(&released_world, &table, 0);
    assert_reclaim_eligible_from_live_table(&revoked_world, &table, 0);
    assert_reclaim_eligible_from_live_table(&owned_copy_world, &table, 0);
}

#[test]
fn stale_release_is_generation_counted_and_does_not_reopen_reclaim_authority() {
    let world = ReclaimFixture::new(13);
    let mut table =
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap());
    let active = table.acquire(world.root, world.lease.clone()).unwrap();

    table.release(active).unwrap();
    assert!(matches!(
        table.release(active),
        Err(HazardLeaseDenial::StaleLeaseGeneration { slot, observed, .. })
            if slot == active.slot() && observed == active.generation()
    ));
    assert_eq!(table.counters().released_leases(), 1);
    assert_eq!(table.counters().stale_release_denials(), 1);

    let proof =
        ReclaimEligibilityProof::admit(world.executed_reachability(), table.live_index_snapshot())
            .unwrap();
    assert!(proof.decision().is_eligible());
}

#[test]
fn reclaim_denies_non_executed_authority_shortcuts() {
    assert_eq!(
        reject_backend_residue_as_reclaim_authority(),
        ReclaimDenial::BackendResidueIsNotReachabilityAuthority
    );
    assert_eq!(
        reject_current_root_absence_as_reclaim_authority(),
        ReclaimDenial::CurrentRootAbsenceIsNotReachabilityAuthority
    );
    assert_eq!(
        reject_raw_reader_handle_scan_as_reclaim_authority(),
        ReclaimDenial::RawReaderHandleScanIsNotReachabilityAuthority
    );
    assert_eq!(
        reject_copied_read_plan_fields_as_reclaim_authority(),
        ReclaimDenial::CopiedReadPlanFieldsAreNotReachabilityAuthority
    );
}

#[test]
fn executed_reachability_denies_copied_candidate_sets() {
    let released_world = ReclaimFixture::new(41);
    let wrong_footprint_world = ReclaimFixture::new(42);
    assert_eq!(
        ExecutedReachabilityEvidence::from_released_old_reachability(
            released_world.released,
            wrong_footprint_world.candidates.clone(),
        )
        .map(|_| ()),
        Err(ReclaimDenial::CandidateDoesNotMatchExecutedReachability {
            executed: released_world.released.footprint_basis(),
            candidate: wrong_footprint_world.candidates.footprint_basis(),
        })
    );

    let wrong_root_world =
        ReclaimFixture::with_operation_digest_authority(41, "phase10-wrong-candidate-root");
    assert_eq!(
        ExecutedReachabilityEvidence::from_released_old_reachability(
            released_world.released,
            wrong_root_world.candidates.clone(),
        )
        .map(|_| ()),
        Err(
            ReclaimDenial::CandidateRootDoesNotMatchExecutedReachability {
                executed: released_world.released.release_receipt().root_epoch(),
                candidate: wrong_root_world.candidates.root_epoch(),
            }
        )
    );
}

#[test]
fn every_phase_ten_hazard_kind_blocks_reclaim_with_exact_overlap() {
    for kind in [
        HazardLeaseKind::ForegroundRead,
        HazardLeaseKind::ScrubWindow,
        HazardLeaseKind::RecoveryVerifier,
        HazardLeaseKind::CheckpointPreservation,
        HazardLeaseKind::QuarantineHold,
        HazardLeaseKind::FutureChunkHold,
        HazardLeaseKind::BufferPoolPin,
    ] {
        let world = ReclaimFixture::new(19);
        let mut table =
            HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap());
        table.acquire(world.root, world.lease_for(kind)).unwrap();
        let proof = ReclaimEligibilityProof::admit(
            world.executed_reachability(),
            table.live_index_snapshot(),
        )
        .unwrap();

        assert!(matches!(
            proof.try_reclaim(),
            Err(ReclaimDenial::BlockedByLiveHazardLease {
                kind: blocked,
                overlapping_ranges: 1,
                ..
            }) if blocked == kind
        ));
        assert_eq!(proof.counters().live_hazard_entries(), 1);
        assert_eq!(proof.counters().indexed_epoch_buckets_touched(), 1);
        assert_eq!(proof.counters().indexed_range_buckets_touched(), 1);
        assert_eq!(proof.counters().indexed_hazard_entries_touched(), 1);
        assert_eq!(proof.counters().overlapping_ranges(), 1);
    }
}

#[test]
fn nonmatching_epoch_hazard_range_is_not_scanned_or_blocking_authority() {
    let world = ReclaimFixture::new(31);
    let other_authority = physical_authority_from_operation_digest_closeout("phase10-other-root");
    let other_root = current_root_from_authority(&other_authority);
    let mut table =
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap());
    table.acquire(other_root, world.lease.clone()).unwrap();
    let proof =
        ReclaimEligibilityProof::admit(world.executed_reachability(), table.live_index_snapshot())
            .unwrap();

    assert!(proof.decision().is_eligible());
    assert_eq!(proof.counters().live_hazard_entries(), 1);
    assert_eq!(proof.counters().indexed_epoch_buckets_touched(), 0);
    assert_eq!(proof.counters().indexed_range_buckets_touched(), 0);
    assert_eq!(proof.counters().indexed_hazard_entries_touched(), 0);
    assert_eq!(proof.counters().hazard_lookup_ranges(), 0);
    assert_eq!(proof.counters().range_comparisons(), 0);
}

#[test]
fn same_epoch_nonoverlapping_hazard_range_is_indexed_out_before_entry_scan() {
    let candidate = ReclaimFixture::new(37);
    let unrelated = [ReclaimFixture::new(38), ReclaimFixture::new(39)];
    let mut table =
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(3).unwrap());
    for world in &unrelated {
        table.acquire(candidate.root, world.lease.clone()).unwrap();
    }
    let proof = ReclaimEligibilityProof::admit(
        candidate.executed_reachability(),
        table.live_index_snapshot(),
    )
    .unwrap();

    assert!(proof.decision().is_eligible());
    assert_eq!(proof.counters().live_hazard_entries(), 2);
    assert_eq!(proof.counters().indexed_epoch_buckets_touched(), 1);
    assert_eq!(proof.counters().indexed_range_buckets_touched(), 2);
    assert_eq!(proof.counters().indexed_hazard_entries_touched(), 0);
}

#[test]
fn release_order_does_not_change_reclaim_eligibility() {
    let first = ReclaimFixture::new(23);
    let second = ReclaimFixture::new(29);
    let forward = eligibility_after_releases([first.clone(), second.clone()]);
    let reverse = eligibility_after_releases([second, first]);

    assert_eq!(forward, reverse);
}

#[test]
fn free_reuse_requires_eligible_reclaim_generation_advance_and_allocator_publication() {
    let world = ReclaimFixture::new(17);
    let (blocked_proof, blocked_active) = {
        let mut table =
            HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap());
        let active = table.acquire(world.root, world.lease.clone()).unwrap();
        let proof = ReclaimEligibilityProof::admit(
            world.executed_reachability(),
            table.live_index_snapshot(),
        )
        .unwrap();
        (proof, active)
    };
    let old_identity = current_generation_page_reference(17);
    let reused_identity = current_generation_page_reference(18);
    let generation = GenerationAdvanceReceipt::from_identity_reuse(
        old_identity,
        reused_identity,
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::GenerationAdvancement),
    )
    .unwrap();
    let allocator = AllocatorPublicationReceipt::from_ordering(
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::AllocatorPublication),
    )
    .unwrap();

    assert_eq!(
        GenerationAdvanceReceipt::from_identity_reuse(
            old_identity,
            reused_identity,
            PhysicalOrderingContract::root_swap_acquire_release(),
        )
        .map(|_| ()),
        Err(FreeReuseFenceDenial::GenerationAdvancementOrderingNotCrashStable)
    );
    assert_eq!(
        AllocatorPublicationReceipt::from_ordering(
            PhysicalOrderingContract::root_swap_acquire_release()
        )
        .map(|_| ()),
        Err(FreeReuseFenceDenial::AllocatorPublicationOrderingNotCrashStable)
    );

    assert!(matches!(
        blocked_proof.admit_reachability_removal(),
        Err(ReclaimDenial::BlockedByLiveHazardLease {
            kind: HazardLeaseKind::ForegroundRead,
            overlapping_ranges: 1,
            slot,
            generation,
        }) if slot == blocked_active.slot() && generation == blocked_active.generation()
    ));

    let eligible = ReclaimEligibilityProof::admit(
        world.executed_reachability(),
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap())
            .live_index_snapshot(),
    )
    .unwrap();
    let removal = eligible.admit_reachability_removal().unwrap();
    let unrelated_generation = GenerationAdvanceReceipt::from_identity_reuse(
        current_generation_page_reference(18),
        current_generation_page_reference(19),
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::GenerationAdvancement),
    )
    .unwrap();
    assert_eq!(
        CrashStableReclaimReuseFence::admit_after_reclaim(
            removal.clone(),
            unrelated_generation,
            allocator
        )
        .map(|_| ()),
        Err(FreeReuseFenceDenial::ReclaimRemovalDoesNotCoverReusedIdentity)
    );
    let fence =
        CrashStableReclaimReuseFence::admit_after_reclaim(removal, generation, allocator).unwrap();

    assert_eq!(fence.generation_advance().old_identity(), old_identity);
    assert_eq!(
        fence.generation_advance().reused_identity(),
        reused_identity
    );
    assert_eq!(fence.reclaim_counters().eligible_reclaims(), 1);
    assert_eq!(
        fence.reachability_removal().evidence().root_epoch(),
        world.root.epoch()
    );
}
