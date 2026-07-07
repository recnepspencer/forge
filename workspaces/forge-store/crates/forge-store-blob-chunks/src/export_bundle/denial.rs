use crate::AdmittedBlobPlacement;

use super::counters::BlobExportBundleCounters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobExportBundleDenial {
    EmptyExportName { counters: BlobExportBundleCounters },
    StaleReachability { counters: BlobExportBundleCounters },
    PlacementLifecycleMismatch { counters: BlobExportBundleCounters },
    PlacementOnlyEvidenceRejected { counters: BlobExportBundleCounters },
    MissingChunk { counters: BlobExportBundleCounters },
    ChunkEvidenceMismatch { counters: BlobExportBundleCounters },
    CustodyNotExportReady { counters: BlobExportBundleCounters },
    TerminalProjectionRejected { counters: BlobExportBundleCounters },
    CopiedExportRowRejected { counters: BlobExportBundleCounters },
    CanonicalExportConstructionDenied { counters: BlobExportBundleCounters },
    CanonicalExportDigestDenied { counters: BlobExportBundleCounters },
}

impl BlobExportBundleDenial {
    pub const fn counters(&self) -> &BlobExportBundleCounters {
        match self {
            Self::EmptyExportName { counters }
            | Self::StaleReachability { counters }
            | Self::PlacementLifecycleMismatch { counters }
            | Self::PlacementOnlyEvidenceRejected { counters }
            | Self::MissingChunk { counters }
            | Self::ChunkEvidenceMismatch { counters }
            | Self::CustodyNotExportReady { counters }
            | Self::TerminalProjectionRejected { counters }
            | Self::CopiedExportRowRejected { counters }
            | Self::CanonicalExportConstructionDenied { counters }
            | Self::CanonicalExportDigestDenied { counters } => counters,
        }
    }
}

pub fn reject_terminal_projection_row_as_blob_export_bundle(_row: &str) -> BlobExportBundleDenial {
    BlobExportBundleDenial::TerminalProjectionRejected {
        counters: BlobExportBundleCounters::start().record_terminal_projection_denial(),
    }
}

pub fn reject_copied_export_row_as_blob_export_bundle(_row: &str) -> BlobExportBundleDenial {
    BlobExportBundleDenial::CopiedExportRowRejected {
        counters: BlobExportBundleCounters::start().record_copied_row_denial(),
    }
}

pub fn reject_placement_only_evidence_as_blob_export_bundle(
    _placement: &AdmittedBlobPlacement,
) -> BlobExportBundleDenial {
    BlobExportBundleDenial::PlacementOnlyEvidenceRejected {
        counters: BlobExportBundleCounters::start().record_placement_only_denial(),
    }
}
