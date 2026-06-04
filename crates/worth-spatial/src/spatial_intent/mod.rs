mod arbitration;
mod constraints;
mod lowering;
mod policy;
pub(crate) mod refs;
pub(crate) mod resolution;

pub use arbitration::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    analyze_spatial_intent_conflict_with_capabilities_and_profile,
    analyze_spatial_intent_conflict_with_profile, resolve_spatial_intent_conflict_by_choice,
    resolve_spatial_intent_conflict_by_policy, SpatialArbitrationContinuityHint,
    SpatialArbitrationPreviewHint, SpatialAuthoredActKind, SpatialBlockedCapability,
    SpatialChosenIntentAuthority, SpatialChosenIntentResolution,
    SpatialIdentityContinuityAssessment, SpatialIdentityContinuityClass,
    SpatialIdentityContinuityExplanationClass, SpatialIntentArbitrationAnalysis,
    SpatialIntentArbitrationDeclaration, SpatialIntentCandidate,
    SpatialIntentCandidateAvailability, SpatialIntentCandidateRank, SpatialIntentCapabilitySet,
    SpatialIntentCapabilitySummary, SpatialIntentConflictClass, SpatialIntentEscalation,
    SpatialIntentExplanationClass, SpatialIntentPreviewCommitDisposition,
    SpatialIntentPreviewWarning, SpatialIntentResolutionError, SpatialObservedRelationFact,
};
pub use constraints::*;
pub use lowering::*;
pub use policy::*;
pub use resolution::{
    admit_spatial_frame, AdmittedSpatialFrameRef, SpatialFrameBasis, SpatialFrameError,
};
