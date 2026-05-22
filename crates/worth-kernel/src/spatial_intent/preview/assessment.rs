use worth_spatial::facade::{
    assess_spatial_identity_continuity_from_analysis,
    prepare_spatial_intent_preview_with_capabilities_and_profile, SpatialAuthoredActKind,
    SpatialIdentityContinuityAssessment, SpatialIntentCapabilitySet, SpatialIntentPolicyProfile,
    SpatialIntentPreview as SpatialPreviewArtifact, SpatialIntentPreviewCommitDisposition,
    SpatialIntentPreviewWarning, SpatialObservedRelationFact,
};

use crate::spatial_intent::arbitration::{
    prepare_primitive_intent_clarification_request, PrimitiveIntentClarificationRequest,
    PrimitiveIntentClarificationRequestError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveIntentPreviewAssessment {
    preview: SpatialPreviewArtifact,
    continuity: SpatialIdentityContinuityAssessment,
    capabilities: SpatialIntentCapabilitySet,
}

impl PrimitiveIntentPreviewAssessment {
    pub fn preview(&self) -> &SpatialPreviewArtifact {
        &self.preview
    }

    pub fn continuity(&self) -> &SpatialIdentityContinuityAssessment {
        &self.continuity
    }

    pub fn analysis(&self) -> &worth_spatial::facade::SpatialIntentArbitrationAnalysis {
        self.preview.analysis()
    }

    pub fn profile(&self) -> SpatialIntentPolicyProfile {
        self.preview.policy_profile()
    }

    pub fn capabilities(&self) -> SpatialIntentCapabilitySet {
        self.capabilities
    }

    pub fn commit_disposition(&self) -> SpatialIntentPreviewCommitDisposition {
        self.preview.commit_disposition()
    }

    pub fn warnings(&self) -> &[SpatialIntentPreviewWarning] {
        self.preview.warnings()
    }

    pub fn clarification_request(
        &self,
    ) -> Result<PrimitiveIntentClarificationRequest, PrimitiveIntentClarificationRequestError> {
        prepare_primitive_intent_clarification_request(self.preview.analysis().clone())
    }
}

pub struct PrimitiveIntentPreview;

impl PrimitiveIntentPreview {
    pub fn analyze(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        profile: SpatialIntentPolicyProfile,
    ) -> PrimitiveIntentPreviewAssessment {
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
    ) -> PrimitiveIntentPreviewAssessment {
        let preview = prepare_spatial_intent_preview_with_capabilities_and_profile(
            authored_act,
            observed_relation_facts,
            capabilities,
            profile,
        );
        let continuity = assess_spatial_identity_continuity_from_analysis(preview.analysis());
        PrimitiveIntentPreviewAssessment {
            preview,
            continuity,
            capabilities,
        }
    }
}
