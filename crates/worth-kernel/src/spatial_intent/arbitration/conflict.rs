use worth_spatial::facade::arbitration::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    analyze_spatial_intent_conflict_with_capabilities_and_profile,
    analyze_spatial_intent_conflict_with_profile, SpatialAuthoredActKind,
    SpatialIntentArbitrationAnalysis, SpatialIntentCandidate, SpatialIntentCandidateRank,
    SpatialIntentCapabilitySet, SpatialIntentConflictClass, SpatialIntentEscalation,
    SpatialIntentPolicyProfile, SpatialIntentResolutionError, SpatialObservedRelationFact,
};

use super::{
    prepare_primitive_intent_clarification_request, PrimitiveIntentClarificationRequest,
    PrimitiveIntentClarificationRequestError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveIntentConflict {
    analysis: SpatialIntentArbitrationAnalysis,
}

impl PrimitiveIntentConflict {
    pub fn analyze(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
    ) -> Self {
        Self {
            analysis: analyze_spatial_intent_conflict(authored_act, observed_relation_facts),
        }
    }

    pub fn analyze_with_capabilities(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        capabilities: SpatialIntentCapabilitySet,
    ) -> Self {
        Self {
            analysis: analyze_spatial_intent_conflict_with_capabilities(
                authored_act,
                observed_relation_facts,
                capabilities,
            ),
        }
    }

    pub fn analyze_with_profile(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        profile: SpatialIntentPolicyProfile,
    ) -> Self {
        Self {
            analysis: analyze_spatial_intent_conflict_with_profile(
                authored_act,
                observed_relation_facts,
                profile,
            ),
        }
    }

    pub fn analyze_with_capabilities_and_profile(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: &[SpatialObservedRelationFact],
        capabilities: SpatialIntentCapabilitySet,
        profile: SpatialIntentPolicyProfile,
    ) -> Self {
        Self {
            analysis: analyze_spatial_intent_conflict_with_capabilities_and_profile(
                authored_act,
                observed_relation_facts,
                capabilities,
                profile,
            ),
        }
    }

    pub fn analysis(&self) -> &SpatialIntentArbitrationAnalysis {
        &self.analysis
    }

    pub fn candidates(&self) -> &[SpatialIntentCandidateRank] {
        self.analysis.candidates()
    }

    pub fn conflict_class(&self) -> SpatialIntentConflictClass {
        self.analysis.conflict_class()
    }

    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.analysis.escalation()
    }

    pub fn clarification_request(
        &self,
    ) -> Result<PrimitiveIntentClarificationRequest, PrimitiveIntentClarificationRequestError> {
        prepare_primitive_intent_clarification_request(self.analysis.clone())
    }

    pub fn resolve_by_policy(
        &self,
    ) -> Result<
        worth_spatial::facade::arbitration::SpatialChosenIntentResolution,
        SpatialIntentResolutionError,
    > {
        worth_spatial::facade::arbitration::resolve_spatial_intent_conflict_by_policy(
            self.analysis.clone(),
        )
    }

    pub fn resolve_by_choice(
        &self,
        chosen_candidate: SpatialIntentCandidate,
    ) -> Result<
        worth_spatial::facade::arbitration::SpatialChosenIntentResolution,
        SpatialIntentResolutionError,
    > {
        worth_spatial::facade::arbitration::resolve_spatial_intent_conflict_by_choice(
            self.analysis.clone(),
            chosen_candidate,
        )
    }
}
