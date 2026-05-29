use crate::history::data::{BranchId, CommitId, OrderedParentList};
use crate::merge::data::MergeExecutionRequest;
use crate::transactions::data::{MutationIntent, RecordRef, TransactionId};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergedCommitPlan {
    pub transaction_id: TransactionId,
    pub merged_intents: Vec<MutationIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredCommitPlan {
    Mutation(MergedCommitPlan),
    Strategy(crate::commit_strategies::data::LoweredStrategyCommitPlan),
}

impl LoweredCommitPlan {
    pub fn merged_plan(&self) -> &MergedCommitPlan {
        match self {
            Self::Mutation(plan) => plan,
            Self::Strategy(plan) => plan.merged_plan(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutionStructuralSummary {
    pub executed_record_count: usize,
    pub adopted_source_record_count: usize,
    pub preserved_shared_record_count: usize,
    pub reconciled_record_count: usize,
    pub converged_deleted_on_both_sides_count: usize,
    pub deleted_on_both_sides_lineage_unchanged_count: usize,
    pub emitted_mutation_intent_count: usize,
    pub emitted_entity_create_count: usize,
    pub emitted_relation_create_count: usize,
    pub emitted_entity_update_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutionSummary {
    pub request: MergeExecutionRequest,
    pub target_head_commit_id: CommitId,
    pub source_head_commit_id: CommitId,
    pub merge_base_commit_id: CommitId,
    pub executed_record_count: usize,
    pub adopted_source_record_count: usize,
    pub preserved_shared_record_count: usize,
    pub reconciled_record_count: usize,
    pub converged_deleted_on_both_sides_count: usize,
    pub deleted_on_both_sides_lineage_unchanged_count: usize,
    pub emitted_mutation_intent_count: usize,
    pub diagnostics_digest: String,
    pub execution_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MergeCommitMutationPlan {
    pub transaction_id: TransactionId,
    pub target_branch: BranchId,
    pub source_branch: BranchId,
    pub merge_parent_branches: Arc<[BranchId]>,
    pub requested_merge_parent_count: usize,
    pub parent_commits: OrderedParentList,
    pub merge_base_commits: Arc<[CommitId]>,
    pub merged_plan: MergedCommitPlan,
    pub structural_summary: MergeExecutionStructuralSummary,
    pub merge_execution_summary: MergeExecutionSummary,
    #[serde(
        skip_serializing,
        skip_deserializing,
        default = "merge_commit_mutation_plan_token"
    )]
    pub(crate) proof_token: MergeCommitMutationPlanToken,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishedMergeExecutionAuthority {
    pub execution_summary: MergeExecutionSummary,
    pub structural_summary: MergeExecutionStructuralSummary,
}

impl PublishedMergeExecutionAuthority {
    pub fn from_merge_plan(plan: &MergeCommitMutationPlan) -> Self {
        Self {
            execution_summary: plan.merge_execution_summary.clone(),
            structural_summary: plan.structural_summary.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeExecutionOutcome {
    pub commit: crate::transactions::data::CommitResult,
    pub execution_summary: MergeExecutionSummary,
    pub structural_summary: MergeExecutionStructuralSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct MergeCommitMutationPlanToken;

pub(crate) fn merge_commit_mutation_plan_token() -> MergeCommitMutationPlanToken {
    MergeCommitMutationPlanToken
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeApplyPlan {
    pub transaction_id: TransactionId,
    pub version_id: crate::identity::data::VersionId,
    pub merged_intents: Vec<MutationIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoRecord {
    pub record: RecordRef,
    pub detail: String,
}
