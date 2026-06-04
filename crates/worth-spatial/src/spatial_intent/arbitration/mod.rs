mod analysis;
mod candidates;
mod capabilities;
mod declared_analysis;
mod facts;
mod resolution;
mod workflow_projection;

pub use analysis::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    analyze_spatial_intent_conflict_with_capabilities_and_profile,
    analyze_spatial_intent_conflict_with_profile,
};
pub use candidates::SpatialIntentCandidate;
pub use capabilities::{
    SpatialBlockedCapability, SpatialIntentCandidateAvailability, SpatialIntentCapabilitySet,
    SpatialIntentCapabilitySummary,
};
pub use declared_analysis::{
    SpatialArbitrationContinuityHint, SpatialArbitrationPreviewHint,
    SpatialIntentArbitrationAnalysis, SpatialIntentArbitrationDeclaration,
    SpatialIntentCandidateRank, SpatialIntentConflictClass, SpatialIntentEscalation,
    SpatialIntentExplanationClass,
};
pub use facts::{SpatialAuthoredActKind, SpatialObservedRelationFact};
pub use resolution::{
    resolve_spatial_intent_conflict_by_choice, resolve_spatial_intent_conflict_by_policy,
    SpatialChosenIntentAuthority, SpatialChosenIntentResolution, SpatialIntentResolutionError,
};
pub use workflow_projection::{
    SpatialIdentityContinuityAssessment, SpatialIdentityContinuityClass,
    SpatialIdentityContinuityExplanationClass, SpatialIntentPreviewCommitDisposition,
    SpatialIntentPreviewWarning,
};

#[cfg(test)]
mod tests;
