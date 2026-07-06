//! # STATE GRAPH
//!
//! Blob corruption is a localized physical damage state machine with explicit readmission:
//!
//! - **Damage evidence** enters from streaming read checksum observation, physical pre-decode
//!   denial ([`classify_physical_pre_decode_damage`]), or offline raw reports (rejected for
//!   blob authority via [`reject_offline_observation_as_blob_corruption_authority`]).
//! - **Classification** runs through [`BlobDamageCase`] decision table before logical decode
//!   or localization ([`classify_streaming_read_damage_from_checksum_match`],
//!   [`classify_damage_case_from_detection_context`]).
//! - **Verification** matches generation/frontier and reference edges before receipt construction.
//! - **Localization** binds damage to affected chunk ordinal and reference edges only.
//! - **Quarantine seal** consumes [`BlobQuarantineAuthority`] and produces diagnostics — not read
//!   authority ([`BlobQuarantineDiagnostics`]).
//! - **Generation posture** classifies repair/readmission capability
//!   ([`BlobCorruptionGenerationClassification`]).
//! - **Readmission** rebuilds current Store authority ([`BlobCorruptionImportReadmission::admit_from_posture`]).
//!
//! Logical decode is blocked at streaming checksum verification and physical pre-decode gates
//! before [`localize_detected_damage`] runs.

mod classification;
mod counters;
mod denial;
mod downstream;
mod orchestration;
mod receipt_construction;
mod transitions;
mod types;
mod verification;

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod shared_reference_tests;

pub use classification::{
    AuthoritativeBlobCorruptionPosture, BlobCorruptionGenerationClassification, BlobDamageCase,
    DerivedBlobCorruptionRebuildReadiness,
};
pub use counters::BlobCorruptionCounterSnapshot;
pub use denial::{
    reject_chunk_integrity_report_as_blob_corruption_authority,
    reject_copied_counters_as_blob_corruption_authority,
    reject_offline_observation_as_blob_corruption_authority,
    reject_physical_quarantine_record_as_blob_corruption_authority,
    reject_raw_digest_as_blob_corruption_authority, BlobCorruptionDenial, BlobCorruptionGuardDenial,
};
pub use downstream::{
    BlobCorruptionCapsuleReadiness, BlobCorruptionCapsuleReadinessOutcome,
    BlobCorruptionExportAdmission, BlobCorruptionExportAdmissionOutcome,
    BlobCorruptionImportReadmission, BlobCorruptionImportReadmissionOutcome,
};
pub use orchestration::BlobQuarantineAuthority;
pub use receipt_construction::{
    BlobChunkQuarantine, BlobCorruptedChunkLocalization, BlobCorruptionGuard,
    BlobQuarantineDiagnostics,
};
pub use types::{
    BlobCorruptionDetectionSource, BlobCorruptionPlacementClass,
    BlobCorruptionReferenceSharingScope, BlobQuarantineLifecycleState,
};
pub use verification::{
    classify_physical_pre_decode_damage, BlobCorruptionReferenceEdge, BlobCorruptionReferenceEdges,
};
pub(crate) use receipt_construction::construct_quarantine_diagnostics;
pub(crate) use transitions::{
    from_streaming_read_request, seal_quarantine_from_localization,
    verify_current_store_authority_for_readmission,
};
pub(crate) use classification::classify_streaming_read_damage_from_checksum_match;