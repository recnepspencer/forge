use crate::AdmittedBlobPlacement;

use super::counters::BlobImportReadmissionCounters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobImportReadmissionDenial {
    ImportedJsonRejected {
        counters: BlobImportReadmissionCounters,
    },
    CopiedExportRowRejected {
        counters: BlobImportReadmissionCounters,
    },
    TerminalProjectionRejected {
        counters: BlobImportReadmissionCounters,
    },
    StaleKeyGeneration {
        counters: BlobImportReadmissionCounters,
    },
    WrongTenantAuthority {
        counters: BlobImportReadmissionCounters,
    },
    CustodyDomainMismatch {
        counters: BlobImportReadmissionCounters,
    },
    MissingChunk {
        counters: BlobImportReadmissionCounters,
    },
    ChunkEvidenceMismatch {
        counters: BlobImportReadmissionCounters,
    },
    PlacementOnlyEvidenceRejected {
        counters: BlobImportReadmissionCounters,
    },
}

impl BlobImportReadmissionDenial {
    pub const fn counters(&self) -> &BlobImportReadmissionCounters {
        match self {
            Self::ImportedJsonRejected { counters }
            | Self::CopiedExportRowRejected { counters }
            | Self::TerminalProjectionRejected { counters }
            | Self::StaleKeyGeneration { counters }
            | Self::WrongTenantAuthority { counters }
            | Self::CustodyDomainMismatch { counters }
            | Self::MissingChunk { counters }
            | Self::ChunkEvidenceMismatch { counters }
            | Self::PlacementOnlyEvidenceRejected { counters } => counters,
        }
    }
}

pub const fn reject_copied_export_row_as_blob_import(_: &str) -> BlobImportReadmissionDenial {
    BlobImportReadmissionDenial::CopiedExportRowRejected {
        counters: BlobImportReadmissionCounters::start().record_copied_row_denial(),
    }
}

pub const fn reject_placement_only_evidence_as_imported_blob_witness(
    _: &AdmittedBlobPlacement,
) -> BlobImportReadmissionDenial {
    BlobImportReadmissionDenial::PlacementOnlyEvidenceRejected {
        counters: BlobImportReadmissionCounters::start().record_placement_only_denial(),
    }
}
