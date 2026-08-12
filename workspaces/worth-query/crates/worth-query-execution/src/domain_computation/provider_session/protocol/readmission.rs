use super::{
    WorthQueryAdmittedProviderExecutionPlan, WorthQueryProviderExecutionPlanView,
    WorthQueryProviderSessionAffinity, WorthQueryProviderSessionDenialKind,
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolCounters,
    WorthQueryProviderSessionProtocolStage, WorthQueryProviderSessionRecoveryPosture,
    WorthQueryProviderSessionTokenAdmission,
};

mod prepared;

pub use prepared::{
    WorthQueryPreparedProviderSession, WorthQuerySessionBoundReadsAndEffects,
    WorthQuerySessionEffectAuthority, WorthQuerySessionPrepareOutcome,
    WorthQuerySessionReadAuthority,
};

/// Readmitted live provider session. Only this owner can mint the phase; its
/// child transition owner may consume it, but protocol siblings cannot relabel
/// another affinity/counter pair as a readmitted session.
pub struct WorthQueryProviderPlanReadmission<'run> {
    affinity: WorthQueryProviderSessionAffinity<'run>,
    counters: WorthQueryProviderSessionProtocolCounters,
}

impl std::fmt::Debug for WorthQueryProviderPlanReadmission<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryProviderPlanReadmission")
            .field("plan_identity", &self.affinity.plan().identity())
            .field(
                "token_identity",
                &self.affinity.session().token().identity(),
            )
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
            affinity: WorthQueryProviderSessionAffinity::mint(
                self.run,
                self.contract,
                self.provider,
                token,
            ),
            counters: self.counters,
        })
    }
}

impl WorthQueryProviderPlanReadmission<'_> {
    pub fn plan_identity(&self) -> &str {
        self.affinity.plan().identity()
    }

    pub fn token_identity(&self) -> &str {
        self.affinity.session().token().identity()
    }

    pub fn token_generation(&self) -> u64 {
        self.affinity.session().token().generation()
    }

    pub fn counters(&self) -> WorthQueryProviderSessionProtocolCounters {
        self.counters
    }
}
