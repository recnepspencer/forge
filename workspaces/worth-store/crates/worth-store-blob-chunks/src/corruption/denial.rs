use worth_store_contracts::CorruptionHandoffDamageCase;

use crate::handoffs::reject_offline_handoff_as_blob_authority;
use crate::{
    BlobChunkOrdinal, BlobCorruptionCounterSnapshot, BlobCorruptionDetectionSource, BlobDamageCase,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForgeableCorruptionEvidenceKind {
    ChunkIntegrityReport,
    PhysicalQuarantineRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCorruptionDenial {
    CorruptOrdinalNotInPublishedFrontier {
        damage_case: BlobDamageCase,
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
        damage_case: BlobDamageCase,
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
    LowerPhysicalEvidenceRejected {
        damage_case: BlobDamageCase,
    },
    ForgeableCorruptionEvidenceRejected {
        evidence_kind: ForgeableCorruptionEvidenceKind,
    },
    StoreAuthorityReadmissionRejected,
    CopiedCountersRejected,
    RawDigestRejected,
    OfflineObservationRejected {
        damage_case: BlobDamageCase,
    },
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

pub const fn reject_offline_observation_as_blob_corruption_authority(
    handoff_case: CorruptionHandoffDamageCase,
) -> BlobCorruptionDenial {
    reject_offline_handoff_as_blob_authority(handoff_case)
}

pub const fn reject_chunk_integrity_report_as_blob_corruption_authority<T>(
    _: &T,
) -> BlobCorruptionDenial {
    BlobCorruptionDenial::ForgeableCorruptionEvidenceRejected {
        evidence_kind: ForgeableCorruptionEvidenceKind::ChunkIntegrityReport,
    }
}

pub const fn reject_physical_quarantine_record_as_blob_corruption_authority<T>(
    _: &T,
) -> BlobCorruptionDenial {
    BlobCorruptionDenial::ForgeableCorruptionEvidenceRejected {
        evidence_kind: ForgeableCorruptionEvidenceKind::PhysicalQuarantineRecord,
    }
}

pub const fn reject_copied_counters_as_blob_corruption_authority<T>(_: &T) -> BlobCorruptionDenial {
    BlobCorruptionDenial::CopiedCountersRejected
}

pub const fn reject_raw_digest_as_blob_corruption_authority<T>(_: &T) -> BlobCorruptionDenial {
    BlobCorruptionDenial::RawDigestRejected
}
