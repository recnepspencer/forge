use crate::BlobLifecycleCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobLifecycleDenial {
    CopiedDigestStringRejected,
    CopiedCounterSnapshotRejected {
        counters: BlobLifecycleCounterSnapshot,
    },
    S3IntegrityReportRejected,
    TerminalProjectionRowRejected,
    ImportedManifestTextRejected,
    S6PlacementSeedCarriesNoLifecycleAuthority,
    ReplayStoredChunkDigestMismatch {
        counters: BlobLifecycleCounterSnapshot,
    },
    DeclarationReachabilityDigestMismatch {
        counters: BlobLifecycleCounterSnapshot,
    },
    DeclarationPlacementDigestMismatch {
        counters: BlobLifecycleCounterSnapshot,
    },
    PlacementReachabilityBasisMismatch {
        counters: BlobLifecycleCounterSnapshot,
    },
}

pub const fn reject_copied_digest_string_as_lifecycle_receipt(_: &str) -> BlobLifecycleDenial {
    BlobLifecycleDenial::CopiedDigestStringRejected
}

pub const fn reject_copied_counters_as_lifecycle_receipt(
    counters: BlobLifecycleCounterSnapshot,
) -> BlobLifecycleDenial {
    BlobLifecycleDenial::CopiedCounterSnapshotRejected { counters }
}

pub const fn reject_physical_integrity_report_as_lifecycle_receipt<T>(
    _: &T,
) -> BlobLifecycleDenial {
    BlobLifecycleDenial::S3IntegrityReportRejected
}

pub const fn reject_terminal_projection_row_as_lifecycle_receipt<T>(_: &T) -> BlobLifecycleDenial {
    BlobLifecycleDenial::TerminalProjectionRowRejected
}

pub const fn reject_imported_manifest_text_as_lifecycle_receipt(_: &str) -> BlobLifecycleDenial {
    BlobLifecycleDenial::ImportedManifestTextRejected
}

pub fn reject_io_qos_placement_seed_as_lifecycle_receipt(_: impl Sized) -> BlobLifecycleDenial {
    BlobLifecycleDenial::S6PlacementSeedCarriesNoLifecycleAuthority
}
