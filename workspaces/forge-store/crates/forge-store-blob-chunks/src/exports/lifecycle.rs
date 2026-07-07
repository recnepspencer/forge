// --- Capabilities (admission handles, next-step types) ---
pub use crate::lifecycle::{
    AuthoritativeBlob, BlobGenerationRegistry, BlobGenerationRegistryAdmission,
    BlobGenerationRegistryAuthority, BlobGenerationRegistryEntry, BlobLifecycleAdmission,
    BlobLifecycleDeclaration, BlobLifecycleExecutionReady, BlobLifecycleLowered,
    BlobLifecycleLoweringCapability, BlobLifecyclePlacementAdmitted,
    BlobLifecycleReachabilityAdmitted, BlobLifecycleReadinessAuthority, BlobLifecycleResolved,
    BlobLifecycleStoreAuthority, BlobObjectClassificationAdmission, DerivedBlob,
    DerivedBlobRebuildAuthority,
};
// --- Outcomes (transition receipts) ---
pub use crate::lifecycle::{
    AuthenticatedFrameDigest, BlobAuthorityClassification, BlobCorruptionClassification,
    BlobDedupeReceipt, BlobGeneration, BlobGenerationObservation, BlobLifecycleExecuted,
    BlobLifecycleExecutionOutcome, BlobLifecycleExecutionReadyOutcome,
    BlobLifecyclePlacementAdmissionOutcome, BlobLifecycleReachabilityAdmissionOutcome,
    BlobLifecycleReplayInput, BlobObjectClassification, BlobObjectId, BlobReachabilityReceipt,
    BlobResumabilityReceipt, BlobRetentionReceipt, ChunkTreeRoot, DerivedBlobRebuildPosture,
    LifecycleReceipt, LogicalContentDigest, StoredChunkDigest,
};
// --- Denials (classified failure enums) ---
pub use crate::lifecycle::{BlobGenerationRegistryDenial, BlobLifecycleDenial};
// --- Counter witnesses (read-only snapshots) ---
pub use crate::lifecycle::{BlobGenerationRegistryCounterSnapshot, BlobLifecycleCounterSnapshot};
