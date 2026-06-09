use super::{
    analyze_spatial_arbitration_conflict, analyze_spatial_arbitration_conflict_with_capabilities,
    analyze_spatial_arbitration_conflict_with_capabilities_and_profile,
    analyze_spatial_arbitration_conflict_with_profile,
    prepare_spatial_arbitration_clarification_request, SpatialArbitrationAnalysis,
    SpatialArbitrationCandidate, SpatialArbitrationCandidateRank, SpatialArbitrationCapabilitySet,
    SpatialArbitrationClarificationRequest, SpatialArbitrationClarificationRequestError,
    SpatialArbitrationConflictClass, SpatialArbitrationEscalation,
    SpatialArbitrationResolutionError, SpatialAuthoredActKind, SpatialChosenArbitrationResolution,
    SpatialObservedRelationFact,
};
use crate::certification::policy_support::SpatialArbitrationPolicyProfile;

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialArbitrationConflict {
    analysis: SpatialArbitrationAnalysis,
}

impl SpatialArbitrationConflict {
    pub fn analyze(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
    ) -> Self {
        Self {
            analysis: analyze_spatial_arbitration_conflict(authored_act, observed_relation_facts),
        }
    }

    pub fn analyze_with_capabilities(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        capabilities: SpatialArbitrationCapabilitySet,
    ) -> Self {
        Self {
            analysis: analyze_spatial_arbitration_conflict_with_capabilities(
                authored_act,
                observed_relation_facts,
                capabilities,
            ),
        }
    }

    pub fn analyze_with_profile(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        profile: SpatialArbitrationPolicyProfile,
    ) -> Self {
        Self {
            analysis: analyze_spatial_arbitration_conflict_with_profile(
                authored_act,
                observed_relation_facts,
                profile,
            ),
        }
    }

    pub fn analyze_with_capabilities_and_profile(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        capabilities: SpatialArbitrationCapabilitySet,
        profile: SpatialArbitrationPolicyProfile,
    ) -> Self {
        Self {
            analysis: analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
                authored_act,
                observed_relation_facts,
                capabilities,
                profile,
            ),
        }
    }

    pub fn analysis(&self) -> &SpatialArbitrationAnalysis {
        &self.analysis
    }

    pub fn candidates(&self) -> &[SpatialArbitrationCandidateRank] {
        self.analysis.candidates()
    }

    pub fn conflict_class(&self) -> SpatialArbitrationConflictClass {
        self.analysis.conflict_class()
    }

    pub fn escalation(&self) -> SpatialArbitrationEscalation {
        self.analysis.escalation()
    }

    pub fn clarification_request(
        &self,
    ) -> Result<SpatialArbitrationClarificationRequest, SpatialArbitrationClarificationRequestError>
    {
        prepare_spatial_arbitration_clarification_request(self.analysis.clone())
    }

    pub fn resolve_by_policy(
        &self,
    ) -> Result<SpatialChosenArbitrationResolution, SpatialArbitrationResolutionError> {
        super::resolve_spatial_arbitration_conflict_by_policy(self.analysis.clone())
    }

    pub fn resolve_by_choice(
        &self,
        chosen_candidate: SpatialArbitrationCandidate,
    ) -> Result<SpatialChosenArbitrationResolution, SpatialArbitrationResolutionError> {
        super::resolve_spatial_arbitration_conflict_by_choice(
            self.analysis.clone(),
            chosen_candidate,
        )
    }
}
