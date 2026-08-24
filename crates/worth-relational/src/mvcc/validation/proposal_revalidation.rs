use crate::authority::commit::phases::proposed_invariant_state::prepare_proposed_invariant_state;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitConflict, CommitValidation, ConflictClass, TransactionCommitError,
};

use super::proposal_invariants::{stale_validated_proposal, validate_proposed_state};
use super::validated_proposal::ValidatedRelationalProposal;

impl RelationalRuntime {
    pub(crate) fn revalidate_proposal_for_publication(
        &mut self,
        candidate: ValidatedRelationalProposal,
    ) -> Result<ValidatedRelationalProposal, TransactionCommitError> {
        self.ensure_validated_proposal_branch_is_current(&candidate)?;
        self.revalidate_validated_proposal_if_version_advanced(candidate)
    }

    fn ensure_validated_proposal_branch_is_current(
        &self,
        candidate: &ValidatedRelationalProposal,
    ) -> Result<(), TransactionCommitError> {
        let binding = candidate.validation_input.basis();
        if binding.identity().runtime_instance_id() != self.runtime_instance_id() {
            return Err(TransactionCommitError::conflict(CommitConflict::new(
                ConflictClass::ForeignRuntime {
                    expected_runtime_instance_id: self.runtime_instance_id(),
                    actual_runtime_instance_id: binding.identity().runtime_instance_id(),
                },
            )));
        }
        if !self.admitted_branch_basis_is_current(binding) {
            return Err(stale_validated_proposal(
                "validated mutation branch binding is no longer current",
            ));
        }
        let Some(cell) = self
            .history
            .branch_cell(candidate.validation_input.target_branch())
        else {
            return Err(stale_validated_proposal(
                "validated mutation branch is no longer registered",
            ));
        };
        if cell.identity() != binding.identity()
            || cell.observation() != binding.reference()
            || cell.truth_version() != candidate.validated_against_branch_version
        {
            return Err(stale_validated_proposal(
                "validated mutation no longer matches the current branch reference",
            ));
        }
        Ok(())
    }

    fn revalidate_validated_proposal_if_version_advanced(
        &mut self,
        mut candidate: ValidatedRelationalProposal,
    ) -> Result<ValidatedRelationalProposal, TransactionCommitError> {
        let proposed_version = self.history.preview_next_version_id();
        if candidate.proposal_identity.proposed_version_id() == proposed_version {
            return Ok(candidate);
        }

        let complexity_before = self.performance_access().complexity_counters_snapshot();
        let proposal_identity = self.issue_mutation_proposal_identity(
            candidate.transaction_id,
            &candidate.validation_input,
        )?;
        let proposed_version = proposal_identity.proposed_version_id();
        let proposed_working_state = prepare_proposed_invariant_state(
            self,
            &candidate.prepared.selected_branch_state,
            &candidate.prepared.working_state,
            &candidate.prepared.merged_plan,
            candidate.prepared.schema_authority.as_ref(),
            proposed_version,
        )?;
        let commit_boundary = self
            .invariant_authority()
            .enforce_commit_boundary_for_selected_branch(
                &candidate.prepared.selected_branch_state,
                &proposed_working_state,
                proposed_version,
                &candidate.prepared.merged_plan,
                Some(&proposal_identity),
            )?;
        let (mutation_sensitive, publication) = validate_proposed_state(
            self,
            &candidate.prepared,
            &proposed_working_state,
            proposed_version,
            Some(&proposal_identity),
        )?;
        candidate.proposed_working_state = proposed_working_state;
        candidate.commit_boundary = commit_boundary.clone();
        candidate.mutation_sensitive = mutation_sensitive.clone();
        candidate.snapshot_publication = publication.clone();
        candidate.proposal_identity = proposal_identity.clone();
        candidate.evidence.proposed_version = proposed_version;
        candidate.evidence.proposal_identity = proposal_identity;
        let summary =
            CommitValidation::summarize(&[commit_boundary, mutation_sensitive, publication]);
        candidate.evidence.summary = summary;
        if let Some(artifacts) = candidate.strategy_commit_artifacts.take() {
            let validation_cost =
                crate::commit_strategies::data::StrategyPreviewValidationCostSummary::new(
                    proposed_version,
                    candidate.prepared.merged_plan.merged_intents.len(),
                    candidate
                        .prepared
                        .structural_summary
                        .touched_partitions
                        .len(),
                    candidate
                        .prepared
                        .structural_summary
                        .bulk_entity_slots_reserved,
                    candidate
                        .prepared
                        .structural_summary
                        .bulk_relation_slots_reserved,
                    2,
                );
            candidate.strategy_commit_artifacts = Some(artifacts.with_preview_validation(
                summary,
                validation_cost,
                candidate.validated_against_commit,
                candidate.validated_against_version,
            ));
        }
        let revalidation_complexity_delta =
            crate::performance::operation_complexity_accounting::complexity_delta_without_commit_gauges(
                complexity_before,
                self.performance_access().complexity_counters_snapshot(),
            );
        candidate.validation_complexity_delta =
            crate::performance::operation_complexity_accounting::combine_complexity_deltas(
                candidate.validation_complexity_delta,
                revalidation_complexity_delta,
            );
        Ok(candidate)
    }
}
