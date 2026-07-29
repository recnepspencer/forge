mod change_classification;
mod framework_turn;
mod inspection;
mod mounted_allocation;
#[cfg(any(test, feature = "certification-support"))]
mod planning;
mod rebind_planning;
mod rebind_publication;
mod replacement;

pub(crate) use replacement::WorthUiRuntimePublicationBasis;

use crate::facade::prepared_application_authority::{
    WorthUiPreparedApplicationGenerationIdentity, WorthUiPreparedApplicationGraphSuccessor,
};
use crate::facade::registry::snapshot::CapabilitySnapshot;
use crate::facade::WorthUiApp;
use crate::graph::{UiGraphAuthority, UiGraphNodeIdentity};
use crate::runtime::{
    WorthUiRuntime, WorthUiRuntimeShutdownReceipt, WorthUiSourceEventIngress, WorthUiSourceProvider,
};

/// Application/runtime authority retained by one active session.
///
/// The active-session composition root delegates named application operations
/// here instead of lending the complete app or runtime to sibling subsystems.
pub(crate) struct WorthUiApplicationSessionState {
    app: WorthUiApp,
    runtime: WorthUiRuntime,
}

impl WorthUiApplicationSessionState {
    pub(crate) fn new(app: WorthUiApp, runtime: WorthUiRuntime) -> Self {
        Self { app, runtime }
    }

    pub(crate) fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        self.app.generation_identity()
    }

    pub(crate) fn capabilities(&self) -> &CapabilitySnapshot {
        self.app.capabilities()
    }

    pub(crate) fn graph(&self) -> UiGraphAuthority<'_> {
        self.app.graph()
    }

    pub(crate) fn graph_snapshot(&self) -> &crate::graph::UiGraphSnapshot {
        self.app.graph_snapshot()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn lookup_consumed_fact(
        &self,
        fact: &crate::fact_contract::UiProducedFact,
    ) -> Result<crate::graph::UiGraphFactLookupReceipt, crate::graph::UiGraphFactLookupDenial> {
        let prepared = self.app.prepared_authority();
        let index = prepared.consumed_fact_index();
        index.lookup(index.basis(), fact)
    }

    pub(crate) fn source_event_ingress(
        &self,
        provider: WorthUiSourceProvider,
    ) -> WorthUiSourceEventIngress {
        self.runtime.source_event_ingress(provider)
    }

    pub(crate) fn begin_observation_turn(
        &mut self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    ) -> Result<
        crate::facade::observation::UiObservationTurn<'_>,
        crate::facade::observation::UiObservationTurnDenial,
    > {
        let source_basis = self.app.capabilities().digest().as_u64();
        self.runtime.begin_observation_turn(session, source_basis)
    }

    pub(crate) fn admission(&self) -> crate::admission::UiAdmissionBoundary<'_> {
        self.app.admission()
    }

    pub(crate) fn try_allocation_touch_for_node(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Result<
        crate::obligations::touch::UiGraphTouchDescriptor,
        crate::obligations::touch::UiGraphTouchDenial,
    > {
        self.app.try_allocation_touch_for_node(graph_node_identity)
    }

    pub(crate) fn allocation_truth_revision(&self) -> crate::runtime::UiAllocationTruthRevision {
        self.runtime.allocation_truth_revision()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn refresh_query_change_for_certification(
        &mut self,
        request: worth_ui_query_binding::WorthUiOperationLiveRefreshRequest<'_>,
    ) -> Result<
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome,
        worth_ui_query_binding::WorthUiOperationLiveRefreshError,
    > {
        self.runtime.query_binding.refresh_operation_live(request)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn measurement_basis_sources_for_certification(
        &self,
    ) -> Box<[crate::declaration::UiDeclaredMeasurementBasisSource]> {
        self.app
            .declaration_artifacts()
            .iter()
            .filter_map(|artifact| artifact.graph_handoff().ok())
            .filter_map(|handoff| {
                handoff
                    .measurement_policy()
                    .admitted()
                    .and_then(|policy| policy.basis_source())
            })
            .collect()
    }

    pub(crate) fn host_measurement_collector(
        &self,
    ) -> crate::host::WorthUiHostMeasurementCollector {
        self.runtime.host_measurement_collector()
    }

    pub(crate) fn prepare_graph_successor(
        &self,
        commit: crate::graph::UiGraphMutationCommitResult,
    ) -> Result<
        WorthUiPreparedApplicationGraphSuccessor,
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGraphSuccessorDenial,
    >{
        self.app.prepare_graph_successor(commit)
    }

    pub(crate) fn shutdown(self) -> WorthUiRuntimeShutdownReceipt {
        self.runtime.shutdown()
    }

    #[cfg(test)]
    pub(crate) fn into_runtime_for_test(self) -> WorthUiRuntime {
        self.runtime
    }
}
