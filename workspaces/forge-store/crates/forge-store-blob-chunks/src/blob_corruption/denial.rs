use crate::{BlobChunkOrdinal, BlobCorruptionCounterSnapshot, BlobCorruptionDetectionSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCorruptionDenial {
    CorruptOrdinalNotInPublishedFrontier {
        ordinal: BlobChunkOrdinal,
        counters: BlobCorruptionCounterSnapshot,
    },
    EmptyAffectedReferenceEdges {
        counters: BlobCorruptionCounterSnapshot,
    },
    DuplicateAffectedReferenceEdge {
        counters: BlobCorruptionCounterSnapshot,
    },
    GenerationFrontierMismatch {
        counters: BlobCorruptionCounterSnapshot,
    },
    AffectedReferenceEdgeMismatch {
        counters: BlobCorruptionCounterSnapshot,
    },
    DerivedRebuildRequiresDerivedBlob {
        counters: BlobCorruptionCounterSnapshot,
    },
    AuthoritativeRepairRequiresAuthoritativeBlob {
        counters: BlobCorruptionCounterSnapshot,
    },
    LowerPhysicalEvidenceRejected,
    CopiedCountersRejected,
    RawDigestRejected,
    OfflineObservationRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCorruptionGuardDenial {
    DedupeDenied {
        source: BlobCorruptionDetectionSource,
        counters: BlobCorruptionCounterSnapshot,
    },
    ExportDenied {
        source: BlobCorruptionDetectionSource,
        counters: BlobCorruptionCounterSnapshot,
    },
    ImportReadmissionDenied {
        source: BlobCorruptionDetectionSource,
        counters: BlobCorruptionCounterSnapshot,
    },
    CapsuleReadinessDenied {
        source: BlobCorruptionDetectionSource,
        counters: BlobCorruptionCounterSnapshot,
    },
    VerifiedReadPublicationDenied {
        source: BlobCorruptionDetectionSource,
        counters: BlobCorruptionCounterSnapshot,
    },
    ReclaimDenied {
        source: BlobCorruptionDetectionSource,
        counters: BlobCorruptionCounterSnapshot,
    },
    CompactionMovementDenied {
        source: BlobCorruptionDetectionSource,
        counters: BlobCorruptionCounterSnapshot,
    },
}

pub const fn reject_chunk_integrity_report_as_blob_corruption_authority<T>(
    _: &T,
) -> BlobCorruptionDenial {
    BlobCorruptionDenial::LowerPhysicalEvidenceRejected
}

pub const fn reject_physical_quarantine_record_as_blob_corruption_authority<T>(
    _: &T,
) -> BlobCorruptionDenial {
    BlobCorruptionDenial::LowerPhysicalEvidenceRejected
}

pub const fn reject_copied_counters_as_blob_corruption_authority<T>(_: &T) -> BlobCorruptionDenial {
    BlobCorruptionDenial::CopiedCountersRejected
}

pub const fn reject_raw_digest_as_blob_corruption_authority<T>(_: &T) -> BlobCorruptionDenial {
    BlobCorruptionDenial::RawDigestRejected
}

pub const fn reject_offline_observation_as_blob_corruption_authority<T>(
    _: &T,
) -> BlobCorruptionDenial {
    BlobCorruptionDenial::OfflineObservationRejected
}
