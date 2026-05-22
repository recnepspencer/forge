use crate::runtime::ForgeQueryAdmittedIntentPlan;

use super::bind_requested;
use crate::domain_capabilities::payloads::{
    ForgeQueryContinuityContributionPayload, ForgeQueryContinuityContributionPosture,
    ForgeQueryContinuityRuntimeSemantics,
};
use crate::domain_capabilities::proof_integration::ForgeQueryRequestedContinuityContribution;
use crate::domain_capabilities::targets::ForgeQueryAdmittedPlanBoundContributionTarget;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContinuityContributionAuthoring {
    pub(super) payload: ForgeQueryContinuityContributionPayload,
}

impl ForgeQueryContinuityContributionAuthoring {
    pub fn preserved(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryContinuityContributionPosture::Preserved,
            semantic_code,
            detail,
        )
    }

    pub fn preserved_rebind(
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identity: impl Into<String>,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryContinuityContributionPosture::Preserved,
            semantic_code,
            detail,
            ForgeQueryContinuityRuntimeSemantics::new(
                crate::runtime::ForgeQueryContinuityMutationFamily::RebindExistingTarget,
                crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSingleSuccessor,
                prior_authoritative_identity,
                [successor_authoritative_identity],
            ),
        )
    }

    pub fn split(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryContinuityContributionPosture::Split,
            semantic_code,
            detail,
        )
    }

    pub fn split_successors<I, S>(
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identities: I,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::with_runtime_semantics(
            ForgeQueryContinuityContributionPosture::Split,
            semantic_code,
            detail,
            ForgeQueryContinuityRuntimeSemantics::new(
                crate::runtime::ForgeQueryContinuityMutationFamily::SplitExistingTarget,
                crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors,
                prior_authoritative_identity,
                successor_authoritative_identities,
            ),
        )
    }

    pub fn replaced(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryContinuityContributionPosture::Replaced,
            semantic_code,
            detail,
        )
    }

    pub fn replaced_merge_successor(
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identity: impl Into<String>,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryContinuityContributionPosture::Replaced,
            semantic_code,
            detail,
            ForgeQueryContinuityRuntimeSemantics::new(
                crate::runtime::ForgeQueryContinuityMutationFamily::RebindExistingTarget,
                crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor,
                prior_authoritative_identity,
                [successor_authoritative_identity],
            ),
        )
    }

    pub fn correspondence_only(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(
            ForgeQueryContinuityContributionPosture::CorrespondenceOnly,
            semantic_code,
            detail,
        )
    }

    pub fn for_admitted_intent_plan(
        self,
        plan: &ForgeQueryAdmittedIntentPlan,
    ) -> ForgeQueryRequestedContinuityContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        self.bind_to_admitted_plan_target(
            ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        )
    }

    pub fn bind_to_admitted_plan_target(
        self,
        target: ForgeQueryAdmittedPlanBoundContributionTarget,
    ) -> ForgeQueryRequestedContinuityContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        bind_requested(self.payload, target)
    }

    fn new(
        posture: ForgeQueryContinuityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            payload: ForgeQueryContinuityContributionPayload::new(posture, semantic_code, detail),
        }
    }

    fn with_runtime_semantics(
        posture: ForgeQueryContinuityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: ForgeQueryContinuityRuntimeSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryContinuityContributionPayload::with_runtime_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
            ),
        }
    }
}
