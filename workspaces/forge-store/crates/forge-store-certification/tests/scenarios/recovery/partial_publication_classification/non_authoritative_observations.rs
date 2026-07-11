use forge_store_recovery_physics::{BackendResidueKind, PartialPublicationObservationSet};

use crate::crash_edge_observations::after_durability_before_ack_edge;

pub(crate) fn replayable_wal_with_non_authoritative_observations(
    start: u64,
    end_exclusive: u64,
) -> PartialPublicationObservationSet {
    PartialPublicationObservationSet::new()
        .with_backend_residue(BackendResidueKind::StalePageImage, "phase8-stale-residue")
        .with_live_ack_memory("phase8-live-ack-memory")
        .with_log_only("phase8-operator-log")
        .with_persisted_crash_edge(after_durability_before_ack_edge(start, end_exclusive))
}
