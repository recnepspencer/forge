use worth_ui_inspection::{UiInspectionQuery, UiInspectionSupportReport};

use super::WorthUiApp;

impl WorthUiApp {
    pub fn inspection_support_report_for(
        &self,
        query: &UiInspectionQuery,
    ) -> UiInspectionSupportReport {
        crate::facade::inspection_bridge::support_routing::inspection_support_report_for(
            self, query,
        )
    }

    pub(crate) fn try_query_touch_for_node(
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

    pub(crate) fn try_allocation_touch_for_node(
        &self,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        crate::obligations::touch::UiGraphTouchDescriptor,
        crate::obligations::touch::UiGraphTouchDenial,
    > {
        crate::facade::inspection_bridge::obligation_routes::try_allocation_touch_for_node(
            self,
            graph_node_identity,
        )
    }
}
