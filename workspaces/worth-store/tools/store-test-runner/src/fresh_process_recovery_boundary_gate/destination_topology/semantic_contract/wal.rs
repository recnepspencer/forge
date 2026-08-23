use super::DestinationTopologyContract;

pub(super) const DESTINATIONS: &[DestinationTopologyContract] = &[
    DestinationTopologyContract::new(
        "crates/worth-store-wal/src/artifact_store/segment_inventory/segment_inspection/denial.rs",
        "wal/artifact-store/segment-inventory/segment-inspection/denial",
        "preallocation-wal-frame-limit-evidence",
        "no-runtime-or-physics-import",
        "phase-3",
        "create",
    ),
    DestinationTopologyContract::new(
        "crates/worth-store-wal/src/artifact_store/segment_inventory/segment_inspection/owned_frame.rs",
        "wal/artifact-store/segment-inventory/segment-inspection/owned-frame",
        "verified-wal-frame-ownership",
        "no-runtime-or-physics-import",
        "phase-4",
        "create",
    ),
    DestinationTopologyContract::new(
        "crates/worth-store-wal/src/recovery_read/mod.rs",
        "wal/recovery-read",
        "recovery-read-admission",
        "no-runtime-or-physics-import",
        "phase-3",
        "preserve",
    ),
    DestinationTopologyContract::new(
        "crates/worth-store-wal/src/wal_topology/mod.rs",
        "wal/wal-topology",
        "wal-topology-primitives",
        "no-runtime-or-physics-import",
        "phase-3",
        "preserve",
    ),
];
