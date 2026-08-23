pub(super) const PRESERVED_WAL_DESTINATION_SURFACES: &[(&str, &str, &str)] = &[
    (
        "AdmittedReplayTailCursor",
        "worth-store-wal/recovery-read",
        "phase-3",
    ),
    (
        "LogSequenceNumber",
        "worth-store-wal/wal-topology",
        "phase-3",
    ),
    ("WalLsnRange", "worth-store-wal/wal-topology", "phase-3"),
    (
        "WalSegmentGeneration",
        "worth-store-wal/wal-topology",
        "phase-3",
    ),
    ("WalSegmentId", "worth-store-wal/wal-topology", "phase-3"),
    (
        "WalSegmentArtifactIdentity",
        "worth-store-wal/artifact-store/segment-inventory",
        "phase-3",
    ),
    (
        "WalSegmentInspection",
        "worth-store-wal/artifact-store/segment-inventory/segment-inspection",
        "phase-3",
    ),
];
