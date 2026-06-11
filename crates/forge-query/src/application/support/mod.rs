mod closure;
mod identity_boundary_hostile_matrix;
pub(crate) mod identity_boundary_inventory;
mod registry;
mod report;
#[cfg(test)]
mod tests;

pub use crate::query_context::QueryContextDeferredScopeMarker;
pub use closure::{
    ForgeQueryEvidenceIdentityBoundaryClosure, ForgeQueryFolkloreResidueStatus,
    ForgeQueryIdentityBoundaryClosure, ForgeQueryMilestoneClosureStatus,
    ForgeQuerySessionLabelBoundaryClosure, ForgeQueryStopClassBoundaryClosure,
};
pub use identity_boundary_hostile_matrix::{
    identity_boundary_hostile_matrix_artifact, identity_boundary_hostile_matrix_digest,
    ForgeQueryIdentityBoundaryHostileMatrixArtifact, ForgeQueryIdentityBoundaryHostileMatrixRow,
    MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES,
    MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES, MILESTONE_NINE_SIX_SUITE_NAME,
};
pub use identity_boundary_inventory::{
    scan_format_digest_residue_paths, scan_raw_session_admission_residue_paths,
    scan_string_carried_session_identity_residue_paths, scan_string_matching_residue_paths,
    EVIDENCE_IDENTITY_COVERED_SURFACES, EXACT_ZERO_FORMAT_DIGEST_PATHS,
    EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS, EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS,
    EXACT_ZERO_STRING_MATCHING_PATHS, EXCLUDED_FOLKLORE_PATHS, SESSION_LABEL_ORDINARY_ENTRYPOINTS,
    STOP_CLASS_COVERED_CONTRACTS,
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
