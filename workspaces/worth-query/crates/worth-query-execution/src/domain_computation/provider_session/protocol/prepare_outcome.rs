use super::{
    WorthQueryProviderExecutionPlanContract, WorthQueryProviderRunBorrow,
    WorthQueryProviderSessionDenialKind, WorthQueryProviderSessionFailure,
    WorthQueryProviderSessionLease, WorthQueryProviderSessionProtocolCounters,
    WorthQueryProviderSessionProtocolStage, WorthQuerySessionBinding,
    WorthQuerySessionBoundReadsAndEffects,
};

pub struct WorthQuerySessionPrepareOutcome<'run> {
    pub(super) _run: WorthQueryProviderRunBorrow<'run>,
    pub(super) contract: WorthQueryProviderExecutionPlanContract,
    pub(super) session: WorthQueryProviderSessionLease,
    pub(super) binding: WorthQuerySessionBinding,
    pub(super) counters: WorthQueryProviderSessionProtocolCounters,
}

impl std::fmt::Debug for WorthQuerySessionPrepareOutcome<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQuerySessionPrepareOutcome")
            .field("plan_identity", &self.contract.identity())
            .field("token_identity", &self.binding.token_identity())
            .finish_non_exhaustive()
    }
}

impl<'run> WorthQuerySessionBoundReadsAndEffects<'run> {
    /// Phase 11 is the only production owner allowed to call this transition.
    /// The method is crate-private so public callers cannot substitute a bool,
    /// receipt string, or selected invariant list for commit-attempt authority.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "Phase 11 is the first production owner of staged prepare"
        )
    )]
    pub(crate) fn prepare_for_commit(
        mut self,
    ) -> Result<WorthQuerySessionPrepareOutcome<'run>, WorthQueryProviderSessionFailure> {
        self.counters.called_provider();
        let invocation = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.session
                .provider()
                .prepare_staged_session(&self.session.token().view())
        }));
        let failure = match invocation {
            Ok(Ok(())) => {
                return Ok(WorthQuerySessionPrepareOutcome {
                    _run: self._run,
                    contract: self.contract,
                    session: self.session,
                    binding: self.binding,
                    counters: self.counters,
                });
            }
            Ok(Err(failure)) => failure.at_stage(
                WorthQueryProviderSessionProtocolStage::StagedPreparation,
                self.counters,
            ),
            Err(_) => WorthQueryProviderSessionFailure::new(
                WorthQueryProviderSessionDenialKind::ProviderPanicked,
                WorthQueryProviderSessionProtocolStage::StagedPreparation,
                "provider panicked while preparing staged session work",
                self.counters,
            ),
        };
        self.counters.called_provider();
        let posture = self.session.abort_after_failure();
        Err(failure.with_recovery_posture(posture))
    }
}

impl WorthQuerySessionPrepareOutcome<'_> {
    pub fn plan_identity(&self) -> &str {
        self.contract.identity()
    }

    pub fn token_identity(&self) -> &str {
        self.binding.token_identity()
    }

    pub fn token_generation(&self) -> u64 {
        self.binding.token_generation()
    }

    pub fn counters(&self) -> WorthQueryProviderSessionProtocolCounters {
        self.counters
    }
}
