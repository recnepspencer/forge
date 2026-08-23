use crate::transactions::data::CommitValidationSummary;
use crate::validation::engine::InvariantExecutionResult;

use super::LoweredStrategyCommitPlan;

#[derive(Debug, Clone)]
pub(crate) struct PreparedStrategyAuthorityScope {
    pub(crate) selected_branch_state: crate::branch::SelectedRelationalBranchState,
    pub(crate) structural_summary:
        crate::authority::commit::structural_summary::CommitStructuralSummary,
    pub(crate) working_state: crate::runtime::WorkingState,
}

#[derive(Debug, Clone)]
pub struct ValidatedStrategyCommitPlan {
    lowered: LoweredStrategyCommitPlan,
    validated_against_commit_id: Option<crate::history::data::CommitId>,
    validated_against_version_id: crate::identity::data::VersionId,
    prepared_scope: PreparedStrategyAuthorityScope,
    proposed_working_state: crate::runtime::WorkingState,
    proposal_identity: crate::transactions::RelationalMutationProposalIdentity,
    commit_boundary_invariants: InvariantExecutionResult,
    preview_mutation_sensitive_invariants: InvariantExecutionResult,
    preview_publication_invariants: InvariantExecutionResult,
    preview_validation_cost: StrategyPreviewValidationCostSummary,
    validation_summary: CommitValidationSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StrategyPreviewValidationCostSummary {
    preview_version_id: crate::identity::data::VersionId,
    merged_intent_count: usize,
    touched_partition_count: usize,
    bulk_entity_slots_reserved: usize,
    bulk_relation_slots_reserved: usize,
    post_mutation_preview_pass_count: usize,
}

impl StrategyPreviewValidationCostSummary {
    pub(crate) fn new(
        preview_version_id: crate::identity::data::VersionId,
        merged_intent_count: usize,
        touched_partition_count: usize,
        bulk_entity_slots_reserved: usize,
        bulk_relation_slots_reserved: usize,
        post_mutation_preview_pass_count: usize,
    ) -> Self {
        Self {
            preview_version_id,
            merged_intent_count,
            touched_partition_count,
            bulk_entity_slots_reserved,
            bulk_relation_slots_reserved,
            post_mutation_preview_pass_count,
        }
    }

    pub fn preview_version_id(&self) -> crate::identity::data::VersionId {
        self.preview_version_id
    }

    pub fn merged_intent_count(&self) -> usize {
        self.merged_intent_count
    }

    pub fn touched_partition_count(&self) -> usize {
        self.touched_partition_count
    }

    pub fn bulk_entity_slots_reserved(&self) -> usize {
        self.bulk_entity_slots_reserved
    }

    pub fn bulk_relation_slots_reserved(&self) -> usize {
        self.bulk_relation_slots_reserved
    }

    pub fn post_mutation_preview_pass_count(&self) -> usize {
        self.post_mutation_preview_pass_count
    }
}

impl ValidatedStrategyCommitPlan {
    pub(crate) fn new(
        lowered: LoweredStrategyCommitPlan,
        validated_against_commit_id: Option<crate::history::data::CommitId>,
        validated_against_version_id: crate::identity::data::VersionId,
        prepared_scope: PreparedStrategyAuthorityScope,
        proposed_working_state: crate::runtime::WorkingState,
        proposal_identity: crate::transactions::RelationalMutationProposalIdentity,
        commit_boundary_invariants: InvariantExecutionResult,
        preview_mutation_sensitive_invariants: InvariantExecutionResult,
        preview_publication_invariants: InvariantExecutionResult,
        preview_validation_cost: StrategyPreviewValidationCostSummary,
        validation_summary: CommitValidationSummary,
    ) -> Self {
        Self {
            lowered,
            validated_against_commit_id,
            validated_against_version_id,
            prepared_scope,
            proposed_working_state,
            proposal_identity,
            commit_boundary_invariants,
            preview_mutation_sensitive_invariants,
            preview_publication_invariants,
            preview_validation_cost,
            validation_summary,
        }
    }

    pub fn lowered_plan(&self) -> &LoweredStrategyCommitPlan {
        &self.lowered
    }

    pub fn validated_against_version_id(&self) -> crate::identity::data::VersionId {
        self.validated_against_version_id
    }

    pub fn validated_against_commit_id(&self) -> Option<crate::history::data::CommitId> {
        self.validated_against_commit_id
    }

    pub fn commit_boundary_invariants(&self) -> &InvariantExecutionResult {
        &self.commit_boundary_invariants
    }

    pub(crate) fn proposed_working_state(&self) -> &crate::runtime::WorkingState {
        &self.proposed_working_state
    }

    pub(crate) fn proposal_identity(
        &self,
    ) -> &crate::transactions::RelationalMutationProposalIdentity {
        &self.proposal_identity
    }

    pub fn validation_summary(&self) -> CommitValidationSummary {
        self.validation_summary
    }

    pub(crate) fn prepared_scope(&self) -> &PreparedStrategyAuthorityScope {
        &self.prepared_scope
    }

    pub fn preview_mutation_sensitive_invariants(&self) -> &InvariantExecutionResult {
        &self.preview_mutation_sensitive_invariants
    }

    pub fn preview_publication_invariants(&self) -> &InvariantExecutionResult {
        &self.preview_publication_invariants
    }

    pub fn preview_validation_cost(&self) -> StrategyPreviewValidationCostSummary {
        self.preview_validation_cost
    }
}
