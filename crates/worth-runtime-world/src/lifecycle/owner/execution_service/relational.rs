use crate::publication::{NoEffectCause, RelationalAttemptProgress};
use crate::recovery::ProductUnpublishedCause;

use super::RuntimeWorldOwnerRoot;

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
    /// The Relational publication leg. Publication either retains the exact
    /// component basis or publishes one prepared candidate; creating a branch
    /// is a separate owner operation and never reaches this path.
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
                self.publish_relational_candidate(candidate)
            }
        }
    }

    fn publish_relational_candidate(
        &self,
        candidate: worth_relational::facade::mvcc::PreparedRelationalCommitCandidate,
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
                    Ok(result) => Ok(RelationalAttemptProgress::settled(
                        commit_identity,
                        successor_basis,
                        result,
                    )),
                    Err(error) => Ok(match error.deferred_settlement() {
                        Some(settlement) => RelationalAttemptProgress::settlement_pending(
                            commit_identity,
                            successor_basis,
                            settlement.clone(),
                        ),
                        None => RelationalAttemptProgress::settlement_required(
                            commit_identity,
                            successor_basis,
                        ),
                    }),
                }
            }
            RelationalPublicationOutcome::Stale(_) => {
                Err(pre_effect_failure(NoEffectCause::StaleExpectedProductHead))
            }
            RelationalPublicationOutcome::Denied(_) => {
                Err(pre_effect_failure(NoEffectCause::OwnerUnavailable))
            }
            RelationalPublicationOutcome::Interrupted(_)
            | RelationalPublicationOutcome::Deferred(_)
            | RelationalPublicationOutcome::Failed(_) => {
                Err(pre_effect_failure(NoEffectCause::PreEffectFailure))
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
