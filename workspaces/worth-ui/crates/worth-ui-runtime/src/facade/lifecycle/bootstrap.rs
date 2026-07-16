use worth_ui_inspection::UiInspectionScopeInventory;

use crate::declaration::{
    derive_declaration_inspection_support_projection, UiDeclarationArtifact,
    UiDeclarationInspectionSupportProjection,
};
use crate::facade::host_observation::WorthUiHostContract;
use crate::facade::inspection_observation::WorthUiInspectionObservationState;
use crate::facade::measurement_inspection_evidence::UiMeasurementInspectionEvidenceSnapshot;
use crate::facade::{
    inspection_bridge::UiMeasurementInspectionEvidenceBundle, registry::CapabilitySnapshot,
    WorthUiRuntimeSupportInventory, RUNTIME_SUPPORT_INVENTORY,
};
use worth_ui_dsl::WorthUiDslPackage;
use worth_ui_inspection::{UiInspectionScope, UiInspectionSupportReport};

pub(crate) struct WorthUiFacadeLifecycleBootstrap {
    inspection_scope_inventory: UiInspectionScopeInventory,
    declaration_inspection_support: UiDeclarationInspectionSupportProjection,
    measurement_inspection_evidence: UiMeasurementInspectionEvidenceSnapshot,
    inspection_observation: WorthUiInspectionObservationState,
    runtime_support_inventory: WorthUiRuntimeSupportInventory,
    _dsl_package: WorthUiDslPackage,
    _host_contract: WorthUiHostContract,
}

impl WorthUiFacadeLifecycleBootstrap {
    pub(crate) fn bootstrap_with_inspection_scope_inventory(
        dsl_package: WorthUiDslPackage,
        host_contract: WorthUiHostContract,
        declaration_artifacts: &[UiDeclarationArtifact],
        measurement_inspection_evidence: Box<[UiMeasurementInspectionEvidenceBundle]>,
        inspection_scope_inventory: UiInspectionScopeInventory,
    ) -> Self {
        Self {
            inspection_scope_inventory,
            declaration_inspection_support: derive_declaration_inspection_support_projection(
                declaration_artifacts,
            ),
            measurement_inspection_evidence: UiMeasurementInspectionEvidenceSnapshot::from_bundles(
                measurement_inspection_evidence,
            ),
            inspection_observation: WorthUiInspectionObservationState::new(),
            runtime_support_inventory: RUNTIME_SUPPORT_INVENTORY,
            _dsl_package: dsl_package,
            _host_contract: host_contract,
        }
    }

    pub(crate) fn inspection_support_report(
        &self,
        scope: UiInspectionScope,
    ) -> UiInspectionSupportReport {
        self.inspection_observation.record_support_report();
        if let Some(report) = self.declaration_inspection_support.support_report(scope) {
            return report;
        }
        self.inspection_scope_inventory.support_report(scope)
    }

    pub(crate) fn inspection_closure_report(
        &self,
    ) -> crate::facade::inspection_bridge::UiInspectionClosureReport {
        self.inspection_scope_inventory.closure_report()
    }

    pub(crate) fn runtime_support_inventory(&self) -> &WorthUiRuntimeSupportInventory {
        &self.runtime_support_inventory
    }

    pub(crate) fn inspection_observation(
        &self,
    ) -> crate::facade::inspection_bridge::UiInspectionFacadeObservation {
        self.inspection_observation.snapshot()
    }

    pub(crate) fn measurement_inspection_evidence(
        &self,
    ) -> &UiMeasurementInspectionEvidenceSnapshot {
        &self.measurement_inspection_evidence
    }

    pub(crate) fn record_inspection_query(&self) {
        self.inspection_observation.record_query();
    }

    pub(crate) fn record_unsupported_inspection_query(&self) {
        self.inspection_observation.record_unsupported_query();
    }

    pub(crate) fn record_rich_artifact_materialization(&self) {
        self.inspection_observation
            .record_rich_artifact_materialization();
    }

    pub(crate) fn record_authored_lookup(&self) {
        self.inspection_observation.record_authored_lookup();
    }

    pub(crate) fn record_graph_node_evidence_index_rebuild(&self) {
        self.inspection_observation
            .record_graph_node_evidence_index_rebuild();
    }

    pub(crate) fn record_graph_aspect_evidence_index_rebuild(&self) {
        self.inspection_observation
            .record_graph_aspect_evidence_index_rebuild();
    }
}

pub(crate) struct WorthUiCapabilityRegistrationFreezeCore {
    capability_snapshot: CapabilitySnapshot,
    declaration_artifacts: Vec<UiDeclarationArtifact>,
    graph_snapshot: crate::graph::UiGraphSnapshot,
    lifecycle: WorthUiFacadeLifecycleBootstrap,
}

impl WorthUiCapabilityRegistrationFreezeCore {
    pub(crate) fn assemble(
        capability_snapshot: CapabilitySnapshot,
        declaration_artifacts: Vec<UiDeclarationArtifact>,
        graph_snapshot: crate::graph::UiGraphSnapshot,
        lifecycle: WorthUiFacadeLifecycleBootstrap,
    ) -> Self {
        Self {
            capability_snapshot,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CapabilitySnapshot,
        Vec<UiDeclarationArtifact>,
        crate::graph::UiGraphSnapshot,
        WorthUiFacadeLifecycleBootstrap,
    ) {
        (
            self.capability_snapshot,
            self.declaration_artifacts,
            self.graph_snapshot,
            self.lifecycle,
        )
    }
}
