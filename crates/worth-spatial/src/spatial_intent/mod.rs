mod arbitration;
mod constraints;
mod continuity;
mod lowering;
mod policy;
mod preview;
mod refs;
mod resolution;

pub use arbitration::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    analyze_spatial_intent_conflict_with_capabilities_and_profile,
    analyze_spatial_intent_conflict_with_profile, resolve_spatial_intent_conflict_by_choice,
    resolve_spatial_intent_conflict_by_policy, SpatialAuthoredActKind, SpatialBlockedCapability,
    SpatialChosenIntentAuthority, SpatialChosenIntentResolution, SpatialIntentArbitrationAnalysis,
    SpatialIntentCandidate, SpatialIntentCandidateAvailability, SpatialIntentCandidateRank,
    SpatialIntentCapabilitySet, SpatialIntentConflictClass, SpatialIntentEscalation,
    SpatialIntentExplanationClass, SpatialIntentResolutionError, SpatialObservedRelationFact,
};
pub use constraints::*;
pub use continuity::*;
pub use lowering::*;
pub use policy::*;
pub use preview::*;
pub use refs::*;
pub use resolution::*;
