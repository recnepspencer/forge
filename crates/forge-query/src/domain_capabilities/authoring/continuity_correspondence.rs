use crate::correspondence::{
    StructuralCandidateDiscoveryPlan, StructuralCandidateOrderingContract,
};

use crate::domain_capabilities::payloads::{
    ForgeQueryContinuityContributionPayload, ForgeQueryContinuityContributionPosture,
    ForgeQueryContinuityCorrespondenceSemantics,
};

use super::continuity::ForgeQueryContinuityContributionAuthoring;

impl ForgeQueryContinuityContributionAuthoring {
    pub fn correspondence_lineage_only(
        canonical_subject: impl Into<String>,
        authoritative_counterpart: impl Into<String>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_correspondence_semantics(
            ForgeQueryContinuityContributionPosture::CorrespondenceOnly,
            semantic_code,
            detail,
            ForgeQueryContinuityCorrespondenceSemantics::lineage_only(
                canonical_subject,
                authoritative_counterpart,
                discovery_plan,
                budget,
            ),
        )
    }

    pub fn correspondence_structural_only(
        candidates: impl IntoIterator<Item = impl Into<String>>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
        ordering_contract: StructuralCandidateOrderingContract,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_correspondence_semantics(
            ForgeQueryContinuityContributionPosture::CorrespondenceOnly,
            semantic_code,
            detail,
            ForgeQueryContinuityCorrespondenceSemantics::structural_only(
                candidates,
                discovery_plan,
                budget,
                ordering_contract,
            ),
        )
    }

    pub fn correspondence_mixed(
        canonical_subject: impl Into<String>,
        authoritative_counterpart: impl Into<String>,
        candidates: impl IntoIterator<Item = impl Into<String>>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
        ordering_contract: StructuralCandidateOrderingContract,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_correspondence_semantics(
            ForgeQueryContinuityContributionPosture::CorrespondenceOnly,
            semantic_code,
            detail,
            ForgeQueryContinuityCorrespondenceSemantics::mixed(
                canonical_subject,
                authoritative_counterpart,
                candidates,
                discovery_plan,
                budget,
                ordering_contract,
            ),
        )
    }

    fn with_correspondence_semantics(
        posture: ForgeQueryContinuityContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        correspondence_semantics: ForgeQueryContinuityCorrespondenceSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryContinuityContributionPayload::with_correspondence_semantics(
                posture,
                semantic_code,
                detail,
                Some(correspondence_semantics),
            ),
        }
    }
}
