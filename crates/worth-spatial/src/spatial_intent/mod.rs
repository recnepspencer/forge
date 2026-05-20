mod arbitration;
mod constraints;
mod lowering;
mod refs;
mod resolution;

pub use arbitration::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    resolve_spatial_intent_conflict_by_choice, resolve_spatial_intent_conflict_by_policy,
    SpatialAuthoredActKind, SpatialBlockedCapability, SpatialChosenIntentAuthority,
    SpatialChosenIntentResolution, SpatialIntentArbitrationAnalysis, SpatialIntentCandidate,
    SpatialIntentCandidateAvailability, SpatialIntentCandidateRank, SpatialIntentCapabilitySet,
    SpatialIntentConflictClass, SpatialIntentEscalation, SpatialIntentExplanationClass,
    SpatialIntentResolutionError, SpatialObservedRelationFact,
};
pub use constraints::*;
pub use lowering::*;
pub use refs::*;
pub use resolution::*;
