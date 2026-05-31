use serde::{Deserialize, Serialize};

use crate::merge::data::{
    CausalAnnotationSummary, ConflictClassificationSummary, IdentityDiscoverySummary,
    LoweredMergePlanSummary, MergeAncestrySummary, MergePlanningDecisionLog,
    MergePlanningDecisionLogDigestBasis, MergePlanningRequest, MergePolicyResolutionSummary,
    ResolvedMergeBase,
};

use super::{
    MergeArtifactDigestBasis, MergeExecutionAuthorityContract, MergeSchemaSnapshotDigestBasis,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePlanningSummary {
    pub request_summary: String,
    pub ancestry_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePlanningArtifactCore {
    pub request: MergePlanningRequest,
    pub schema_snapshot: MergeSchemaSnapshotDigestBasis,
    pub execution_authority_contract: MergeExecutionAuthorityContract,
    pub merge_base: ResolvedMergeBase,
    pub ancestry: MergeAncestrySummary,
    pub identity_discovery: IdentityDiscoverySummary,
    pub conflict_classification: ConflictClassificationSummary,
    pub causal_annotation: CausalAnnotationSummary,
    pub policy_resolution: MergePolicyResolutionSummary,
    pub lowered_plan: LoweredMergePlanSummary,
    pub decision_log: MergePlanningDecisionLog,
    pub digest_basis: MergeArtifactDigestBasis,
    pub decision_log_digest_basis: MergePlanningDecisionLogDigestBasis,
    pub summary: MergePlanningSummary,
}
