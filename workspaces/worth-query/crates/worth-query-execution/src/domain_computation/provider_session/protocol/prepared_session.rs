use super::{
    WorthQueryProviderExecutionPlanContract, WorthQueryProviderPlanReadmission,
    WorthQueryProviderRunBorrow, WorthQueryProviderSessionDenialKind,
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionLease,
    WorthQueryProviderSessionProtocolCounters, WorthQueryProviderSessionProtocolStage,
};

pub struct WorthQueryPreparedProviderSession<'run> {
    pub(super) run: WorthQueryProviderRunBorrow<'run>,
    pub(super) contract: WorthQueryProviderExecutionPlanContract,
    pub(super) session: WorthQueryProviderSessionLease,
    pub(super) counters: WorthQueryProviderSessionProtocolCounters,
}

impl std::fmt::Debug for WorthQueryPreparedProviderSession<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryPreparedProviderSession")
            .field("plan_identity", &self.contract.identity())
            .field("token_identity", &self.session.token().identity())
            .finish_non_exhaustive()
    }
}

impl<'run> WorthQueryProviderPlanReadmission<'run> {
    pub fn prepare(
        mut self,
    ) -> Result<WorthQueryPreparedProviderSession<'run>, WorthQueryProviderSessionFailure> {
        self.counters.called_provider();
        let invocation = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.session
                .provider()
                .prepare_session(&self.session.token().view())
        }));
        let failure = match invocation {
            Ok(Ok(())) => {
                return Ok(WorthQueryPreparedProviderSession {
                    run: self.run,
                    contract: self.contract,
                    session: self.session,
                    counters: self.counters,
                });
            }
            Ok(Err(failure)) => failure.at_stage(
                WorthQueryProviderSessionProtocolStage::SessionPreparation,
                self.counters,
            ),
            Err(_) => WorthQueryProviderSessionFailure::new(
                WorthQueryProviderSessionDenialKind::ProviderPanicked,
                WorthQueryProviderSessionProtocolStage::SessionPreparation,
                "provider panicked while preparing the readmitted session",
                self.counters,
            ),
        };
        self.counters.called_provider();
        let posture = self.session.abort_after_failure();
        Err(failure.with_recovery_posture(posture))
    }
}

impl WorthQueryPreparedProviderSession<'_> {
    pub fn plan(&self) -> &WorthQueryProviderExecutionPlanContract {
        &self.contract
    }

    pub fn token_identity(&self) -> &str {
        self.session.token().identity()
    }

    pub fn token_generation(&self) -> u64 {
        self.session.token().generation()
    }

    pub fn counters(&self) -> WorthQueryProviderSessionProtocolCounters {
        self.counters
    }
}
