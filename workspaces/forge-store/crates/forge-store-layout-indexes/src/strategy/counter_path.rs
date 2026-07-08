use super::counter_evidence::S8StrategyCounterEvidence;
use super::declaration::S8StrategyDeclaration;
use super::invariant_suite::S8StrategyCounterProfile;
use super::S8LayoutStrategyFamily;
use crate::execution::{
    S8AccessPathCounterSnapshot, S8AccessPathKind, S8PlannedVsObservedCounterReceipt,
};
use forge_store_physical_format::layout_access::baseline_btree_counter_observation::{
    execute_baseline_btree_lookup, execute_baseline_btree_replay_recovery,
    execute_baseline_btree_root_publication,
};
use forge_store_wal::layout_access::baseline_lsm_counter_observation::{
    execute_baseline_lsm_lookup, execute_baseline_lsm_manifest_publication,
    execute_baseline_lsm_replay,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum S8StrategyCounterLane {
    Lookup,
    Publication,
    Recovery,
}

pub(super) fn derive_strategy_counter_evidence(
    declaration: S8StrategyDeclaration,
) -> S8StrategyCounterEvidence {
    let lookup = lower_strategy_counter_lane(declaration.family(), S8StrategyCounterLane::Lookup);
    let publication =
        lower_strategy_counter_lane(declaration.family(), S8StrategyCounterLane::Publication);
    let recovery =
        lower_strategy_counter_lane(declaration.family(), S8StrategyCounterLane::Recovery);
    let aggregate = aggregate_profile(
        lookup.observed(),
        publication.observed(),
        recovery.observed(),
    );

    S8StrategyCounterEvidence::new(lookup, publication, recovery, aggregate)
}

fn lower_strategy_counter_lane(
    family: S8LayoutStrategyFamily,
    lane: S8StrategyCounterLane,
) -> S8PlannedVsObservedCounterReceipt {
    S8PlannedVsObservedCounterReceipt::new(
        path_kind_for(family, lane),
        planned_snapshot_for(family, lane),
        observed_snapshot_for(family, lane),
    )
}

fn observed_snapshot_for(
    family: S8LayoutStrategyFamily,
    lane: S8StrategyCounterLane,
) -> S8AccessPathCounterSnapshot {
    match (family, lane) {
        (S8LayoutStrategyFamily::BTree, S8StrategyCounterLane::Lookup) => {
            let counters = execute_baseline_btree_lookup().counters();
            S8AccessPathCounterSnapshot::new(
                counters.point_lookups(),
                counters.range_lookups(),
                0,
                counters.publications(),
                counters.maintenance_reads(),
            )
        }
        (S8LayoutStrategyFamily::BTree, S8StrategyCounterLane::Publication) => {
            let counters = execute_baseline_btree_root_publication().counters();
            S8AccessPathCounterSnapshot::new(
                counters.point_lookups(),
                counters.range_lookups(),
                0,
                counters.publications(),
                counters.maintenance_reads(),
            )
        }
        (S8LayoutStrategyFamily::BTree, S8StrategyCounterLane::Recovery) => {
            let counters = execute_baseline_btree_replay_recovery().counters();
            S8AccessPathCounterSnapshot::new(
                counters.point_lookups(),
                counters.range_lookups(),
                0,
                counters.publications(),
                counters.maintenance_reads(),
            )
        }
        (S8LayoutStrategyFamily::Lsm, S8StrategyCounterLane::Lookup) => {
            let counters = execute_baseline_lsm_lookup().counters();
            S8AccessPathCounterSnapshot::new(
                counters.point_lookups(),
                counters.range_lookups(),
                counters.wal_replays(),
                counters.publications(),
                counters.maintenance_reads(),
            )
        }
        (S8LayoutStrategyFamily::Lsm, S8StrategyCounterLane::Publication) => {
            let counters = execute_baseline_lsm_manifest_publication().counters();
            S8AccessPathCounterSnapshot::new(
                counters.point_lookups(),
                counters.range_lookups(),
                counters.wal_replays(),
                counters.publications(),
                counters.maintenance_reads(),
            )
        }
        (S8LayoutStrategyFamily::Lsm, S8StrategyCounterLane::Recovery) => {
            let counters = execute_baseline_lsm_replay().counters();
            S8AccessPathCounterSnapshot::new(
                counters.point_lookups(),
                counters.range_lookups(),
                counters.wal_replays(),
                counters.publications(),
                counters.maintenance_reads(),
            )
        }
        (S8LayoutStrategyFamily::ChunkTree, _) | (S8LayoutStrategyFamily::ExactScan, _) => {
            S8AccessPathCounterSnapshot::new(0, 0, 0, 0, 0)
        }
    }
}

const fn path_kind_for(
    family: S8LayoutStrategyFamily,
    lane: S8StrategyCounterLane,
) -> S8AccessPathKind {
    match (family, lane) {
        (S8LayoutStrategyFamily::BTree, S8StrategyCounterLane::Lookup) => {
            S8AccessPathKind::BaselineBTreePointLookup
        }
        (S8LayoutStrategyFamily::BTree, S8StrategyCounterLane::Publication) => {
            S8AccessPathKind::BaselineBTreeRootPublication
        }
        (S8LayoutStrategyFamily::BTree, S8StrategyCounterLane::Recovery) => {
            S8AccessPathKind::BaselineBTreeReplayRecovery
        }
        (S8LayoutStrategyFamily::Lsm, S8StrategyCounterLane::Lookup) => {
            S8AccessPathKind::BaselineLsmPointLookup
        }
        (S8LayoutStrategyFamily::Lsm, S8StrategyCounterLane::Publication) => {
            S8AccessPathKind::BaselineLsmManifestPublication
        }
        (S8LayoutStrategyFamily::Lsm, S8StrategyCounterLane::Recovery) => {
            S8AccessPathKind::BaselineLsmWalReplay
        }
        (S8LayoutStrategyFamily::ChunkTree, _) | (S8LayoutStrategyFamily::ExactScan, _) => {
            S8AccessPathKind::ExactForegroundRead
        }
    }
}

const fn planned_snapshot_for(
    family: S8LayoutStrategyFamily,
    lane: S8StrategyCounterLane,
) -> S8AccessPathCounterSnapshot {
    match (family, lane) {
        (S8LayoutStrategyFamily::BTree, S8StrategyCounterLane::Lookup) => {
            S8AccessPathCounterSnapshot::new(1, 1, 0, 0, 0)
        }
        (S8LayoutStrategyFamily::BTree, S8StrategyCounterLane::Publication) => {
            S8AccessPathCounterSnapshot::new(0, 0, 0, 1, 0)
        }
        (S8LayoutStrategyFamily::BTree, S8StrategyCounterLane::Recovery) => {
            S8AccessPathCounterSnapshot::new(0, 0, 0, 0, 1)
        }
        (S8LayoutStrategyFamily::Lsm, S8StrategyCounterLane::Lookup) => {
            S8AccessPathCounterSnapshot::new(1, 1, 0, 0, 0)
        }
        (S8LayoutStrategyFamily::Lsm, S8StrategyCounterLane::Publication) => {
            S8AccessPathCounterSnapshot::new(0, 0, 0, 2, 2)
        }
        (S8LayoutStrategyFamily::Lsm, S8StrategyCounterLane::Recovery) => {
            S8AccessPathCounterSnapshot::new(0, 0, 1, 0, 1)
        }
        (S8LayoutStrategyFamily::ChunkTree, _) | (S8LayoutStrategyFamily::ExactScan, _) => {
            S8AccessPathCounterSnapshot::new(0, 0, 0, 0, 0)
        }
    }
}

const fn aggregate_profile(
    lookup: S8AccessPathCounterSnapshot,
    publication: S8AccessPathCounterSnapshot,
    recovery: S8AccessPathCounterSnapshot,
) -> S8StrategyCounterProfile {
    S8StrategyCounterProfile::new(
        lookup.point_lookups() + publication.point_lookups() + recovery.point_lookups(),
        lookup.range_lookups() + publication.range_lookups() + recovery.range_lookups(),
        lookup.wal_replays() + publication.wal_replays() + recovery.wal_replays(),
        lookup.publications() + publication.publications() + recovery.publications(),
        lookup.maintenance_reads() + publication.maintenance_reads() + recovery.maintenance_reads(),
    )
}
