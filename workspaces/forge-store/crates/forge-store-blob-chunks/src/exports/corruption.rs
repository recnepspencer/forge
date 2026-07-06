// --- Capabilities (admission handles, next-step types) ---
pub use crate::corruption::{
    AuthoritativeBlobCorruptionPosture, BlobChunkQuarantine, BlobCorruptionExportAdmission,
    BlobCorruptionGuard, BlobCorruptionImportReadmission, BlobQuarantineAuthority,
    DerivedBlobCorruptionRebuildReadiness,
};
// --- Outcomes (transition receipts) ---
pub use crate::corruption::{
    BlobCorruptedChunkLocalization, BlobCorruptionCapsuleReadiness,
    BlobCorruptionCapsuleReadinessOutcome, BlobCorruptionExportAdmissionOutcome,
    BlobCorruptionGenerationClassification, BlobCorruptionImportReadmissionOutcome,
    BlobCorruptionPlacementClass, BlobCorruptionReferenceEdge, BlobCorruptionReferenceEdges,
    BlobCorruptionReferenceSharingScope, BlobDamageCase, BlobQuarantineDiagnostics,
    BlobQuarantineLifecycleState, classify_physical_pre_decode_damage,
};
// --- Denials (classified failure enums) ---
pub use crate::corruption::{
    BlobCorruptionDenial, BlobCorruptionDetectionSource, BlobCorruptionGuardDenial,
};
// --- Counter witnesses (read-only snapshots) ---
pub use crate::corruption::BlobCorruptionCounterSnapshot;