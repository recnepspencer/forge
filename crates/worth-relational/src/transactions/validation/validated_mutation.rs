//! Validated mutation owner and invariant evidence.

use crate::authority::commit::phases::prepare::{
    prepare_working_state_scope, PreparedWorkingStateScope,
};
use crate::authority::commit::phases::proposed_invariant_state::prepare_proposed_invariant_state;
use crate::branch::RelationalBranchVersion;
use crate::history::data::{BranchId, CommitId};
use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitConflict, CommitResult, CommitValidation, CommitValidationSummary, ConflictClass,
    TransactionCommitError,
};
use crate::validation::engine::InvariantExecutionResult;

use crate::transactions::RelationalTransaction;

/// Owner-minted evidence that Relational evaluated the exact proposed mutation
/// through its installed commit-boundary, mutation-sensitive, and publication
/// invariant families.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalMutationInvariantEvidence {
    branch: BranchId,
    proposed_version: VersionId,
    proposal_identity: super::proposal_identity::RelationalMutationProposalIdentity,
    summary: CommitValidationSummary,
}

impl RelationalMutationInvariantEvidence {
    pub const fn branch(&self) -> &BranchId {
        &self.branch
    }

    pub const fn proposed_version(&self) -> VersionId {
        self.proposed_version
    }

    pub fn proposal_identity(
        &self,
    ) -> &super::proposal_identity::RelationalMutationProposalIdentity {
        &self.proposal_identity
    }

    pub const fn summary(&self) -> CommitValidationSummary {
        self.summary
    }
}

/// Move-only Relational authority for one invariant-validated proposed state.
///
/// Construction is private to `RelationalTransaction::validate`. Committing it
/// rechecks the exact branch-qualified validation basis before publication.
pub struct ValidatedRelationalMutation {
    pub(crate) transaction_id: crate::transactions::data::TransactionId,
    pub(crate) options: crate::transactions::data::TransactionOptions,
    pub(crate) prepared: PreparedWorkingStateScope,
    pub(crate) proposed_working_state: crate::runtime::WorkingState,
    pub(crate) commit_boundary: InvariantExecutionResult,
    pub(crate) evidence: RelationalMutationInvariantEvidence,
    pub(crate) proposal_identity: super::proposal_identity::RelationalMutationProposalIdentity,
    pub(crate) validated_against_commit: Option<CommitId>,
    pub(crate) validated_against_version: VersionId,
    pub(crate) validated_against_branch_version: RelationalBranchVersion,
    pub(crate) batch_count: usize,
}

impl ValidatedRelationalMutation {
    pub const fn invariant_evidence(&self) -> &RelationalMutationInvariantEvidence {
        &self.evidence
    }
}

impl RelationalTransaction<'_> {
    pub fn validate(mut self) -> Result<ValidatedRelationalMutation, TransactionCommitError> {
        let batch_count = self.batches.len();
        let branch_binding = self.options.branch_binding().clone();
        let branch = branch_binding.identity().branch_id().clone();
        self.runtime
            .history
            .record_transaction_validation_attempt(&branch);
        if branch_binding.identity().runtime_instance_id() != self.runtime.runtime_instance_id() {
            return Err(TransactionCommitError::conflict(CommitConflict::new(
                ConflictClass::StaleValidationBasis {
                    detail: "branch binding belongs to another Relational runtime".to_owned(),
                },
            )));
        }
        if !self
            .runtime
            .legacy_branch_binding_is_current(&branch_binding)
        {
            return Err(TransactionCommitError::conflict(CommitConflict::new(
                ConflictClass::StaleValidationBasis {
                    detail: "owner-issued branch binding is no longer current".to_owned(),
                },
            )));
        }
        let binding_commit = self.runtime.legacy_branch_binding_commit(&branch_binding);
        self.runtime
            .history
            .record_retained_history_head_lookup(&branch);
        let validated_against_version = self
            .runtime
            .legacy_branch_binding_version(&branch_binding)
            .ok_or_else(|| {
                TransactionCommitError::conflict(CommitConflict::new(
                    ConflictClass::StaleValidationBasis {
                        detail: "owner-issued branch binding has no exact local version basis"
                            .to_owned(),
                    },
                ))
            })?;
        let validated_against_commit = binding_commit.as_ref().map(|head| head.commit_id());
        let prepared = prepare_working_state_scope(&mut self)?;
        self.runtime.history.record_candidate_preparation(&branch);
        let proposal_identity = self
            .runtime
            .issue_mutation_proposal_identity(self.transaction_id, &self.options)?;
        let proposed_version = proposal_identity.proposed_version_id();
        let proposed_working_state = prepare_proposed_invariant_state(
            self.runtime,
            &prepared.selected_branch_state,
            &prepared.working_state,
            &prepared.merged_plan,
            proposed_version,
        )?;
        let commit_boundary = self
            .runtime
            .invariant_authority()
            .enforce_commit_boundary_for_selected_branch(
                &prepared.selected_branch_state,
                &proposed_working_state,
                proposed_version,
                &prepared.merged_plan,
                Some(&proposal_identity),
            )?;
        let (mutation_sensitive, publication) = validate_proposed_state(
            self.runtime,
            &prepared,
            &proposed_working_state,
            proposed_version,
            Some(&proposal_identity),
        )?;
        let summary = CommitValidation::summarize(&[
            commit_boundary.clone(),
            mutation_sensitive,
            publication,
        ]);
        Ok(ValidatedRelationalMutation {
            transaction_id: self.transaction_id,
            options: self.options,
            prepared,
            proposed_working_state,
            commit_boundary,
            evidence: RelationalMutationInvariantEvidence {
                branch,
                proposed_version,
                proposal_identity: proposal_identity.clone(),
                summary,
            },
            proposal_identity,
            validated_against_commit,
            validated_against_version,
            validated_against_branch_version: branch_binding.truth_version(),
            batch_count,
        })
    }
}

impl RelationalRuntime {
    pub fn commit_validated_mutation(
        &mut self,
        candidate: ValidatedRelationalMutation,
    ) -> Result<CommitResult, TransactionCommitError> {
        self.ensure_validated_mutation_branch_is_current(&candidate)?;
        let candidate = self.revalidate_validated_mutation_if_version_advanced(candidate)?;
        self.history
            .record_publication_attempt(&candidate.evidence.branch);
        crate::authority::commit::pipeline::execute_authoritative_commit(
            self,
            crate::authority::commit::pipeline::AuthoritativeCommitContext::from_validated_mutation(
                candidate,
            ),
        )
    }

    fn ensure_validated_mutation_branch_is_current(
        &self,
        candidate: &ValidatedRelationalMutation,
    ) -> Result<(), TransactionCommitError> {
        let binding = candidate.options.branch_binding();
        if binding.identity().runtime_instance_id() != self.runtime_instance_id() {
            return Err(stale_validated_mutation(
                "validated mutation branch binding belongs to another Relational runtime",
            ));
        }
        if !self.legacy_branch_binding_is_current(binding) {
            return Err(stale_validated_mutation(
                "validated mutation branch binding is no longer current",
            ));
        }
        let Some(cell) = self.history.branch_cell(candidate.options.target_branch()) else {
            return Err(stale_validated_mutation(
                "validated mutation branch is no longer registered",
            ));
        };
        if cell.identity() != binding.identity()
            || cell.observation() != binding.observation()
            || cell.truth_version() != candidate.validated_against_branch_version
        {
            return Err(stale_validated_mutation(
                "validated mutation no longer matches the current branch reference",
            ));
        }
        Ok(())
    }

    fn revalidate_validated_mutation_if_version_advanced(
        &mut self,
        mut candidate: ValidatedRelationalMutation,
    ) -> Result<ValidatedRelationalMutation, TransactionCommitError> {
        let proposed_version = self.history.preview_next_version_id();
        if candidate.proposal_identity.proposed_version_id() == proposed_version {
            return Ok(candidate);
        }

        let proposal_identity =
            self.issue_mutation_proposal_identity(candidate.transaction_id, &candidate.options)?;
        let proposed_version = proposal_identity.proposed_version_id();
        let proposed_working_state = prepare_proposed_invariant_state(
            self,
            &candidate.prepared.selected_branch_state,
            &candidate.prepared.working_state,
            &candidate.prepared.merged_plan,
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
        candidate.proposal_identity = proposal_identity.clone();
        candidate.evidence.proposed_version = proposed_version;
        candidate.evidence.proposal_identity = proposal_identity;
        candidate.evidence.summary =
            CommitValidation::summarize(&[commit_boundary, mutation_sensitive, publication]);
        Ok(candidate)
    }
}

fn stale_validated_mutation(detail: &str) -> TransactionCommitError {
    TransactionCommitError::conflict(CommitConflict::new(ConflictClass::StaleValidationBasis {
        detail: detail.to_owned(),
    }))
}

fn validate_proposed_state(
    runtime: &mut RelationalRuntime,
    prepared: &PreparedWorkingStateScope,
    proposed: &crate::runtime::WorkingState,
    proposed_version: VersionId,
    proposal_identity: Option<&super::proposal_identity::RelationalMutationProposalIdentity>,
) -> Result<(InvariantExecutionResult, InvariantExecutionResult), TransactionCommitError> {
    let mutation_sensitive = runtime
        .invariant_authority()
        .enforce_mutation_sensitive_for_working_state(
            &prepared.selected_branch_state,
            proposed,
            proposed_version,
            &prepared.merged_plan,
            proposal_identity,
        )
        .map_err(TransactionCommitError::conflict)?;
    let publication = runtime
        .invariant_authority()
        .enforce_snapshot_publication_for_working_state(
            &prepared.selected_branch_state,
            proposed,
            proposed_version,
            &prepared.merged_plan,
            proposal_identity,
        )
        .map_err(TransactionCommitError::publication)?;
    Ok((mutation_sensitive, publication))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support::{create_entity, runtime_with_test_schema};

    #[test]
    fn stale_owner_binding_is_rejected_during_validation() {
        let mut runtime = runtime_with_test_schema();
        let identity = runtime.main_branch_identity();
        let stale_options = runtime
            .transaction_options_for(&identity)
            .expect("main branch owner binding");
        let _ = create_entity(&mut runtime, "head-advance");

        let denied = match runtime.begin_transaction(stale_options).validate() {
            Err(denied) => denied,
            Ok(_) => panic!("Relational must reject a stale expected head"),
        };

        assert!(matches!(
            denied,
            TransactionCommitError::Conflict {
                error: CommitConflict {
                    class: ConflictClass::StaleValidationBasis { .. },
                    ..
                },
                ..
            }
        ));
    }

    #[test]
    fn owner_rechecks_head_after_validation_before_commit() {
        let mut runtime = runtime_with_test_schema();
        let identity = runtime.main_branch_identity();
        let options = runtime
            .transaction_options_for(&identity)
            .expect("main branch owner binding");
        let candidate = runtime
            .begin_transaction(options)
            .validate()
            .expect("head is current at validation");
        let candidate_ordinal = candidate.proposal_identity.ordinal();

        let _ = create_entity(&mut runtime, "post-validation-advance");
        let denied = runtime
            .commit_validated_mutation(candidate)
            .expect_err("Relational must close the validate/commit race");

        assert!(matches!(
            denied,
            TransactionCommitError::Conflict {
                error: CommitConflict {
                    class: ConflictClass::StaleValidationBasis { .. },
                    ..
                },
                ..
            }
        ));

        let fresh_options = runtime
            .transaction_options_for(&runtime.main_branch_identity())
            .expect("fresh main binding");
        let fresh = runtime
            .begin_transaction(fresh_options)
            .validate()
            .expect("fresh validation remains admissible");
        assert_eq!(
            fresh.proposal_identity.ordinal(),
            candidate_ordinal + 2,
            "stale same-branch denial must not consume a proposal ordinal"
        );
    }
}
