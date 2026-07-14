use worth_ui_inspection::{UiInspectionQuery, UiInspectionScope, UiInspectionSupportReport};

use crate::admission::UiAdmissionBoundary;
use crate::declaration::{
    UiDeclarationArtifact, UiDeclarationAuthoredEvidenceIndex, UiDeclarationCloseoutReport,
};
use crate::evidence::{UiEvidenceExpansion, UiEvidenceRef};
use crate::facade::{
    inspection::expand_evidence_ref as expand_inspection_evidence_ref,
    inspection_bridge::{
        route_inspection, UiInspectionClosureReport, UiInspectionFacadeObservation,
        UiInspectionReceipt,
    },
    lifecycle::{
        build_graph_evidence_indexes, WorthUiCapabilityRegistrationFreezeCore,
        WorthUiFacadeLifecycleBootstrap,
    },
    registry::CapabilitySnapshot,
    retained_obligation_registry::WorthUiRetainedObligationRegistry,
    runtime_handoff::{WorthUiRuntime, WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial},
};
use crate::graph::{
    UiGraphAspectEvidenceIndexes, UiGraphAuthority, UiGraphCloseoutReport,
    UiGraphNodeEvidenceIndex, UiGraphSnapshot,
};
use crate::lifecycle::WorthUiRuntimeSupportInventory;
use crate::obligations::closeout::UiObligationCloseoutReport;
use crate::obligations::touch::UiGraphTouchDescriptor;
use crate::runtime::WorthUiRetainedAllocationPlanningEvidenceRegistry;
use std::rc::Rc;

/// Runtime facade entrypoint for building Worth UI applications.
pub struct WorthUi {
    _sealed: (),
}

impl WorthUi {
    /// Start a Worth UI application definition.
    pub fn app() -> crate::facade::entry::WorthUiBuilder {
        crate::facade::entry::WorthUiBuilder::new()
    }
}

/// Worth UI application after capability registration has frozen.
pub struct WorthUiApp {
    capability_snapshot: CapabilitySnapshot,
    declaration_artifacts: Vec<UiDeclarationArtifact>,
    graph_snapshot: UiGraphSnapshot,
    lifecycle: WorthUiFacadeLifecycleBootstrap,
    authored_evidence_index: UiDeclarationAuthoredEvidenceIndex,
    graph_node_evidence_index: UiGraphNodeEvidenceIndex,
    graph_aspect_evidence_indexes: UiGraphAspectEvidenceIndexes,
    retained_obligations: WorthUiRetainedObligationRegistry,
    retained_allocation_planning_evidence: Rc<WorthUiRetainedAllocationPlanningEvidenceRegistry>,
}

impl WorthUiApp {
    pub(crate) fn from_freeze_core(core: WorthUiCapabilityRegistrationFreezeCore) -> Self {
        let (capability_snapshot, declaration_artifacts, graph_snapshot, lifecycle) =
            core.into_parts();
        Self::from_authority_parts(
            capability_snapshot,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
        )
    }

    pub(crate) fn from_authority_parts(
        capability_snapshot: CapabilitySnapshot,
        declaration_artifacts: Vec<UiDeclarationArtifact>,
        graph_snapshot: UiGraphSnapshot,
        lifecycle: WorthUiFacadeLifecycleBootstrap,
    ) -> Self {
        let authored_evidence_index =
            UiDeclarationAuthoredEvidenceIndex::rebuild(&declaration_artifacts, &graph_snapshot);
        let graph_evidence =
            build_graph_evidence_indexes(&declaration_artifacts, &graph_snapshot, &lifecycle);

        Self {
            capability_snapshot,
            declaration_artifacts,
            authored_evidence_index,
            graph_node_evidence_index: graph_evidence.node,
            graph_aspect_evidence_indexes: graph_evidence.aspect,
            graph_snapshot,
            lifecycle,
            retained_obligations: WorthUiRetainedObligationRegistry::default(),
            retained_allocation_planning_evidence: Rc::default(),
        }
    }

    #[cfg(test)]
    pub(crate) fn into_authority_parts(
        self,
    ) -> (
        CapabilitySnapshot,
        Vec<UiDeclarationArtifact>,
        UiGraphSnapshot,
        WorthUiFacadeLifecycleBootstrap,
    ) {
        (
            self.capability_snapshot,
            self.declaration_artifacts,
            self.graph_snapshot,
            self.lifecycle,
        )
    }

    /// Inspect the immutable capability snapshot owned by this app.
    pub fn capabilities(&self) -> &CapabilitySnapshot {
        &self.capability_snapshot
    }

    /// Inspect the canonical declaration artifacts admitted during app freeze.
    pub fn declaration_artifacts(&self) -> &[UiDeclarationArtifact] {
        &self.declaration_artifacts
    }

    /// Inspect the proof-bearing graph authority surface owned by this app.
    pub fn graph(&self) -> UiGraphAuthority<'_> {
        UiGraphAuthority::new(&self.graph_snapshot)
    }

    /// Enter the runtime-owned admission boundary through one formal facade lane.
    pub fn admission(&self) -> UiAdmissionBoundary<'_> {
        UiAdmissionBoundary::new(&self.declaration_artifacts, &self.graph_snapshot)
    }

    /// Admit Query-backed measurement eligibility from an ordinary projection-fact
    /// consumption attempt without requiring callers to mint prerequisite artifacts.
    pub fn admit_query_measurement_eligibility_for_touch_from_query_authority(
        &self,
        touch: &UiGraphTouchDescriptor,
        authority: worth_ui_query_binding::WorthUiQueryAuthorityHandle,
    ) -> Option<crate::admission::UiQueryMeasurementEligibility> {
        self.admission()
            .admit_query_measurement_eligibility_for_touch_from_query_authority(touch, authority)
    }

    pub(crate) fn graph_snapshot(&self) -> &UiGraphSnapshot {
        &self.graph_snapshot
    }

    pub(crate) fn lifecycle(&self) -> &WorthUiFacadeLifecycleBootstrap {
        &self.lifecycle
    }

    pub(crate) fn authored_evidence_index(&self) -> &UiDeclarationAuthoredEvidenceIndex {
        &self.authored_evidence_index
    }

    pub(crate) fn graph_node_evidence_index(&self) -> &UiGraphNodeEvidenceIndex {
        &self.graph_node_evidence_index
    }

    pub(crate) fn graph_aspect_evidence_indexes(&self) -> &UiGraphAspectEvidenceIndexes {
        &self.graph_aspect_evidence_indexes
    }

    pub(crate) fn measurement_inspection_evidence(
        &self,
    ) -> &crate::facade::measurement_inspection_evidence::UiMeasurementInspectionEvidenceSnapshot
    {
        self.lifecycle.measurement_inspection_evidence()
    }

    pub fn graph_closeout_report(&self) -> UiGraphCloseoutReport {
        UiGraphCloseoutReport::milestone33()
    }

    /// Inspect milestone-closeout metadata owned by the declaration boundary.
    pub fn declaration_closeout_report(&self) -> UiDeclarationCloseoutReport {
        UiDeclarationCloseoutReport::milestone32()
    }

    pub fn obligation_closeout_report(&self) -> UiObligationCloseoutReport {
        UiObligationCloseoutReport::milestone34()
    }

    /// Enter the runtime-owned inspection surface through one formal facade lane.
    pub fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        route_inspection(self, query)
    }

    pub fn expand_evidence_ref(
        &self,
        evidence_ref: UiEvidenceRef,
        requested_richness: worth_ui_inspection::UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        expand_inspection_evidence_ref(self, evidence_ref, requested_richness)
    }

    pub fn discard_evidence_slice(&self, slice_ref: crate::evidence::UiEvidenceSliceRef) -> bool {
        self.retained_obligations.discard_slice(slice_ref)
            || self
                .retained_allocation_planning_registry()
                .discard_slice(slice_ref)
    }

    pub fn inspection_support_report(&self, scope: UiInspectionScope) -> UiInspectionSupportReport {
        self.lifecycle.inspection_support_report(scope)
    }

    pub fn inspection_closure_report(&self) -> UiInspectionClosureReport {
        self.lifecycle.inspection_closure_report()
    }

    pub fn runtime_support_inventory(&self) -> &WorthUiRuntimeSupportInventory {
        self.lifecycle.runtime_support_inventory()
    }

    pub fn inspection_observation(&self) -> UiInspectionFacadeObservation {
        self.lifecycle.inspection_observation()
    }

    #[cfg(test)]
    pub(crate) fn rebuild_authored_evidence_index_from_authority(&mut self) {
        self.authored_evidence_index =
            crate::declaration::UiDeclarationAuthoredEvidenceIndex::rebuild(
                &self.declaration_artifacts,
                &self.graph_snapshot,
            );
    }

    #[cfg(test)]
    pub(crate) fn rebuild_graph_evidence_indexes_from_authority(&mut self) {
        let graph_evidence = build_graph_evidence_indexes(
            &self.declaration_artifacts,
            &self.graph_snapshot,
            &self.lifecycle,
        );
        self.graph_node_evidence_index = graph_evidence.node;
        self.graph_aspect_evidence_indexes = graph_evidence.aspect;
    }

    /// Launch the runtime whose ordinary frame boundary owns dispatcher close/pump.
    pub fn launch_runtime(
        &self,
        launch: WorthUiRuntimeLaunch,
    ) -> Result<WorthUiRuntime, WorthUiRuntimeLaunchDenial> {
        WorthUiRuntime::launch(
            launch,
            self.capability_snapshot.digest(),
            Rc::clone(&self.retained_allocation_planning_evidence),
        )
    }

    pub(crate) fn retained_obligation_registry(&self) -> &WorthUiRetainedObligationRegistry {
        &self.retained_obligations
    }

    pub(crate) fn retained_allocation_planning_registry(
        &self,
    ) -> &WorthUiRetainedAllocationPlanningEvidenceRegistry {
        self.retained_allocation_planning_evidence.as_ref()
    }

    pub fn inspection_support_report_for(
        &self,
        query: &UiInspectionQuery,
    ) -> UiInspectionSupportReport {
        crate::facade::inspection_bridge::support_routing::inspection_support_report_for(
            self, query,
        )
    }

    pub fn try_query_touch_for_node(
        &self,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        crate::obligations::touch::UiGraphTouchDescriptor,
        crate::obligations::touch::UiGraphTouchDenial,
    > {
        crate::facade::inspection_bridge::obligation_routes::try_query_touch_for_node(
            self,
            graph_node_identity,
        )
    }
}
