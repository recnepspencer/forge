use std::sync::Arc;

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
    managed_run: Arc<str>,
    graph_work_session: Option<u64>,
    graph_work_managed_run: Option<u64>,
}

impl WorthQueryProviderSessionTerminalBinding {
    pub(super) fn from_affinity(affinity: &WorthQueryProviderSessionAffinity<'_>) -> Self {
        Self {
            affinity: WorthQueryProviderSessionAffinityIdentity::from_token(
                affinity.session().token(),
            ),
            session: affinity.binding().clone(),
            managed_run: affinity.plan().managed_run_identity().into(),
            graph_work_session: affinity.plan().graph_work_session_identity(),
            graph_work_managed_run: affinity.plan().graph_work_managed_run_identity(),
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
        self.managed_run.as_ref() == worker
            && self.graph_work_session == Some(session.as_u64())
            && self.graph_work_managed_run == Some(managed_run.as_u64())
    }

    pub(in crate::domain_computation) fn same_session(&self, other: &Self) -> bool {
        self == other
    }
}
