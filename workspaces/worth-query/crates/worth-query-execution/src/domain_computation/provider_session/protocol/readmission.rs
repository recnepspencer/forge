use super::{
    WorthQueryAdmittedProviderExecutionPlan, WorthQueryProviderExecutionPlanContract,
    WorthQueryProviderExecutionPlanView, WorthQueryProviderRunBorrow,
    WorthQueryProviderSessionDenialKind, WorthQueryProviderSessionFailure,
    WorthQueryProviderSessionLease, WorthQueryProviderSessionProtocolCounters,
    WorthQueryProviderSessionProtocolStage, WorthQueryProviderSessionRecoveryPosture,
    WorthQueryProviderSessionTokenAdmission,
};

pub struct WorthQueryProviderPlanReadmission<'run> {
    pub(super) run: WorthQueryProviderRunBorrow<'run>,
    pub(super) contract: WorthQueryProviderExecutionPlanContract,
    pub(super) session: WorthQueryProviderSessionLease,
    pub(super) counters: WorthQueryProviderSessionProtocolCounters,
}

impl std::fmt::Debug for WorthQueryProviderPlanReadmission<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryProviderPlanReadmission")
            .field("plan_identity", &self.contract.identity())
            .field("token_identity", &self.session.token().identity())
            .finish_non_exhaustive()
    }
}

impl<'run> WorthQueryAdmittedProviderExecutionPlan<'run> {
    pub fn readmit(
        mut self,
    ) -> Result<WorthQueryProviderPlanReadmission<'run>, WorthQueryProviderSessionFailure> {
        self.counters.called_provider();
        let admission = WorthQueryProviderSessionTokenAdmission::new(&self.contract);
        let invocation = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.provider.readmit_session(
                &WorthQueryProviderExecutionPlanView::new(&self.contract),
                admission,
            )
        }));
        let token = match invocation {
            Ok(Ok(token)) => token,
            Ok(Err(failure)) => {
                return Err(failure.at_stage(
                    WorthQueryProviderSessionProtocolStage::PlanReadmission,
                    self.counters,
                ));
            }
            Err(_) => {
                return Err(WorthQueryProviderSessionFailure::new(
                    WorthQueryProviderSessionDenialKind::ProviderPanicked,
                    WorthQueryProviderSessionProtocolStage::PlanReadmission,
                    "provider panicked while readmitting the sealed execution plan",
                    self.counters,
                )
                .with_recovery_posture(
                    WorthQueryProviderSessionRecoveryPosture::RecoveryRequired,
                ));
            }
        };
        if !token.belongs_to(&self.contract) {
            return Err(WorthQueryProviderSessionFailure::new(
                WorthQueryProviderSessionDenialKind::TokenNotMintedForPlan,
                WorthQueryProviderSessionProtocolStage::PlanReadmission,
                "provider returned a token minted for another plan or generation",
                self.counters,
            )
            .with_recovery_posture(WorthQueryProviderSessionRecoveryPosture::RecoveryRequired));
        }
        self.counters.minted_token();
        Ok(WorthQueryProviderPlanReadmission {
            run: self.run,
            contract: self.contract,
            session: WorthQueryProviderSessionLease::new(self.provider, token),
            counters: self.counters,
        })
    }
}

impl WorthQueryProviderPlanReadmission<'_> {
    pub fn plan_identity(&self) -> &str {
        self.contract.identity()
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
