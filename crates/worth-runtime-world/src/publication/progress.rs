use worth_relational::facade::history::RelationalCommitReceipt;
use worth_relational::facade::mvcc::{
    PerformedRelationalCommit, PreparedRelationalCommitCandidate,
};
use worth_relational::facade::publication::DeferredPublicationSettlement;
use worth_signal::facade::branch::{SignalBranchAdvanceOutcome, SignalBranchForkOutcome};

/// Exact Relational owner progress. A generic ordinal cannot say which owner
/// evidence or settlement obligation is alive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalAttemptProgressPosture {
    Untouched,
    Prepared,
    Performed,
    SettlementPending,
    Settled,
}

#[derive(Debug)]
pub(super) enum RelationalProgressEvidence {
    Prepared(PreparedRelationalCommitCandidate),
    Performed(PerformedRelationalCommit),
    SettlementPending {
        performed: PerformedRelationalCommit,
        settlement: DeferredPublicationSettlement,
    },
    Settled {
        performed: PerformedRelationalCommit,
        receipt: RelationalCommitReceipt,
    },
}

#[derive(Debug)]
pub struct RelationalAttemptProgress {
    posture: RelationalAttemptProgressPosture,
    evidence: Option<RelationalProgressEvidence>,
}

impl RelationalAttemptProgress {
    pub(crate) fn untouched() -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Untouched,
            evidence: None,
        }
    }

    pub(crate) fn prepared(candidate: PreparedRelationalCommitCandidate) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Prepared,
            evidence: Some(RelationalProgressEvidence::Prepared(candidate)),
        }
    }

    pub(crate) fn performed_settlement_pending(
        performed: PerformedRelationalCommit,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::SettlementPending,
            evidence: Some(RelationalProgressEvidence::SettlementPending {
                performed,
                settlement,
            }),
        }
    }

    pub(crate) fn performed(performed: PerformedRelationalCommit) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Performed,
            evidence: Some(RelationalProgressEvidence::Performed(performed)),
        }
    }

    pub(crate) fn settled(
        performed: PerformedRelationalCommit,
        receipt: RelationalCommitReceipt,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Settled,
            evidence: Some(RelationalProgressEvidence::Settled { performed, receipt }),
        }
    }

    pub const fn posture(&self) -> RelationalAttemptProgressPosture {
        self.posture
    }

    pub(super) fn into_evidence(self) -> Option<RelationalProgressEvidence> {
        self.evidence
    }
}

/// Exact Signal owner progress. Signal has no Relational settlement state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalAttemptProgressPosture {
    Untouched,
    PreparedForExecution,
    Performed,
}

#[derive(Debug)]
pub(super) enum SignalProgressEvidence {
    Prepared,
    Advanced(SignalBranchAdvanceOutcome),
    Forked(SignalBranchForkOutcome),
}

#[derive(Debug)]
pub struct SignalAttemptProgress {
    posture: SignalAttemptProgressPosture,
    evidence: Option<SignalProgressEvidence>,
}

impl SignalAttemptProgress {
    pub(crate) fn untouched() -> Self {
        Self {
            posture: SignalAttemptProgressPosture::Untouched,
            evidence: None,
        }
    }

    pub(crate) fn prepared_for_execution() -> Self {
        Self {
            posture: SignalAttemptProgressPosture::PreparedForExecution,
            evidence: Some(SignalProgressEvidence::Prepared),
        }
    }

    pub(crate) fn advanced(outcome: SignalBranchAdvanceOutcome) -> Self {
        Self {
            posture: SignalAttemptProgressPosture::Performed,
            evidence: Some(SignalProgressEvidence::Advanced(outcome)),
        }
    }

    pub(crate) fn forked(outcome: SignalBranchForkOutcome) -> Self {
        Self {
            posture: SignalAttemptProgressPosture::Performed,
            evidence: Some(SignalProgressEvidence::Forked(outcome)),
        }
    }

    pub const fn posture(&self) -> SignalAttemptProgressPosture {
        self.posture
    }

    pub(super) fn into_evidence(self) -> Option<SignalProgressEvidence> {
        self.evidence
    }
}

#[derive(Debug)]
pub struct CompositeAttemptProgress {
    relational: RelationalAttemptProgress,
    signal: SignalAttemptProgress,
}

impl CompositeAttemptProgress {
    pub(crate) fn untouched() -> Self {
        Self {
            relational: RelationalAttemptProgress::untouched(),
            signal: SignalAttemptProgress::untouched(),
        }
    }

    pub(crate) fn new(
        relational: RelationalAttemptProgress,
        signal: SignalAttemptProgress,
    ) -> Self {
        Self { relational, signal }
    }

    pub const fn relational_posture(&self) -> RelationalAttemptProgressPosture {
        self.relational.posture()
    }

    pub const fn signal_posture(&self) -> SignalAttemptProgressPosture {
        self.signal.posture()
    }

    pub fn relational(&self) -> &RelationalAttemptProgress {
        &self.relational
    }

    pub fn signal(&self) -> &SignalAttemptProgress {
        &self.signal
    }

    pub(crate) const fn owner_effect_count(&self) -> usize {
        owner_effect_count_from_postures(self.relational.posture(), self.signal.posture())
    }

    pub(crate) fn into_parts(self) -> (RelationalAttemptProgress, SignalAttemptProgress) {
        (self.relational, self.signal)
    }

    pub(super) fn into_ready_results(
        self,
    ) -> Result<(Self, super::CompositeOwnerExecutionResults), Self> {
        let relational_ready = matches!(
            self.relational.posture,
            RelationalAttemptProgressPosture::Untouched
                | RelationalAttemptProgressPosture::Performed
                | RelationalAttemptProgressPosture::Settled
        );
        let signal_ready = matches!(
            self.signal.posture,
            SignalAttemptProgressPosture::Untouched | SignalAttemptProgressPosture::Performed
        );
        if !relational_ready || !signal_ready || self.owner_effect_count() == 0 {
            return Err(self);
        }

        let Self { relational, signal } = self;
        let relational_posture = relational.posture;
        let signal_posture = signal.posture;
        let relational_result = match relational.evidence {
            None if relational_posture == RelationalAttemptProgressPosture::Untouched => {
                super::CompositeRelationalOwnerResult::retained()
            }
            Some(RelationalProgressEvidence::Performed(performed)) => {
                super::CompositeRelationalOwnerResult::published(performed, None)
            }
            Some(RelationalProgressEvidence::Settled { performed, receipt }) => {
                super::CompositeRelationalOwnerResult::published(performed, Some(receipt))
            }
            _ => unreachable!("ready Relational progress carries matching evidence"),
        };
        let signal_result = match signal.evidence {
            None if signal_posture == SignalAttemptProgressPosture::Untouched => {
                super::CompositeSignalOwnerResult::retained()
            }
            Some(SignalProgressEvidence::Advanced(advanced)) => {
                super::CompositeSignalOwnerResult::advanced(advanced)
            }
            Some(SignalProgressEvidence::Forked(forked)) => {
                super::CompositeSignalOwnerResult::forked(forked)
            }
            _ => unreachable!("ready Signal progress carries matching evidence"),
        };
        let summary = Self {
            relational: RelationalAttemptProgress {
                posture: relational_posture,
                evidence: None,
            },
            signal: SignalAttemptProgress {
                posture: signal_posture,
                evidence: None,
            },
        };
        Ok((
            summary,
            super::CompositeOwnerExecutionResults::from_components(
                relational_result,
                signal_result,
            ),
        ))
    }
}

const fn owner_effect_count_from_postures(
    relational: RelationalAttemptProgressPosture,
    signal: SignalAttemptProgressPosture,
) -> usize {
    let relational = match relational {
        RelationalAttemptProgressPosture::Untouched
        | RelationalAttemptProgressPosture::Prepared => 0,
        RelationalAttemptProgressPosture::Performed
        | RelationalAttemptProgressPosture::SettlementPending
        | RelationalAttemptProgressPosture::Settled => 1,
    };
    let signal = match signal {
        SignalAttemptProgressPosture::Untouched
        | SignalAttemptProgressPosture::PreparedForExecution => 0,
        SignalAttemptProgressPosture::Performed => 1,
    };
    relational + signal
}

#[cfg(test)]
mod tests {
    use super::{
        owner_effect_count_from_postures, RelationalAttemptProgressPosture,
        SignalAttemptProgressPosture,
    };

    #[test]
    fn owner_effect_projection_covers_zero_one_and_two_performed_owners() {
        let cases = [
            (
                RelationalAttemptProgressPosture::Prepared,
                SignalAttemptProgressPosture::PreparedForExecution,
                0,
            ),
            (
                RelationalAttemptProgressPosture::Performed,
                SignalAttemptProgressPosture::PreparedForExecution,
                1,
            ),
            (
                RelationalAttemptProgressPosture::Settled,
                SignalAttemptProgressPosture::Performed,
                2,
            ),
        ];
        for (relational, signal, expected) in cases {
            assert_eq!(
                owner_effect_count_from_postures(relational, signal),
                expected
            );
        }
    }
}
