mod analysis;
mod arbitration_clarification;
mod arbitration_conflict;
mod arbitration_preview_assessment;
mod candidates;
mod capabilities;
mod declaration;
mod facts;
mod resolution;
mod workflow_projection;

pub(crate) use analysis::{
    analyze_spatial_arbitration_conflict, analyze_spatial_arbitration_conflict_with_capabilities,
    analyze_spatial_arbitration_conflict_with_capabilities_and_profile,
    analyze_spatial_arbitration_conflict_with_profile,
};
pub(crate) use arbitration_clarification::prepare_spatial_arbitration_clarification_request;
pub use arbitration_clarification::{
    SpatialArbitrationClarificationCandidate, SpatialArbitrationClarificationRequest,
    SpatialArbitrationClarificationRequestError,
};
pub use arbitration_conflict::SpatialArbitrationConflict;
pub use arbitration_preview_assessment::SpatialArbitrationPreviewAssessment;
pub use candidates::SpatialArbitrationCandidate;
pub use capabilities::{
    SpatialArbitrationCandidateAvailability, SpatialArbitrationCapabilitySet,
    SpatialArbitrationCapabilitySummary, SpatialBlockedCapability,
};
pub use declaration::{
    SpatialArbitrationAnalysis, SpatialArbitrationCandidateRank, SpatialArbitrationConflictClass,
    SpatialArbitrationContinuityHint, SpatialArbitrationDeclaration, SpatialArbitrationEscalation,
    SpatialArbitrationExplanationClass, SpatialArbitrationPreviewHint,
};
pub use facts::{SpatialAuthoredActKind, SpatialObservedRelationFact};
pub(crate) use resolution::{
    resolve_spatial_arbitration_conflict_by_choice, resolve_spatial_arbitration_conflict_by_policy,
};
pub use resolution::{
    SpatialArbitrationResolutionError, SpatialChosenArbitrationAuthority,
    SpatialChosenArbitrationResolution,
};
pub use workflow_projection::{
    SpatialArbitrationPreviewCommitDisposition, SpatialArbitrationPreviewWarning,
    SpatialIdentityContinuityAssessment, SpatialIdentityContinuityClass,
    SpatialIdentityContinuityExplanationClass,
};

#[cfg(test)]
mod tests;
