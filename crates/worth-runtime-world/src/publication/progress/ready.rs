use super::*;

impl CompositeAttemptProgress {
    pub(crate) fn into_ready_results(
        self,
    ) -> Result<(Self, crate::publication::CompositeOwnerExecutionResults), Self> {
        let relational_ready = matches!(
            self.relational.posture,
            RelationalAttemptProgressPosture::Untouched | RelationalAttemptProgressPosture::Settled
        ) || self.relational.is_fork_only();
        let signal_ready = matches!(
            self.signal.posture,
            SignalAttemptProgressPosture::Untouched | SignalAttemptProgressPosture::Performed
        );
        if !relational_ready || !signal_ready || self.owner_effect_count() == 0 {
            return Err(self);
        }

        let Self { relational, signal } = self;
        let signal_posture = signal.posture;
        let RelationalAttemptProgress {
            posture: relational_posture,
            evidence: relational_evidence,
            fork,
            fork_successor_basis,
        } = relational;
        let relational_result = match (relational_evidence, fork, fork_successor_basis) {
            (None, None, None)
                if relational_posture == RelationalAttemptProgressPosture::Untouched =>
            {
                crate::publication::CompositeRelationalOwnerResult::retained()
            }
            (None, Some(fork), Some(successor_basis))
                if relational_posture == RelationalAttemptProgressPosture::Performed =>
            {
                crate::publication::CompositeRelationalOwnerResult::forked(fork, successor_basis)
            }
            (
                Some(RelationalProgressEvidence::Settled {
                    commit_identity,
                    successor_basis,
                    result,
                }),
                None,
                None,
            ) => crate::publication::CompositeRelationalOwnerResult::settled(
                commit_identity,
                successor_basis,
                result,
            ),
            (
                Some(RelationalProgressEvidence::Settled {
                    commit_identity,
                    successor_basis,
                    result,
                }),
                Some(fork),
                None,
            ) => crate::publication::CompositeRelationalOwnerResult::settled_after_fork(
                fork,
                commit_identity,
                successor_basis,
                result,
            ),
            (Some(RelationalProgressEvidence::SettlementPending { .. }), _, _) => {
                unreachable!("pending Relational progress is rejected above")
            }
            (Some(RelationalProgressEvidence::SettlementRequired { .. }), _, _) => {
                unreachable!("required Relational settlement is rejected above")
            }
            (Some(RelationalProgressEvidence::SettledReceipt { .. }), _, _) => {
                unreachable!("receipt-only settlement is recovery evidence")
            }
            _ => unreachable!("ready Relational progress carries matching evidence"),
        };
        let signal_result = match signal.evidence {
            None if signal_posture == SignalAttemptProgressPosture::Untouched => {
                crate::publication::CompositeSignalOwnerResult::retained()
            }
            Some(SignalProgressEvidence::Advanced(advanced)) => {
                crate::publication::CompositeSignalOwnerResult::advanced(advanced)
            }
            Some(SignalProgressEvidence::Forked(forked)) => {
                crate::publication::CompositeSignalOwnerResult::forked(forked)
            }
            Some(SignalProgressEvidence::ForkedAndAdvanced { forked, advanced }) => {
                crate::publication::CompositeSignalOwnerResult::forked_and_advanced(
                    forked, advanced,
                )
            }
            _ => unreachable!("ready Signal progress carries matching evidence"),
        };
        let summary = Self {
            relational: RelationalAttemptProgress {
                posture: relational_posture,
                evidence: None,
                fork: None,
                fork_successor_basis: None,
            },
            signal: SignalAttemptProgress {
                posture: signal_posture,
                evidence: None,
            },
        };
        Ok((
            summary,
            crate::publication::CompositeOwnerExecutionResults::from_components(
                relational_result,
                signal_result,
            ),
        ))
    }
}
