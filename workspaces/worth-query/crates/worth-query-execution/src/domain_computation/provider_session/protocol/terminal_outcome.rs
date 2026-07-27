use super::{
    WorthQueryProviderSessionDenialKind, WorthQueryProviderSessionFailure,
    WorthQueryProviderSessionProtocolCounters, WorthQueryProviderSessionProtocolStage,
    WorthQueryProviderSessionRecoveryPosture, WorthQuerySessionBoundReadsAndEffects,
    WorthQuerySessionPrepareOutcome,
};

#[derive(Debug)]
pub enum WorthQuerySessionCommitOrAbortOutcome {
    Committed {
        plan_identity: String,
        token_identity: String,
        provider_receipt: String,
        counters: WorthQueryProviderSessionProtocolCounters,
    },
    Aborted {
        plan_identity: String,
        token_identity: String,
        provider_receipt: String,
        counters: WorthQueryProviderSessionProtocolCounters,
    },
    CommitRecoveryRequired(WorthQueryProviderSessionFailure),
    AbortRecoveryRequired(WorthQueryProviderSessionFailure),
}

impl WorthQuerySessionCommitOrAbortOutcome {
    pub fn recovery_posture(&self) -> WorthQueryProviderSessionRecoveryPosture {
        match self {
            Self::Committed { .. } | Self::Aborted { .. } => {
                WorthQueryProviderSessionRecoveryPosture::Closed
            }
            Self::CommitRecoveryRequired(_) | Self::AbortRecoveryRequired(_) => {
                WorthQueryProviderSessionRecoveryPosture::RecoveryRequired
            }
        }
    }

    pub fn failure(&self) -> Option<&WorthQueryProviderSessionFailure> {
        match self {
            Self::CommitRecoveryRequired(failure) | Self::AbortRecoveryRequired(failure) => {
                Some(failure)
            }
            Self::Committed { .. } | Self::Aborted { .. } => None,
        }
    }
}

impl WorthQuerySessionBoundReadsAndEffects<'_> {
    pub fn abort(mut self) -> WorthQuerySessionCommitOrAbortOutcome {
        self.counters.called_provider();
        let invocation =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.session.abort()));
        match invocation {
            Ok(Ok(provider_receipt)) => WorthQuerySessionCommitOrAbortOutcome::Aborted {
                plan_identity: self.contract.identity().to_owned(),
                token_identity: self.binding.token_identity().to_owned(),
                provider_receipt,
                counters: self.counters,
            },
            Ok(Err(failure)) => WorthQuerySessionCommitOrAbortOutcome::AbortRecoveryRequired(
                failure
                    .at_stage(WorthQueryProviderSessionProtocolStage::Abort, self.counters)
                    .with_recovery_posture(
                        WorthQueryProviderSessionRecoveryPosture::RecoveryRequired,
                    ),
            ),
            Err(_) => WorthQuerySessionCommitOrAbortOutcome::AbortRecoveryRequired(
                WorthQueryProviderSessionFailure::new(
                    WorthQueryProviderSessionDenialKind::ProviderPanicked,
                    WorthQueryProviderSessionProtocolStage::Abort,
                    "provider panicked while aborting the session",
                    self.counters,
                )
                .with_recovery_posture(WorthQueryProviderSessionRecoveryPosture::RecoveryRequired),
            ),
        }
    }
}

impl WorthQuerySessionPrepareOutcome<'_> {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "Phase 11 is the first production owner of provider commit"
        )
    )]
    pub(crate) fn commit(mut self) -> WorthQuerySessionCommitOrAbortOutcome {
        self.counters.called_provider();
        let invocation =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.session.commit()));
        match invocation {
            Ok(Ok(provider_receipt)) => WorthQuerySessionCommitOrAbortOutcome::Committed {
                plan_identity: self.contract.identity().to_owned(),
                token_identity: self.binding.token_identity().to_owned(),
                provider_receipt,
                counters: self.counters,
            },
            Ok(Err(failure)) => WorthQuerySessionCommitOrAbortOutcome::CommitRecoveryRequired(
                failure
                    .at_stage(
                        WorthQueryProviderSessionProtocolStage::Commit,
                        self.counters,
                    )
                    .with_recovery_posture(
                        WorthQueryProviderSessionRecoveryPosture::RecoveryRequired,
                    ),
            ),
            Err(_) => WorthQuerySessionCommitOrAbortOutcome::CommitRecoveryRequired(
                WorthQueryProviderSessionFailure::new(
                    WorthQueryProviderSessionDenialKind::ProviderPanicked,
                    WorthQueryProviderSessionProtocolStage::Commit,
                    "provider panicked while committing the prepared session",
                    self.counters,
                )
                .with_recovery_posture(WorthQueryProviderSessionRecoveryPosture::RecoveryRequired),
            ),
        }
    }
}
