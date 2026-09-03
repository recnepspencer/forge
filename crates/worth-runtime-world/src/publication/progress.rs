use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_relational::facade::history::RelationalCommitIdentity;
use worth_relational::facade::mvcc::PerformedRelationalCommit;
use worth_relational::facade::publication::DeferredPublicationSettlement;
use worth_relational::facade::transactions::CommitResult;
use worth_signal::facade::branch::{SignalBranchAdvanceOutcome, SignalBranchForkOutcome};

#[path = "progress/recovery.rs"]
mod recovery;
pub(crate) use recovery::RelationalRecoveryRoute;

#[path = "progress/preparation.rs"]
mod preparation;

#[path = "progress/effect_count.rs"]
mod effect_count;
pub(crate) use effect_count::owner_effect_count_from_postures;

/// Exact Relational owner progress. A generic ordinal cannot say which owner
/// evidence or settlement obligation is alive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalAttemptProgressPosture {
    Untouched,
    Prepared,
    Performed,
    SettlementRequired,
    SettlementPending,
    Settled,
}

#[derive(Debug)]
pub(super) enum RelationalProgressEvidence {
    Performed(PerformedRelationalCommit),
    SettlementRequired {
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    },
    SettlementPending {
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    },
    SettledReceipt {
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    },
    Settled {
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        result: CommitResult,
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

    /// Record a performed Relational route after its owner consumed the
    /// linear performed witness and returned a deferred settlement. The
    /// successor basis is captured before that consuming owner call; the
    /// deferred settlement is the remaining owner-issued repair authority.
    pub(crate) fn settlement_pending(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        settlement: DeferredPublicationSettlement,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::SettlementPending,
            evidence: Some(RelationalProgressEvidence::SettlementPending {
                commit_identity,
                successor_basis,
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

    /// Record the exact result returned by the consuming Relational
    /// settlement call. The commit receipt is retained inside `CommitResult`;
    /// no second performed witness is manufactured.
    pub(crate) fn settled(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        result: CommitResult,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Settled,
            evidence: Some(RelationalProgressEvidence::Settled {
                commit_identity,
                successor_basis,
                result,
            }),
        }
    }

    pub const fn posture(&self) -> RelationalAttemptProgressPosture {
        self.posture
    }

    pub(crate) fn successor_basis(&self) -> Option<&AdmittedRelationalBranchBasis> {
        match self.evidence.as_ref() {
            Some(RelationalProgressEvidence::Performed(performed)) => Some(performed.next_basis()),
            Some(RelationalProgressEvidence::SettlementRequired {
                successor_basis, ..
            })
            | Some(RelationalProgressEvidence::SettlementPending {
                successor_basis, ..
            })
            | Some(RelationalProgressEvidence::Settled {
                successor_basis, ..
            }) => Some(successor_basis),
            Some(RelationalProgressEvidence::SettledReceipt {
                successor_basis, ..
            }) => Some(successor_basis),
            None => None,
        }
    }

    pub(crate) fn settled_receipt(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::Settled,
            evidence: Some(RelationalProgressEvidence::SettledReceipt {
                commit_identity,
                successor_basis,
                receipt,
            }),
        }
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
    ForkedAndAdvanced {
        forked: SignalBranchForkOutcome,
        advanced: SignalBranchAdvanceOutcome,
    },
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

    pub(crate) const fn summary(posture: SignalAttemptProgressPosture) -> Self {
        Self {
            posture,
            evidence: None,
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

    pub(crate) fn forked_and_advanced(
        forked: SignalBranchForkOutcome,
        advanced: SignalBranchAdvanceOutcome,
    ) -> Self {
        Self {
            posture: SignalAttemptProgressPosture::Performed,
            evidence: Some(SignalProgressEvidence::ForkedAndAdvanced { forked, advanced }),
        }
    }

    pub const fn posture(&self) -> SignalAttemptProgressPosture {
        self.posture
    }

    pub(crate) fn successor_basis(
        &self,
    ) -> Option<&worth_signal::facade::branch::AdmittedSignalBranchBasis> {
        match self.evidence.as_ref() {
            Some(SignalProgressEvidence::Advanced(outcome)) => Some(outcome.advanced_basis()),
            Some(SignalProgressEvidence::Forked(outcome)) => Some(outcome.created_basis()),
            Some(SignalProgressEvidence::ForkedAndAdvanced { advanced, .. }) => {
                Some(advanced.advanced_basis())
            }
            _ => None,
        }
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

impl RelationalAttemptProgress {
    pub(crate) fn settlement_required(
        commit_identity: RelationalCommitIdentity,
        successor_basis: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self {
            posture: RelationalAttemptProgressPosture::SettlementRequired,
            evidence: Some(RelationalProgressEvidence::SettlementRequired {
                commit_identity,
                successor_basis,
            }),
        }
    }
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
            RelationalAttemptProgressPosture::Untouched | RelationalAttemptProgressPosture::Settled
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
            Some(RelationalProgressEvidence::Settled {
                commit_identity,
                successor_basis,
                result,
            }) => super::CompositeRelationalOwnerResult::settled(
                commit_identity,
                successor_basis,
                result,
            ),
            Some(RelationalProgressEvidence::SettlementPending { .. }) => {
                unreachable!("pending Relational progress is rejected above")
            }
            Some(RelationalProgressEvidence::SettlementRequired { .. }) => {
                unreachable!("required Relational settlement is rejected above")
            }
            Some(RelationalProgressEvidence::SettledReceipt { .. }) => {
                unreachable!("receipt-only settlement is recovery evidence")
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
            Some(SignalProgressEvidence::ForkedAndAdvanced { forked, advanced }) => {
                super::CompositeSignalOwnerResult::forked_and_advanced(forked, advanced)
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

#[cfg(test)]
#[path = "progress/tests.rs"]
mod tests;
