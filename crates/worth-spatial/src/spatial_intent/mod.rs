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
    analyze_spatial_intent_conflict_with_profile, declare_spatial_arbitration_runtime,
    materialize_spatial_arbitration_support_report, resolve_spatial_intent_conflict_by_choice,
    resolve_spatial_intent_conflict_by_policy, SpatialArbitrationContinuityHint,
    SpatialArbitrationMaterializationDenial, SpatialArbitrationMaterializationProfilePlan,
    SpatialArbitrationPreviewHint, SpatialArbitrationRuntimeDeclaration,
    SpatialArbitrationSupportMaterialization, SpatialAuthoredActKind, SpatialBlockedCapability,
    SpatialChosenIntentAuthority, SpatialChosenIntentResolution, SpatialIntentArbitrationAnalysis,
    SpatialIntentArbitrationDeclaration, SpatialIntentCandidate,
    SpatialIntentCandidateAvailability, SpatialIntentCandidateRank, SpatialIntentCapabilitySet,
    SpatialIntentCapabilitySummary, SpatialIntentConflictClass, SpatialIntentEscalation,
    SpatialIntentExplanationClass, SpatialIntentResolutionError, SpatialObservedRelationFact,
};
pub use constraints::*;
pub use continuity::*;
pub use lowering::*;
pub use policy::*;
pub use preview::*;
pub use refs::*;
pub use resolution::*;
