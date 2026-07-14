mod authority;
mod counters;
mod denial;
mod generation_classification;
mod generation_registry;
mod generation_registry_authority;
mod generation_registry_counters;
mod generation_registry_denial;
mod identity;
mod progression;
mod progression_steps;
mod receipts;

#[cfg(test)]
mod boundary_tests;
#[cfg(any(test, feature = "certification-test-authority"))]
pub(crate) mod generation_registry_test_support;
#[cfg(test)]
mod generation_registry_tests;

pub use authority::{
    BlobLifecycleLoweringCapability, BlobLifecycleReadinessAuthority, BlobLifecycleStoreAuthority,
};
pub use counters::BlobLifecycleCounterSnapshot;
pub use denial::{
    reject_copied_counters_as_lifecycle_receipt, reject_copied_digest_string_as_lifecycle_receipt,
    reject_imported_manifest_text_as_lifecycle_receipt,
    reject_io_qos_placement_seed_as_lifecycle_receipt,
    reject_physical_integrity_report_as_lifecycle_receipt,
    reject_terminal_projection_row_as_lifecycle_receipt, BlobLifecycleDenial,
};
pub use generation_classification::{
    AuthoritativeBlob, BlobCorruptionClassification, BlobObjectClassification,
    BlobObjectClassificationAdmission, DerivedBlob, DerivedBlobRebuildPosture,
};
pub use generation_registry::{
    BlobGenerationObservation, BlobGenerationRegistry, BlobGenerationRegistryAdmission,
    BlobGenerationRegistryEntry,
};
pub use generation_registry_authority::{
    BlobGenerationRegistryAuthority, DerivedBlobRebuildAuthority,
};
pub use generation_registry_counters::BlobGenerationRegistryCounterSnapshot;
pub use generation_registry_denial::{
    reject_chunk_tree_equality_as_blob_identity, reject_copied_lifecycle_receipt_as_blob_identity,
    reject_digest_equality_as_blob_identity, reject_physical_generation_as_blob_generation,
    reject_raw_generation_number_as_blob_identity, reject_semantic_reference_id_as_blob_identity,
    reject_terminal_projection_row_as_blob_identity, BlobGenerationRegistryDenial,
};
pub use identity::{
    AuthenticatedFrameDigest, BlobAuthorityClassification, BlobGeneration,
    BlobLifecycleDeclaration, BlobObjectId, ChunkTreeRoot, LogicalContentDigest, StoredChunkDigest,
};
pub use progression::{
    BlobLifecycleAdmission, BlobLifecycleExecuted, BlobLifecycleExecutionOutcome,
    BlobLifecycleExecutionReady, BlobLifecycleExecutionReadyOutcome, BlobLifecycleLowered,
    BlobLifecyclePlacementAdmissionOutcome, BlobLifecyclePlacementAdmitted,
    BlobLifecycleReachabilityAdmissionOutcome, BlobLifecycleReachabilityAdmitted,
    BlobLifecycleReplayInput, BlobLifecycleResolved,
};
pub use receipts::{
    BlobDedupeReceipt, BlobReachabilityReceipt, BlobResumabilityReceipt, BlobRetentionReceipt,
    LifecycleReceipt,
};
