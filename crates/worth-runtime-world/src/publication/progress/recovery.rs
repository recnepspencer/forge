use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_relational::facade::history::RelationalCommitIdentity;
use worth_relational::facade::mvcc::PerformedRelationalCommit;
use worth_relational::facade::publication::DeferredPublicationSettlement;

use super::{
    CompositeAttemptProgress, RelationalAttemptProgress, RelationalAttemptProgressPosture,
    RelationalProgressEvidence, SignalAttemptProgress, SignalAttemptProgressPosture,
};

impl CompositeAttemptProgress {
    pub(crate) fn into_relational_recovery_parts(
        self,
    ) -> Result<
        (
            RelationalCommitIdentity,
            AdmittedRelationalBranchBasis,
            RelationalRecoveryRoute,
            SignalAttemptProgressPosture,
        ),
        Self,
    > {
        if self.signal.evidence.is_some() {
            return Err(self);
        }
        let Self { relational, signal } = self;
        let RelationalAttemptProgress { posture, evidence } = relational;
        let route = match evidence {
            Some(RelationalProgressEvidence::Performed(performed))
                if posture == RelationalAttemptProgressPosture::Performed =>
            {
                (
                    performed.commit_identity(),
                    performed.next_basis().clone(),
                    RelationalRecoveryRoute::Performed(performed),
                    signal.posture,
                )
            }
            Some(RelationalProgressEvidence::SettlementPending {
                commit_identity,
                successor_basis,
                settlement,
            }) if posture == RelationalAttemptProgressPosture::SettlementPending => (
                commit_identity,
                successor_basis,
                RelationalRecoveryRoute::SettlementPending(settlement),
                signal.posture,
            ),
            Some(RelationalProgressEvidence::SettlementRequired {
                commit_identity,
                successor_basis,
            }) if posture == RelationalAttemptProgressPosture::SettlementRequired => (
                commit_identity,
                successor_basis,
                RelationalRecoveryRoute::IdentityRequired,
                signal.posture,
            ),
            evidence => {
                return Err(Self {
                    relational: RelationalAttemptProgress { posture, evidence },
                    signal,
                })
            }
        };
        Ok(route)
    }

    /// Turn the one legal incomplete owner state into recovery evidence. A
    /// pending Relational settlement is already an owner effect, but it is
    /// not publication-ready and it must prevent any Signal contact. The
    /// deferred settlement is cloneable route evidence; the original stays
    /// in the retained progress row while the result projection describes
    /// the same exact occurrence.
    pub(crate) fn into_recovery_results(
        self,
    ) -> Result<(Self, crate::publication::CompositeOwnerExecutionResults), Self> {
        let Self { relational, signal } = self;
        let Some(relational_result) = recovery_result(&relational) else {
            return Err(Self { relational, signal });
        };
        let (signal, signal_result) = match signal.into_recovery_result() {
            Ok(result) => result,
            Err(signal) => {
                return Err(Self { relational, signal });
            }
        };
        let summary = Self { relational, signal };
        Ok((
            summary,
            crate::publication::CompositeOwnerExecutionResults::from_components(
                relational_result,
                signal_result,
            ),
        ))
    }
}

fn recovery_result(
    progress: &RelationalAttemptProgress,
) -> Option<crate::publication::CompositeRelationalOwnerResult> {
    match progress.evidence.as_ref() {
        None if progress.posture() == RelationalAttemptProgressPosture::Untouched => {
            Some(crate::publication::CompositeRelationalOwnerResult::retained())
        }
        Some(RelationalProgressEvidence::Performed(performed)) => Some(
            crate::publication::CompositeRelationalOwnerResult::settlement_required(
                performed.commit_identity(),
                performed.next_basis().clone(),
            ),
        ),
        Some(RelationalProgressEvidence::SettlementPending {
            commit_identity,
            successor_basis,
            settlement,
        }) => Some(
            crate::publication::CompositeRelationalOwnerResult::settlement_pending(
                commit_identity.clone(),
                successor_basis.clone(),
                settlement.clone(),
            ),
        ),
        Some(RelationalProgressEvidence::SettlementRequired {
            commit_identity,
            successor_basis,
        }) => Some(
            crate::publication::CompositeRelationalOwnerResult::settlement_required(
                commit_identity.clone(),
                successor_basis.clone(),
            ),
        ),
        Some(RelationalProgressEvidence::Settled {
            commit_identity,
            successor_basis,
            result,
        }) => Some(crate::publication::CompositeRelationalOwnerResult::settled(
            commit_identity.clone(),
            successor_basis.clone(),
            result.clone(),
        )),
        _ => None,
    }
}

pub(crate) enum RelationalRecoveryRoute {
    Performed(PerformedRelationalCommit),
    SettlementPending(DeferredPublicationSettlement),
    IdentityRequired,
}

impl SignalAttemptProgress {
    fn into_recovery_result(
        self,
    ) -> Result<
        (
            SignalAttemptProgress,
            crate::publication::CompositeSignalOwnerResult,
        ),
        Self,
    > {
        let Self { posture, evidence } = self;
        let result = match evidence {
            None if posture == SignalAttemptProgressPosture::Untouched
                || posture == SignalAttemptProgressPosture::PreparedForExecution =>
            {
                crate::publication::CompositeSignalOwnerResult::retained()
            }
            Some(super::SignalProgressEvidence::Prepared)
                if posture == SignalAttemptProgressPosture::PreparedForExecution =>
            {
                crate::publication::CompositeSignalOwnerResult::retained()
            }
            Some(super::SignalProgressEvidence::Advanced(outcome))
                if posture == SignalAttemptProgressPosture::Performed =>
            {
                crate::publication::CompositeSignalOwnerResult::advanced(outcome)
            }
            Some(super::SignalProgressEvidence::Forked(outcome))
                if posture == SignalAttemptProgressPosture::Performed =>
            {
                crate::publication::CompositeSignalOwnerResult::forked(outcome)
            }
            Some(super::SignalProgressEvidence::ForkedAndAdvanced { forked, advanced })
                if posture == SignalAttemptProgressPosture::Performed =>
            {
                crate::publication::CompositeSignalOwnerResult::forked_and_advanced(
                    forked, advanced,
                )
            }
            evidence => return Err(Self { posture, evidence }),
        };
        Ok((Self::summary(posture), result))
    }
}
