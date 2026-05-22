mod assessment;

pub use assessment::{PrimitiveIntentPreview, PrimitiveIntentPreviewAssessment};
use worth_spatial::facade::{
    assess_spatial_identity_continuity_from_analysis,
    prepare_spatial_intent_preview_with_capabilities_and_profile, SpatialAuthoredActKind,
    SpatialIdentityContinuityAssessment, SpatialIntentCapabilitySet, SpatialIntentPolicyProfile,
    SpatialIntentPreview, SpatialObservedRelationFact,
};

pub fn preview_primitive_intent_with_capabilities_and_profile(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
    profile: SpatialIntentPolicyProfile,
) -> SpatialIntentPreview {
    prepare_spatial_intent_preview_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        capabilities,
        profile,
    )
}

pub fn preview_primitive_intent_continuity_with_capabilities_and_profile(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
    profile: SpatialIntentPolicyProfile,
) -> SpatialIdentityContinuityAssessment {
    let preview = preview_primitive_intent_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        capabilities,
        profile,
    );
    assess_spatial_identity_continuity_from_analysis(preview.analysis())
}
