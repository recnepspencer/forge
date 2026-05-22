mod clarification;
mod conflict;

pub use conflict::PrimitiveIntentConflict;
use worth_spatial::facade::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    resolve_spatial_intent_conflict_by_choice, resolve_spatial_intent_conflict_by_policy,
    SpatialAuthoredActKind, SpatialChosenIntentResolution, SpatialIntentArbitrationAnalysis,
    SpatialIntentCandidate, SpatialIntentCapabilitySet, SpatialIntentResolutionError,
    SpatialObservedRelationFact,
};

pub use clarification::{
    prepare_primitive_intent_clarification_request, PrimitiveIntentClarificationCandidate,
    PrimitiveIntentClarificationRequest, PrimitiveIntentClarificationRequestError,
};

pub fn analyze_primitive_intent_conflict(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
) -> SpatialIntentArbitrationAnalysis {
    analyze_spatial_intent_conflict(authored_act, observed_relation_facts)
}

pub fn analyze_primitive_intent_conflict_with_capabilities(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
) -> SpatialIntentArbitrationAnalysis {
    analyze_spatial_intent_conflict_with_capabilities(
        authored_act,
        observed_relation_facts,
        capabilities,
    )
}

pub fn resolve_primitive_intent_conflict_by_policy(
    analysis: SpatialIntentArbitrationAnalysis,
) -> Result<SpatialChosenIntentResolution, SpatialIntentResolutionError> {
    resolve_spatial_intent_conflict_by_policy(analysis)
}

pub fn resolve_primitive_intent_conflict_by_choice(
    analysis: SpatialIntentArbitrationAnalysis,
    chosen_candidate: SpatialIntentCandidate,
) -> Result<SpatialChosenIntentResolution, SpatialIntentResolutionError> {
    resolve_spatial_intent_conflict_by_choice(analysis, chosen_candidate)
}
