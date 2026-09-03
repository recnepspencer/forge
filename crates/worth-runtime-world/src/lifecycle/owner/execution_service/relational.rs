use crate::publication::{NoEffectCause, RelationalAttemptProgress};

use super::RuntimeWorldOwnerRoot;

use worth_relational::facade::mvcc::RelationalPublicationOutcome;

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    pub(super) fn execute_relational(
        &self,
        attempt: &mut crate::publication::ReservedCompositePublicationAttempt,
    ) -> Result<RelationalAttemptProgress, NoEffectCause> {
        match attempt.plan().relational().posture() {
            crate::publication::RelationalComponentPlanPosture::RetainExact => {
                Ok(RelationalAttemptProgress::untouched())
            }
            crate::publication::RelationalComponentPlanPosture::PublishPrepared => {
                let candidate = attempt
                    .take_relational_candidate()
                    .ok_or(NoEffectCause::PreEffectFailure)?;
                self.publish_relational_candidate(candidate)
            }
            crate::publication::RelationalComponentPlanPosture::ForkThenPublish => {
                Err(NoEffectCause::OwnerUnavailable)
            }
        }
    }

    fn publish_relational_candidate(
        &self,
        candidate: worth_relational::facade::mvcc::PreparedRelationalCommitCandidate,
    ) -> Result<RelationalAttemptProgress, NoEffectCause> {
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
            RelationalPublicationOutcome::Stale(_) => Err(NoEffectCause::StaleExpectedProductHead),
            RelationalPublicationOutcome::Denied(_) => Err(NoEffectCause::OwnerUnavailable),
            RelationalPublicationOutcome::Interrupted(_)
            | RelationalPublicationOutcome::Deferred(_)
            | RelationalPublicationOutcome::Failed(_) => Err(NoEffectCause::PreEffectFailure),
        }
    }
}
