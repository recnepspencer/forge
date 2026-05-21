mod blocked;
mod candidates;
mod conflicts;
mod escalation;
mod ranking;
mod resolution;

pub use blocked::{
    SpatialBlockedCapability, SpatialIntentCandidateAvailability, SpatialIntentCapabilitySet,
};
pub use candidates::SpatialIntentCandidate;
pub use conflicts::{
    SpatialAuthoredActKind, SpatialIntentConflictClass, SpatialObservedRelationFact,
};
pub use escalation::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    analyze_spatial_intent_conflict_with_capabilities_and_profile,
    analyze_spatial_intent_conflict_with_profile, SpatialIntentArbitrationAnalysis,
    SpatialIntentEscalation,
};
pub use ranking::{SpatialIntentCandidateRank, SpatialIntentExplanationClass};
pub use resolution::{
    resolve_spatial_intent_conflict_by_choice, resolve_spatial_intent_conflict_by_policy,
    SpatialChosenIntentAuthority, SpatialChosenIntentResolution, SpatialIntentResolutionError,
};

#[cfg(test)]
mod tests;
