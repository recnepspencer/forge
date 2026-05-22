mod analysis;
mod candidates;
mod capabilities;
mod declared_analysis;
mod facts;
mod materialization;
mod materialization_vocab;
mod progression;
mod resolution;
mod runtime_declaration;

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
pub use materialization::{
    materialize_spatial_arbitration_support_report, SpatialArbitrationMaterializationDenial,
    SpatialArbitrationMaterializationProfilePlan, SpatialArbitrationSupportMaterialization,
};
pub use resolution::{
    resolve_spatial_intent_conflict_by_choice, resolve_spatial_intent_conflict_by_policy,
    SpatialChosenIntentAuthority, SpatialChosenIntentResolution, SpatialIntentResolutionError,
};
pub use runtime_declaration::{
    declare_spatial_arbitration_runtime, SpatialArbitrationRuntimeDeclaration,
};

#[cfg(test)]
mod tests;
