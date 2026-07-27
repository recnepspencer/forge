use super::WorthUiApplicationSessionState;

impl WorthUiApplicationSessionState {
    pub(crate) fn mounted_allocation_projection_for(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        Option<worth_ui_host_contract::UiMountedAllocationProjection>,
        crate::runtime::UiMountedAllocationProjectionDenial,
    > {
        self.runtime
            .allocation_receipt_ledger
            .mounted_projection_source(None)
            .projection(graph_node)
    }

    pub(crate) fn measurement_policy_for(
        &self,
        declaration: &crate::declaration::UiDeclarationIdentity,
    ) -> Option<crate::declaration::UiDeclaredMeasurementPolicyPosture> {
        self.app
            .declaration_artifacts()
            .iter()
            .find(|artifact| artifact.identity() == declaration)
            .and_then(|artifact| artifact.graph_handoff().ok())
            .and_then(|handoff| handoff.measurement_policy().admitted().cloned())
    }

    pub(crate) fn activate_initial_mounted_allocation_catalog(
        &mut self,
        graph_successor: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGraphSuccessor,
        admitted: crate::graph::UiAdmittedAllocationCatalogBasisSet,
        boundary: crate::runtime::WorthUiFrameBoundary,
    ) -> Result<
        crate::runtime::UiCommittedAllocationReplan,
        crate::runtime::WorthUiInitialMountedAllocationActivationDenial,
    > {
        self.runtime.activate_initial_mounted_allocation_catalog(
            &mut self.app,
            graph_successor,
            admitted,
            boundary,
        )
    }
}
