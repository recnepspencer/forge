use super::{
    analyze_spatial_arbitration_conflict_with_capabilities_and_profile,
    prepare_spatial_arbitration_clarification_request, SpatialArbitrationAnalysis,
    SpatialArbitrationCapabilitySet, SpatialArbitrationClarificationRequest,
    SpatialArbitrationClarificationRequestError, SpatialArbitrationDeclaration,
    SpatialArbitrationPreviewCommitDisposition, SpatialArbitrationPreviewWarning,
    SpatialAuthoredActKind, SpatialIdentityContinuityAssessment, SpatialObservedRelationFact,
};
use crate::certification::policy_support::{
    SpatialArbitrationPolicyProfile, SpatialPreviewRichness,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialArbitrationPreviewAssessment {
    declaration: SpatialArbitrationDeclaration,
    continuity: SpatialIdentityContinuityAssessment,
    capabilities: SpatialArbitrationCapabilitySet,
    warnings: Vec<SpatialArbitrationPreviewWarning>,
}

impl SpatialArbitrationPreviewAssessment {
    pub fn analyze(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        profile: SpatialArbitrationPolicyProfile,
    ) -> Self {
        Self::analyze_with_capabilities(
            authored_act,
            observed_relation_facts,
            SpatialArbitrationCapabilitySet::blocked_defaults(),
            profile,
        )
    }

    pub fn analyze_with_capabilities(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        capabilities: SpatialArbitrationCapabilitySet,
        profile: SpatialArbitrationPolicyProfile,
    ) -> Self {
        let declaration = analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
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

    pub fn declaration(&self) -> &SpatialArbitrationDeclaration {
        &self.declaration
    }

    pub fn analysis(&self) -> &SpatialArbitrationAnalysis {
        &self.declaration
    }

    pub fn continuity(&self) -> &SpatialIdentityContinuityAssessment {
        &self.continuity
    }

    pub fn profile(&self) -> SpatialArbitrationPolicyProfile {
        self.declaration.policy_profile()
    }

    pub fn capabilities(&self) -> SpatialArbitrationCapabilitySet {
        self.capabilities
    }

    pub fn commit_disposition(&self) -> SpatialArbitrationPreviewCommitDisposition {
        self.declaration.preview_commit_disposition()
    }

    pub fn preview_richness(&self) -> SpatialPreviewRichness {
        self.declaration.preview_richness()
    }

    pub fn warnings(&self) -> &[SpatialArbitrationPreviewWarning] {
        &self.warnings
    }

    pub fn clarification_request(
        &self,
    ) -> Result<SpatialArbitrationClarificationRequest, SpatialArbitrationClarificationRequestError>
    {
        prepare_spatial_arbitration_clarification_request(self.analysis().clone())
    }
}
