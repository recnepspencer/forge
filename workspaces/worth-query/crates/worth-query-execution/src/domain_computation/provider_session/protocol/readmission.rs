use super::{WorthQueryProviderSessionAffinity, WorthQueryProviderSessionProtocolCounters};

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

impl WorthQueryProviderPlanReadmission<'_> {
    pub(super) fn from_admitted(
        affinity: WorthQueryProviderSessionAffinity<'_>,
        counters: WorthQueryProviderSessionProtocolCounters,
        _seal: super::execution_plan::WorthQueryProviderPlanReadmissionSeal,
    ) -> WorthQueryProviderPlanReadmission<'_> {
        WorthQueryProviderPlanReadmission { affinity, counters }
    }
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
