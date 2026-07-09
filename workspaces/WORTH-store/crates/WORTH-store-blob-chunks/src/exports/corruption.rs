// --- Capabilities (admission handles, next-step types) ---
pub use crate::corruption::{
    AuthoritativeBlobCorruptionPosture, BlobChunkQuarantine, BlobCorruptionExportAdmission,
    BlobCorruptionGuard, BlobCorruptionImportReadmission, BlobQuarantineAuthority,
    DerivedBlobCorruptionRebuildReadiness, PhysicalCorruptionHandoffClassification,
};
// --- Outcomes (transition receipts) ---
pub use crate::corruption::{
    classify_blob_damage_before_decode, classify_generation_posture,
    classify_physical_pre_decode_damage, classify_streaming_damage_before_decode,
    BlobCorruptedChunkLocalization, BlobCorruptionCapsuleReadiness,
    BlobCorruptionCapsuleReadinessOutcome, BlobCorruptionExportAdmissionOutcome,
    BlobCorruptionGenerationClassification, BlobCorruptionImportReadmissionOutcome,
    BlobCorruptionPlacementClass, BlobCorruptionReferenceEdge, BlobCorruptionReferenceEdges,
    BlobCorruptionReferenceSharingScope, BlobDamageCase, BlobDamageEvidence,
    BlobQuarantineDiagnostics, BlobQuarantineLifecycleState, BlobQuarantineRepairCapability,
};
// --- Denials (classified failure enums) ---
pub use crate::corruption::{
    BlobCorruptionDenial, BlobCorruptionDetectionSource, BlobCorruptionGuardDenial,
    WORTHableCorruptionEvidenceKind,
};
// --- Counter witnesses (read-only snapshots) ---
pub use crate::corruption::BlobCorruptionCounterSnapshot;
