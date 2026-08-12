use worth_store::physical_runtime::{
    PhysicalRecoveryCleanupRemovalOutcome, StoreRecoveryCleanupFreshnessAdmission,
    StoreRecoveryCleanupFreshnessSample,
};

use crate::handoff::RecoveryCleanupDeferralEvidence;
use crate::progression::ReopenedPhysicalRecovery;

use super::{
    command_basis::RecoveryCleanupCommandBasis, PerformedRecoveryCleanupRemoval,
    RecoveryCleanupEligibility, RecoveryCleanupPlan, RecoveryCleanupTarget,
};

pub(super) enum RecoveryCleanupAttempt {
    Completed {
        freshness: StoreRecoveryCleanupFreshnessSample,
        performed: PerformedRecoveryCleanupRemoval,
        revalidation: worth_store::physical_runtime::RecoveryCleanupArtifactRevalidationProgress,
    },
    Deferred {
        freshness: Option<StoreRecoveryCleanupFreshnessSample>,
        evidence: RecoveryCleanupDeferralEvidence,
        stop: bool,
    },
}

pub(super) struct RecoveryCleanupAttemptBasis<'recovery, 'basis> {
    reopened: &'recovery ReopenedPhysicalRecovery,
    plan: &'basis RecoveryCleanupPlan,
    command_basis: Option<&'basis RecoveryCleanupCommandBasis<'recovery>>,
}

impl<'recovery, 'basis> RecoveryCleanupAttemptBasis<'recovery, 'basis> {
    pub(super) const fn new(
        reopened: &'recovery ReopenedPhysicalRecovery,
        plan: &'basis RecoveryCleanupPlan,
        command_basis: Option<&'basis RecoveryCleanupCommandBasis<'recovery>>,
    ) -> Self {
        Self {
            reopened,
            plan,
            command_basis,
        }
    }

    pub(super) fn execute(
        &self,
        expected_policy: Option<[u8; 32]>,
        candidate: RecoveryCleanupEligibility,
    ) -> RecoveryCleanupAttempt {
        let target = RecoveryCleanupTarget::Wal(candidate.artifact());
        let admission = match self.sample_freshness(target.clone(), candidate) {
            Ok(admission) => admission,
            Err(attempt) => return attempt,
        };
        if let Some(evidence) =
            self.changed_evidence(target.clone(), admission.sample(), expected_policy)
        {
            let (freshness, _) = admission.into_parts();
            return RecoveryCleanupAttempt::Deferred {
                freshness: Some(freshness),
                evidence,
                stop: true,
            };
        }
        let (freshness, command) = admission.into_parts();
        let Some(command) = command else {
            return RecoveryCleanupAttempt::Deferred {
                evidence: RecoveryCleanupDeferralEvidence::EligibilityChanged { target },
                freshness: Some(freshness),
                stop: true,
            };
        };
        lower_removal_outcome(self.reopened, target, freshness, command)
    }

    fn sample_freshness(
        &self,
        target: RecoveryCleanupTarget,
        candidate: RecoveryCleanupEligibility,
    ) -> Result<StoreRecoveryCleanupFreshnessAdmission, RecoveryCleanupAttempt> {
        self.command_basis
            .ok_or_else(|| RecoveryCleanupAttempt::Deferred {
                freshness: None,
                evidence: RecoveryCleanupDeferralEvidence::EligibilityChanged {
                    target: target.clone(),
                },
                stop: true,
            })?
            .admit(
                self.reopened.state.coordination.owner(),
                &self.reopened.state.authority.media,
                candidate.artifact(),
            )
            .map_err(|failure| RecoveryCleanupAttempt::Deferred {
                freshness: None,
                evidence: RecoveryCleanupDeferralEvidence::Freshness { target, failure },
                stop: true,
            })
    }

    fn changed_evidence(
        &self,
        target: RecoveryCleanupTarget,
        freshness: &StoreRecoveryCleanupFreshnessSample,
        expected_policy: Option<[u8; 32]>,
    ) -> Option<RecoveryCleanupDeferralEvidence> {
        if freshness.observed_published_generation() != self.plan.published_generation() {
            Some(
                RecoveryCleanupDeferralEvidence::PublishedGenerationChanged {
                    target,
                    expected: self.plan.published_generation(),
                    observed: freshness.observed_published_generation(),
                },
            )
        } else if !freshness_matches(self.reopened, self.plan, freshness, expected_policy) {
            Some(RecoveryCleanupDeferralEvidence::EligibilityChanged { target })
        } else {
            None
        }
    }
}

fn lower_removal_outcome(
    reopened: &ReopenedPhysicalRecovery,
    target: RecoveryCleanupTarget,
    freshness: StoreRecoveryCleanupFreshnessSample,
    command: worth_store::physical_runtime::PhysicalRecoveryCleanupRemovalCommand,
) -> RecoveryCleanupAttempt {
    match reopened
        .state
        .coordination
        .owner()
        .execute_cleanup_removal(&reopened.state.authority.media, command)
    {
        PhysicalRecoveryCleanupRemovalOutcome::Completed(completed) => {
            let revalidation = completed.revalidation();
            RecoveryCleanupAttempt::Completed {
                freshness,
                performed: PerformedRecoveryCleanupRemoval::new(completed.into_performed()),
                revalidation,
            }
        }
        PhysicalRecoveryCleanupRemovalOutcome::DeniedBeforeEffect(denial) => {
            RecoveryCleanupAttempt::Deferred {
                freshness: Some(freshness),
                evidence: RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { target, denial },
                stop: false,
            }
        }
        PhysicalRecoveryCleanupRemovalOutcome::Indeterminate(evidence) => {
            RecoveryCleanupAttempt::Deferred {
                freshness: Some(freshness),
                evidence: RecoveryCleanupDeferralEvidence::IndeterminateEffect { target, evidence },
                stop: true,
            }
        }
    }
}

fn freshness_matches(
    reopened: &ReopenedPhysicalRecovery,
    plan: &RecoveryCleanupPlan,
    sample: &StoreRecoveryCleanupFreshnessSample,
    expected_policy: Option<[u8; 32]>,
) -> bool {
    sample.store_identity() == reopened.store_identity()
        && Some(sample.cleanup_plan_identity()) == plan.authority_identity()
        && sample.sealed_publication_basis() == reopened.expectation.plan_identity()
        && sample.policy_identity() != [0; 32]
        && expected_policy.is_none_or(|policy| policy == sample.policy_identity())
}
