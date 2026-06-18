mod closure;
mod concurrent_hostile_matrix;
mod consumer_kit_closure;
mod identity_boundary_certification_gate;
mod identity_boundary_hostile_matrix;
pub(crate) mod identity_boundary_inventory;
mod journal_identity;
mod milestone_nine_seven_closure;
mod public_bridge_reader_lane;
mod registry;
mod report;
mod shared_read_pinning;
#[cfg(test)]
mod tests;
mod worth_ui_binding_evidence;

pub use crate::query_context::QueryContextDeferredScopeMarker;
pub use closure::{
    ForgeQueryEvidenceIdentityBoundaryClosure, ForgeQueryFolkloreResidueStatus,
    ForgeQueryIdentityBoundaryClosure, ForgeQueryMilestoneClosureStatus,
    ForgeQuerySessionLabelBoundaryClosure, ForgeQueryStopClassBoundaryClosure,
};
pub use concurrent_hostile_matrix::{
    ForgeQueryConcurrentHostileMatrixArtifact, ForgeQueryConcurrentHostileMatrixPosture,
    ForgeQueryConcurrentHostileMatrixSabotage, ForgeQueryConcurrentHostileMatrixSabotageKind,
};
pub use consumer_kit_closure::{
    milestone_nine_eight_consumer_kit_closure, ForgeQueryConsumerKitCertificationCase,
    ForgeQueryConsumerKitCertificationCaseRow, ForgeQueryConsumerKitCertificationTier,
    ForgeQueryConsumerKitClosure, ForgeQueryConsumerKitDocsAgreement,
    ForgeQueryConsumerKitDocsFamilyRow, ForgeQueryConsumerKitFamilyClosureRow,
    ForgeQueryConsumerKitFamilyName, ForgeQueryConsumerKitHostileCertification,
    ForgeQueryConsumerKitReferenceResidue, ForgeQueryConsumerKitResidueBreakdown,
};
pub use identity_boundary_certification_gate::{
    milestone_nine_six_certification_gate_certified, MILESTONE_9_6_CERTIFICATION_GATE_PATHS,
};
pub use identity_boundary_hostile_matrix::{
    identity_boundary_hostile_matrix_artifact, identity_boundary_hostile_matrix_digest,
    ForgeQueryIdentityBoundaryHostileMatrixArtifact, ForgeQueryIdentityBoundaryHostileMatrixRow,
    MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES,
    MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES, MILESTONE_NINE_SIX_SUITE_NAME,
};
#[cfg(test)]
pub use identity_boundary_inventory::scan_format_digest_residue_path_patterns;
#[allow(unused_imports)]
pub use identity_boundary_inventory::{
    scan_format_digest_residue_paths, scan_lower_runtime_identity_shim_paths,
    scan_raw_session_admission_residue_paths, scan_string_carried_session_identity_residue_paths,
    scan_string_matching_residue_paths, EVIDENCE_IDENTITY_COVERED_SURFACES,
    EXACT_ZERO_FORMAT_DIGEST_PATHS, EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS,
    EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS, EXACT_ZERO_STRING_MATCHING_PATHS,
    EXCLUDED_FOLKLORE_DEFERRALS, EXCLUDED_FOLKLORE_PATHS, LOWER_RUNTIME_IDENTITY_SHIM_PATHS,
    SESSION_LABEL_ORDINARY_ENTRYPOINTS, STOP_CLASS_COVERED_CONTRACTS,
};
#[cfg(test)]
pub(crate) use journal_identity::{
    forge_query_journal_identity_inventory, scan_journal_identity_forbidden_patterns,
    scan_journal_identity_required_pattern_failures, ForgeQueryJournalIdentityOperationKind,
};
pub use journal_identity::{
    ForgeQueryJournalIdentityBoundaryPosture, ForgeQueryJournalIdentityCertification,
    ForgeQueryJournalIdentityInventoryEvidence, ForgeQueryJournalIdentityScheduleEvidence,
    ForgeQueryJournalReplayBoundaryCertification, ForgeQueryJournalReplaySurfaceEvidence,
};
pub use milestone_nine_seven_closure::{
    ForgeQueryMilestoneNineSevenDerivedClosure, ForgeQueryMilestoneNineSevenPhaseClosure,
};
pub use public_bridge_reader_lane::{
    ForgeQueryPublicBridgeForbiddenAccessFinding, ForgeQueryPublicBridgeForbiddenAccessPattern,
    ForgeQueryPublicBridgeProjectionConsumptionEvidence,
    ForgeQueryPublicBridgePublishedProjectionReader, ForgeQueryPublicBridgeReaderLaneCertification,
    ForgeQueryPublicBridgeReaderLaneInventory, ForgeQueryPublicBridgeReaderLanePosture,
    ForgeQueryPublicBridgeReaderLaneSabotage, ForgeQueryPublicBridgeReaderLaneSabotageKind,
    ForgeQueryPublicBridgeReaderLaneSabotageOutcome,
};
pub use registry::{
    ForgeQueryCapabilityDescriptor, ForgeQueryCapabilityFamily, ForgeQueryCapabilityRegistry,
    ForgeQueryCapabilityStatus, ForgeQueryCapabilitySupportStatus, ForgeQuerySupportMatrix,
};
pub use report::{
    ForgeQueryIdentityEvolutionSupportProfile, ForgeQueryQueryCompositionSupportProfile,
    ForgeQueryQueryContextSupportProfile, ForgeQuerySupportReport, ForgeQuerySupportReportCounters,
    ForgeQuerySupportSectionPosture,
};
#[cfg(test)]
pub(crate) use shared_read_pinning::{
    scan_shared_read_mint_forbidden_patterns, scan_shared_read_pin_hot_path_forbidden_patterns,
    scan_shared_read_pin_required_pattern_failures, scan_shared_read_pin_retire_forbidden_patterns,
    shared_read_pinning_operation_inventory, ForgeQuerySharedReadPinningOperationKind,
};
pub use shared_read_pinning::{
    ForgeQuerySharedReadPinningBoundaryClosure, ForgeQuerySharedReadPinningBoundaryPosture,
    ForgeQuerySharedReadPinningCertification, ForgeQuerySharedReadPinningCounterEvidence,
    ForgeQuerySharedReadPinningHostileMatrixEvidence, ForgeQuerySharedReadPinningInventoryEvidence,
    ForgeQuerySharedReadPortabilityEvidence, ForgeQuerySharedReadStaleBasisDenialEvidence,
};
pub use worth_ui_binding_evidence::worth_ui_query_binding_evidence_identity;
