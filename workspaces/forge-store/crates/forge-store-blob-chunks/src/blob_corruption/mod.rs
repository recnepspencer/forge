mod authority;
mod classification;
mod counters;
mod denial;
mod downstream_denial;
mod guard;
mod localization;
mod quarantine;
mod reference_edges;

pub use authority::BlobQuarantineAuthority;
pub use classification::{
    AuthoritativeBlobCorruptionPosture, BlobCorruptionGenerationClassification,
    DerivedBlobCorruptionRebuildReadiness,
};
pub use counters::BlobCorruptionCounterSnapshot;
pub use denial::{
    reject_chunk_integrity_report_as_blob_corruption_authority,
    reject_copied_counters_as_blob_corruption_authority,
    reject_offline_observation_as_blob_corruption_authority,
    reject_physical_quarantine_record_as_blob_corruption_authority,
    reject_raw_digest_as_blob_corruption_authority, BlobCorruptionDenial,
    BlobCorruptionGuardDenial,
};
pub use downstream_denial::{
    BlobCorruptionCapsuleReadiness, BlobCorruptionCapsuleReadinessOutcome,
    BlobCorruptionExportAdmission, BlobCorruptionExportAdmissionOutcome,
    BlobCorruptionImportReadmission, BlobCorruptionImportReadmissionOutcome,
};
pub use guard::BlobCorruptionGuard;
pub use localization::{
    BlobCorruptedChunkLocalization, BlobCorruptionDetectionSource, BlobCorruptionPlacementClass,
    BlobCorruptionReferenceSharingScope,
};
pub use quarantine::{BlobChunkQuarantine, BlobQuarantineLifecycleState};
pub use reference_edges::{BlobCorruptionReferenceEdge, BlobCorruptionReferenceEdges};
