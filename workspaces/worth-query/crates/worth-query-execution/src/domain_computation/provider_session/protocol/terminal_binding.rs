use super::{
    WorthQueryProviderSessionAffinity, WorthQueryProviderSessionAffinityIdentity,
    WorthQuerySessionBinding,
};

/// Immutable proof that one provider session reached a terminal transition.
///
/// The live affinity owner mints this projection. It carries no lease or retry
/// authority, and its fields stay private so a domain sibling cannot assemble
/// a session identity, token binding, and execution plan independently.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) struct WorthQueryProviderSessionTerminalBinding {
    affinity: WorthQueryProviderSessionAffinityIdentity,
    session: WorthQuerySessionBinding,
    plan: super::WorthQueryProviderExecutionPlanContract,
}

impl WorthQueryProviderSessionTerminalBinding {
    pub(super) fn from_affinity(affinity: &WorthQueryProviderSessionAffinity<'_>) -> Self {
        Self {
            affinity: WorthQueryProviderSessionAffinityIdentity::from_token(
                affinity.session().token(),
            ),
            session: affinity.binding().clone(),
            plan: affinity.plan().clone(),
        }
    }

    pub(in crate::domain_computation) const fn affinity_identity(
        &self,
    ) -> WorthQueryProviderSessionAffinityIdentity {
        self.affinity
    }

    pub(in crate::domain_computation) fn admits_mutation_run(
        &self,
        session: super::super::WorthQueryGraphWorkSessionIdentity,
        managed_run: super::super::WorthQueryGraphWorkManagedRunIdentity,
        worker: &str,
    ) -> bool {
        self.plan.managed_run_identity() == worker
            && self.plan.graph_work_session_identity() == Some(session.as_u64())
            && self.plan.graph_work_managed_run_identity() == Some(managed_run.as_u64())
    }

    pub(in crate::domain_computation) const fn plan(
        &self,
    ) -> &super::WorthQueryProviderExecutionPlanContract {
        &self.plan
    }

    pub(in crate::domain_computation) fn admits_session_view(
        &self,
        session: super::WorthQueryProviderSessionView<'_>,
    ) -> bool {
        self.affinity == session.affinity_identity()
            && self.session.token_identity() == session.identity()
            && self.session.token_generation() == session.generation()
            && self.session.provider_identity() == session.provider_identity()
            && self.session.provider_generation() == session.provider_generation()
            && self.plan.provider_identity() == session.provider_identity()
            && self.plan.provider_generation() == session.provider_generation()
            && self.plan.identity() == session.plan_identity()
    }

    pub(in crate::domain_computation) fn same_session(&self, other: &Self) -> bool {
        self == other
    }

    pub(in crate::domain_computation) fn admits_cleanup_binding(
        &self,
        cleanup: &crate::domain_computation::provider_session::WorthQueryProvisionalOverlayCleanupBinding,
    ) -> bool {
        self.affinity == cleanup.affinity_identity()
            && self.session.token_identity() == cleanup.token_identity()
            && self.session.token_generation() == cleanup.token_generation()
            && self.session.provider_identity() == cleanup.provider_identity()
            && self.session.provider_generation() == cleanup.provider_generation()
            && self.plan.identity() == cleanup.plan_identity()
    }
}
