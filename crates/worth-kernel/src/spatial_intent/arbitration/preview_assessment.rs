use worth_spatial::facade::arbitration::{
    analyze_spatial_intent_conflict_with_capabilities_and_profile, SpatialAuthoredActKind,
    SpatialIdentityContinuityAssessment, SpatialIntentArbitrationAnalysis,
    SpatialIntentArbitrationDeclaration, SpatialIntentCapabilitySet, SpatialIntentPolicyProfile,
    SpatialIntentPreviewCommitDisposition, SpatialIntentPreviewWarning,
    SpatialObservedRelationFact, SpatialPreviewRichness,
};

use super::{
    prepare_primitive_intent_clarification_request, PrimitiveIntentClarificationRequest,
    PrimitiveIntentClarificationRequestError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveIntentPreviewAssessment {
    declaration: SpatialIntentArbitrationDeclaration,
    continuity: SpatialIdentityContinuityAssessment,
    capabilities: SpatialIntentCapabilitySet,
    warnings: Vec<SpatialIntentPreviewWarning>,
}

impl PrimitiveIntentPreviewAssessment {
    pub fn analyze(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        profile: SpatialIntentPolicyProfile,
    ) -> Self {
        Self::analyze_with_capabilities(
            authored_act,
            observed_relation_facts,
            SpatialIntentCapabilitySet::blocked_defaults(),
            profile,
        )
    }

    pub fn analyze_with_capabilities(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        capabilities: SpatialIntentCapabilitySet,
        profile: SpatialIntentPolicyProfile,
    ) -> Self {
        let declaration = analyze_spatial_intent_conflict_with_capabilities_and_profile(
            authored_act,
            observed_relation_facts,
            capabilities,
            profile,
        );
        let continuity = declaration.identity_continuity_assessment();
        let warnings = declaration.preview_warnings();
        Self {
            declaration,
            continuity,
            capabilities,
            warnings,
        }
    }

    pub fn declaration(&self) -> &SpatialIntentArbitrationDeclaration {
        &self.declaration
    }

    pub fn analysis(&self) -> &SpatialIntentArbitrationAnalysis {
        &self.declaration
    }

    pub fn continuity(&self) -> &SpatialIdentityContinuityAssessment {
        &self.continuity
    }

    pub fn profile(&self) -> SpatialIntentPolicyProfile {
        self.declaration.policy_profile()
    }

    pub fn capabilities(&self) -> SpatialIntentCapabilitySet {
        self.capabilities
    }

    pub fn commit_disposition(&self) -> SpatialIntentPreviewCommitDisposition {
        self.declaration.preview_commit_disposition()
    }

    pub fn preview_richness(&self) -> SpatialPreviewRichness {
        self.declaration.preview_richness()
    }

    pub fn warnings(&self) -> &[SpatialIntentPreviewWarning] {
        &self.warnings
    }

    pub fn clarification_request(
        &self,
    ) -> Result<PrimitiveIntentClarificationRequest, PrimitiveIntentClarificationRequestError> {
        prepare_primitive_intent_clarification_request(self.analysis().clone())
    }
}
