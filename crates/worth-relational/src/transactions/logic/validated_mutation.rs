use crate::authority::commit::phases::mutation::branch_local_delete_allowance_for_plan;
use crate::authority::commit::phases::prepare::{
    prepare_working_state_scope, PreparedWorkingStateScope,
};
use crate::authority::mutation::apply_plan_to_working_state;
use crate::history::data::{BranchId, CommitId};
use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{
    AuthoritativeApplyPlan, CommitConflict, CommitResult, CommitValidation,
    CommitValidationSummary, ConflictClass, TransactionCommitError,
};
use crate::validation::engine::InvariantExecutionResult;

use super::RelationalTransaction;

/// Owner-minted evidence that Relational evaluated the exact proposed mutation
/// through its installed commit-boundary, mutation-sensitive, and publication
/// invariant families.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalMutationInvariantEvidence {
    branch: BranchId,
    proposed_version: VersionId,
    summary: CommitValidationSummary,
}

impl RelationalMutationInvariantEvidence {
    pub const fn branch(&self) -> &BranchId {
        &self.branch
    }

    pub const fn proposed_version(&self) -> VersionId {
        self.proposed_version
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
    pub(crate) commit_boundary: InvariantExecutionResult,
    pub(crate) evidence: RelationalMutationInvariantEvidence,
    pub(crate) validated_against_commit: Option<CommitId>,
    pub(crate) validated_against_version: VersionId,
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
        let branch = self
            .options
            .target_branch
            .clone()
            .unwrap_or_else(|| self.runtime.config.history.main_branch.clone());
        let validated_against_commit = self
            .runtime
            .history()
            .branch_head(&branch)
            .map(|head| head.commit_id);
        if self
            .options
            .expected_branch_head
            .is_some_and(|expected| expected.observed_commit() != validated_against_commit)
        {
            return Err(TransactionCommitError::conflict(CommitConflict::new(
                ConflictClass::StaleValidationBasis {
                    detail: "expected branch head differs from Relational owner truth".to_owned(),
                },
            )));
        }
        let validated_against_version = self
            .runtime
            .history()
            .branch_head(&branch)
            .map(|head| head.version_id)
            .unwrap_or_else(|| self.runtime.current_version_id());
        let prepared = prepare_working_state_scope(&mut self)?;
        let commit_boundary = self
            .runtime
            .invariant_authority()
            .enforce_commit_boundary(&prepared.merged_plan)?;
        let proposed_version = self.runtime.history().preview_next_version_id();
        let (mutation_sensitive, publication) = validate_proposed_state(
            self.runtime,
            &prepared,
            proposed_version,
            self.options.target_branch.as_ref(),
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
            commit_boundary,
            evidence: RelationalMutationInvariantEvidence {
                branch,
                proposed_version,
                summary,
            },
            validated_against_commit,
            validated_against_version,
            batch_count,
        })
    }
}

impl RelationalRuntime {
    pub fn commit_validated_mutation(
        &mut self,
        candidate: ValidatedRelationalMutation,
    ) -> Result<CommitResult, TransactionCommitError> {
        crate::authority::commit::pipeline::execute_authoritative_commit(
            self,
            crate::authority::commit::pipeline::AuthoritativeCommitContext::from_validated_mutation(
                candidate,
            ),
        )
    }
}

fn validate_proposed_state(
    runtime: &mut RelationalRuntime,
    prepared: &PreparedWorkingStateScope,
    proposed_version: VersionId,
    target_branch: Option<&BranchId>,
) -> Result<(InvariantExecutionResult, InvariantExecutionResult), TransactionCommitError> {
    let apply_plan = AuthoritativeApplyPlan {
        transaction_id: prepared.merged_plan.transaction_id,
        version_id: proposed_version,
        merged_intents: prepared.merged_plan.merged_intents.clone(),
    };
    let mutation_config = crate::config::data::MutationConfig {
        cascade_delete_policy: runtime.config.storage.cascade_delete_policy,
        adjacency_policy: runtime.config.storage.adjacency_policy.clone(),
        cross_context_policy: runtime.config.storage.cross_context_policy,
        execution_model: runtime.config.execution.execution_model,
    };
    let mut proposed = prepared.working_state.clone();
    let allowance =
        branch_local_delete_allowance_for_plan(runtime, &prepared.merged_plan, target_branch);
    let mut symbols = runtime.services.symbols.clone();
    apply_plan_to_working_state(
        &mut proposed,
        &apply_plan,
        &mutation_config,
        &runtime.config.schema.registry,
        &runtime.schema_contract_runtime.aspect_contract_plans,
        &mut symbols,
        allowance,
    )
    .map_err(TransactionCommitError::conflict)?;
    let mutation_sensitive = runtime
        .invariant_authority()
        .enforce_mutation_sensitive_for_working_state(
            &proposed,
            proposed_version,
            &prepared.merged_plan,
        )
        .map_err(TransactionCommitError::conflict)?;
    let publication = runtime
        .invariant_authority()
        .enforce_snapshot_publication_for_working_state(
            &proposed,
            proposed_version,
            &prepared.merged_plan,
        )
        .map_err(TransactionCommitError::publication)?;
    Ok((mutation_sensitive, publication))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support::{create_entity, runtime_with_test_schema};
    use crate::transactions::data::{ExpectedBranchHead, TransactionOptions};

    #[test]
    fn expected_head_is_checked_against_owner_truth_during_validation() {
        let mut runtime = runtime_with_test_schema();
        let branch = runtime.config.history.main_branch.clone();
        let stale = runtime
            .history()
            .branch_head(&branch)
            .map(|head| head.commit_id);
        let _ = create_entity(&mut runtime, "head-advance");
        let expected = stale.map_or(ExpectedBranchHead::Empty, ExpectedBranchHead::Commit);

        let denied = match runtime
            .begin_transaction(TransactionOptions::for_branch(branch).expect_branch_head(expected))
            .validate()
        {
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
        let branch = runtime.config.history.main_branch.clone();
        let expected = runtime
            .history()
            .branch_head(&branch)
            .map_or(ExpectedBranchHead::Empty, |head| {
                ExpectedBranchHead::Commit(head.commit_id)
            });
        let candidate = runtime
            .begin_transaction(TransactionOptions::for_branch(branch).expect_branch_head(expected))
            .validate()
            .expect("head is current at validation");

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
    }
}
