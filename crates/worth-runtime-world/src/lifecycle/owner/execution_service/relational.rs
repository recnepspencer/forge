use crate::publication::{NoEffectCause, RelationalAttemptProgress, RelationalForkPlanInput};
use crate::recovery::ProductUnpublishedCause;

use super::RuntimeWorldOwnerRoot;

use worth_relational::facade::branch::{
    AdmittedRelationalBranchBasis, RelationalForkDenial, RelationalForkOutcome,
};
use worth_relational::facade::mvcc::RelationalPublicationOutcome;

pub(super) struct RelationalExecutionFailure {
    pub(super) cause: ProductUnpublishedCause,
    pub(super) no_effect: NoEffectCause,
    pub(super) partial: RelationalAttemptProgress,
}

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    pub(super) fn execute_relational(
        &self,
        attempt: &mut crate::publication::ReservedCompositePublicationAttempt,
    ) -> Result<RelationalAttemptProgress, RelationalExecutionFailure> {
        match attempt.plan().relational().posture() {
            crate::publication::RelationalComponentPlanPosture::RetainExact => {
                Ok(RelationalAttemptProgress::untouched())
            }
            crate::publication::RelationalComponentPlanPosture::PublishPrepared => {
                let candidate = attempt
                    .take_relational_candidate()
                    .ok_or_else(|| pre_effect_failure(NoEffectCause::PreEffectFailure))?;
                self.publish_relational_candidate(candidate, None)
            }
            crate::publication::RelationalComponentPlanPosture::ForkExact => {
                self.execute_relational_fork(attempt, false)
            }
            crate::publication::RelationalComponentPlanPosture::ForkAndAdvance => {
                self.execute_relational_fork(attempt, true)
            }
        }
    }

    fn execute_relational_fork(
        &self,
        attempt: &mut crate::publication::ReservedCompositePublicationAttempt,
        advance: bool,
    ) -> Result<RelationalAttemptProgress, RelationalExecutionFailure> {
        let input = attempt
            .take_relational_fork_input()
            .ok_or_else(|| pre_effect_failure(NoEffectCause::PreEffectFailure))?;
        let (source, destination, batch) = input.into_parts();
        let (fork, target_basis) = self
            .state
            .relational
            .fork_port()
            .fork_reserved_with_basis(destination, source)
            .map_err(|denial| fork_failure(&denial))?;
        if !advance {
            return Ok(RelationalAttemptProgress::forked(fork, target_basis));
        }

        let partial = || RelationalAttemptProgress::forked(fork.clone(), target_basis.clone());
        let batch =
            batch.ok_or_else(|| after_fork_failure(NoEffectCause::PreEffectFailure, partial()))?;
        let intent = attempt
            .plan()
            .component_intent()
            .relational_change()
            .cloned()
            .ok_or_else(|| after_fork_failure(NoEffectCause::PreEffectFailure, partial()))?;
        let mut transaction = self
            .state
            .relational
            .transaction_admission_port()
            .begin_branch_transaction(&target_basis, intent)
            .map_err(|_| after_fork_failure(NoEffectCause::OwnerUnavailable, partial()))?;
        transaction
            .push_batch(batch)
            .map_err(|_| after_fork_failure(NoEffectCause::CapacityExhausted, partial()))?;
        let candidate = self
            .state
            .relational
            .preparation_port()
            .prepare_branch_transaction(transaction)
            .map_err(|_| after_fork_failure(NoEffectCause::PreEffectFailure, partial()))?;
        self.publish_relational_candidate(candidate, Some((fork, target_basis)))
    }

    fn publish_relational_candidate(
        &self,
        candidate: worth_relational::facade::mvcc::PreparedRelationalCommitCandidate,
        fork: Option<(RelationalForkOutcome, AdmittedRelationalBranchBasis)>,
    ) -> Result<RelationalAttemptProgress, RelationalExecutionFailure> {
        match self
            .state
            .relational
            .publication_port()
            .compare_and_publish(candidate)
        {
            RelationalPublicationOutcome::Performed(performed) => {
                let commit_identity = performed.commit_identity();
                let successor_basis = performed.next_basis().clone();
                match self
                    .state
                    .relational
                    .settlement_port()
                    .settle_performed_publication(performed)
                {
                    Ok(result) => Ok(match fork {
                        Some((fork, _)) => RelationalAttemptProgress::settled_after_fork(
                            fork,
                            commit_identity,
                            successor_basis,
                            result,
                        ),
                        None => RelationalAttemptProgress::settled(
                            commit_identity,
                            successor_basis,
                            result,
                        ),
                    }),
                    Err(error) => Ok(match error.deferred_settlement() {
                        Some(settlement) => match fork {
                            Some((fork, _)) => {
                                RelationalAttemptProgress::settlement_pending_after_fork(
                                    fork,
                                    commit_identity,
                                    successor_basis,
                                    settlement.clone(),
                                )
                            }
                            None => RelationalAttemptProgress::settlement_pending(
                                commit_identity,
                                successor_basis,
                                settlement.clone(),
                            ),
                        },
                        None => match fork {
                            Some((fork, _)) => {
                                RelationalAttemptProgress::settlement_required_after_fork(
                                    fork,
                                    commit_identity,
                                    successor_basis,
                                )
                            }
                            None => RelationalAttemptProgress::settlement_required(
                                commit_identity,
                                successor_basis,
                            ),
                        },
                    }),
                }
            }
            RelationalPublicationOutcome::Stale(_) => Err(publication_failure(
                NoEffectCause::StaleExpectedProductHead,
                fork,
            )),
            RelationalPublicationOutcome::Denied(_) => {
                Err(publication_failure(NoEffectCause::OwnerUnavailable, fork))
            }
            RelationalPublicationOutcome::Interrupted(_)
            | RelationalPublicationOutcome::Deferred(_)
            | RelationalPublicationOutcome::Failed(_) => {
                Err(publication_failure(NoEffectCause::PreEffectFailure, fork))
            }
        }
    }
}

fn pre_effect_failure(no_effect: NoEffectCause) -> RelationalExecutionFailure {
    RelationalExecutionFailure {
        cause: ProductUnpublishedCause::SiblingOwnerDenied,
        no_effect,
        partial: RelationalAttemptProgress::untouched(),
    }
}

fn after_fork_failure(
    no_effect: NoEffectCause,
    partial: RelationalAttemptProgress,
) -> RelationalExecutionFailure {
    RelationalExecutionFailure {
        cause: ProductUnpublishedCause::SiblingOwnerDenied,
        no_effect,
        partial,
    }
}

fn publication_failure(
    no_effect: NoEffectCause,
    fork: Option<(RelationalForkOutcome, AdmittedRelationalBranchBasis)>,
) -> RelationalExecutionFailure {
    let partial = fork.map_or_else(RelationalAttemptProgress::untouched, |(fork, basis)| {
        RelationalAttemptProgress::forked(fork, basis)
    });
    after_fork_failure(no_effect, partial)
}

fn fork_failure(denial: &RelationalForkDenial) -> RelationalExecutionFailure {
    let no_effect = match denial {
        RelationalForkDenial::RetentionCapacityExhausted => NoEffectCause::CapacityExhausted,
        RelationalForkDenial::RetentionIdentityExhausted => NoEffectCause::CapacityExhausted,
        RelationalForkDenial::OwnerUnavailable
        | RelationalForkDenial::RetentionOwnerUnavailable => NoEffectCause::OwnerUnavailable,
        _ => NoEffectCause::PreEffectFailure,
    };
    pre_effect_failure(no_effect)
}
