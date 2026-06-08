use serde::{Deserialize, Serialize};

use crate::history::data::RelationalMergeBranchBasis;
use crate::merge::data::{
    CausalAnnotationSummary, ConflictClassificationSummary, IdentityDiscoverySummary,
    LoweredMergePlanSummary, MergeAncestrySummary, MergePlanningDecisionLog,
    MergePlanningDecisionLogDigestBasis, MergePolicyResolutionSummary,
    NormalizedRelationalMergeRequest, RelationalMergeStrategyWitness,
    RelationalSchemaReconciliationWitness,
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
    pub request: NormalizedRelationalMergeRequest,
    pub branch_basis: RelationalMergeBranchBasis,
    pub schema_snapshot: MergeSchemaSnapshotDigestBasis,
    pub schema_reconciliation_witness: RelationalSchemaReconciliationWitness,
    pub strategy_witness: RelationalMergeStrategyWitness,
    pub execution_authority_contract: MergeExecutionAuthorityContract,
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

impl MergePlanningArtifactCore {
    pub fn merge_base(&self) -> &crate::history::data::ResolvedMergeBase {
        self.branch_basis.merge_base()
    }
}
