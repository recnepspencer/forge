use forge_store_physical_isolation::{
    admit_seed_stable_read_plan, CompactProtectedReferenceSet, ExecutedReachabilityEvidence,
    HazardLeaseKind, HazardLeaseTable, HazardLeaseTableCapacity, PhysicalReadPlanReleaseSemantics,
    PhysicalReadReachabilityBarrier, PostProtectionPhysicalReadObservation,
    ProtectedPhysicalReferenceSet, ProtectedReferenceLease, PublishedReaderHazard,
    ReadPlanAdmissionScratchArena, ReclaimCandidateSet, ReclaimDenial, ReclaimEligibilityProof,
    ReleasedOldReachability, TraversalAdmissionGuard, UnprotectedReadIntent,
};

use crate::support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout, physical_authority_from_operation_digest_closeout,
};

#[derive(Clone)]
pub(crate) struct ReclaimFixture {
    pub(crate) root: forge_store_physical_isolation::CurrentPhysicalRoot,
    pub(crate) lease: ProtectedReferenceLease,
    pub(crate) barrier: PhysicalReadReachabilityBarrier,
    pub(crate) compact: CompactProtectedReferenceSet,
    pub(crate) candidates: ReclaimCandidateSet,
    pub(crate) released: ReleasedOldReachability,
}

impl ReclaimFixture {
    pub(crate) fn new(generation: u64) -> Self {
        let authority = physical_authority_from_complete_closeout();
        Self::from_authority(generation, &authority)
    }

    pub(crate) fn with_operation_digest_authority(generation: u64, digest: &str) -> Self {
        let authority = physical_authority_from_operation_digest_closeout(digest);
        Self::from_authority(generation, &authority)
    }

    pub(crate) fn lease_for(&self, kind: HazardLeaseKind) -> ProtectedReferenceLease {
        ProtectedReferenceLease::from_barrier(kind, self.barrier, self.compact.clone()).unwrap()
    }

    pub(crate) fn executed_reachability(&self) -> ExecutedReachabilityEvidence {
        ExecutedReachabilityEvidence::from_released_old_reachability(
            self.released,
            self.candidates.clone(),
        )
        .unwrap()
    }

    fn from_authority(
        generation: u64,
        authority: &forge_store_physical_isolation::PhysicalReadStabilityAuthority,
    ) -> Self {
        let root = current_root_from_authority(authority);
        let reference = current_generation_page_reference(generation);
        let references = ProtectedPhysicalReferenceSet::from_current_generation_refs_with_scratch(
            [reference],
            ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1),
        )
        .unwrap();
        let observed_references = references.clone();
        let compact = forge_store_physical_isolation::CompactProtectedReferenceSet::from_reference_set_with_scratch(
            references.clone(),
            ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1),
        )
        .unwrap();
        let intent = UnprotectedReadIntent::for_known_footprint(root, references, 4096)
            .with_release_semantics(PhysicalReadPlanReleaseSemantics::reader_releases_all());
        let hazard = PublishedReaderHazard::publish(authority, intent).unwrap();
        let lease = ProtectedReferenceLease::from_reader_hazard(&hazard, compact.clone()).unwrap();
        let barrier = hazard.reachability_barrier();
        let observed =
            PostProtectionPhysicalReadObservation::from_authority_after_hazard_publication(
                authority,
                &hazard,
                root,
                observed_references,
            )
            .unwrap();
        let validated = hazard
            .observe_authority_after_publication(authority, observed)
            .unwrap()
            .validate()
            .unwrap();
        let receipt = TraversalAdmissionGuard::from_validated_root(validated)
            .admit(ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1))
            .unwrap();
        let read_release = admit_seed_stable_read_plan(receipt.into_cursor().finish())
            .unwrap()
            .into_execution_ready_handle()
            .release();
        let released =
            forge_store_physical_isolation::OldReachabilityPreservation::from_protected_footprint(
                compact.declared_footprint_basis(),
            )
            .unwrap()
            .admit_release(read_release)
            .unwrap();
        let candidates =
            ReclaimCandidateSet::from_released_old_reachability(released, &compact).unwrap();
        Self {
            root,
            lease,
            barrier,
            compact,
            candidates,
            released,
        }
    }
}

pub(crate) fn eligibility_after_releases(worlds: [ReclaimFixture; 2]) -> Vec<bool> {
    let mut table =
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(2).unwrap());
    let first = table
        .acquire(worlds[0].root, worlds[0].lease.clone())
        .unwrap();
    let second = table
        .acquire(worlds[1].root, worlds[1].lease.clone())
        .unwrap();
    assert!(ReclaimEligibilityProof::admit(
        worlds[0].executed_reachability(),
        table.live_index_snapshot(),
    )
    .unwrap()
    .try_reclaim()
    .is_err());

    table.release(first).unwrap();
    let after_first = ReclaimEligibilityProof::admit(
        worlds[0].executed_reachability(),
        table.live_index_snapshot(),
    )
    .unwrap()
    .decision()
    .is_eligible();
    table.release(second).unwrap();
    let after_second = ReclaimEligibilityProof::admit(
        worlds[0].executed_reachability(),
        table.live_index_snapshot(),
    )
    .unwrap()
    .decision()
    .is_eligible();
    vec![after_first, after_second]
}

pub(crate) fn assert_reclaim_eligible_from_live_table(
    world: &ReclaimFixture,
    table: &HazardLeaseTable,
    expected_live_entries: u64,
) {
    let proof =
        ReclaimEligibilityProof::admit(world.executed_reachability(), table.live_index_snapshot())
            .unwrap();

    assert!(proof.decision().is_eligible());
    assert_eq!(
        proof.counters().live_hazard_entries(),
        expected_live_entries
    );
    assert_eq!(proof.counters().blocked_reclaims(), 0);
    assert_eq!(proof.counters().eligible_reclaims(), 1);
    assert_eq!(proof.counters().indexed_hazard_entries_touched(), 0);
    assert_eq!(proof.counters().overlapping_ranges(), 0);
}

pub(crate) fn assert_reclaim_blocked_by_live_hazard(
    world: &ReclaimFixture,
    table: &HazardLeaseTable,
    expected_live_entries: u64,
) {
    let proof =
        ReclaimEligibilityProof::admit(world.executed_reachability(), table.live_index_snapshot())
            .unwrap();

    assert!(matches!(
        proof.try_reclaim(),
        Err(ReclaimDenial::BlockedByLiveHazardLease {
            kind: HazardLeaseKind::ForegroundRead,
            overlapping_ranges: 1,
            ..
        })
    ));
    assert_eq!(
        proof.counters().live_hazard_entries(),
        expected_live_entries
    );
    assert_eq!(proof.counters().blocked_reclaims(), 1);
    assert_eq!(proof.counters().eligible_reclaims(), 0);
    assert_eq!(proof.counters().indexed_hazard_entries_touched(), 1);
    assert_eq!(proof.counters().overlapping_ranges(), 1);
}
